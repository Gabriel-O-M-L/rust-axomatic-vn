mod database;
mod routers;
mod views;
mod tools;

use sqlx::PgPool;
use axum::{Router, serve};
use tokio::net::TcpListener;
use dotenv::dotenv;

async fn router_creator() -> Router {
    routers::router_creator(db_pool().await)
}
async fn db_pool() -> PgPool {
    match database::build_db_connection().await
    {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Error connecting to database: {}", e);
            std::process::exit(1);
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app: Router = router_creator().await;
    let running_port = std::env::var("PORT").unwrap_or("8080".to_string());
    let address = format!("{}{}", "127.0.0.1:", running_port);

    let listener = TcpListener::bind(address).await.unwrap();

    match serve(listener, app.into_make_service()).await{
        Ok(_) => eprintln!("Server started on port 8080"),
        Err(e) => println!("Error starting server: {}", e),
    }
}
