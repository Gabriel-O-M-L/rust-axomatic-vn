

mod database;
mod routers;

use sqlx::PgPool;
use axum::{Router, routing, serve};
use tokio::net::TcpListener;
use dotenv::dotenv;
fn router_creator() -> Router {
    Router::new()
        .route("/", routing::get(|| async { "Hello, world!" }))
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
    let database = db_pool().await;

    let app: Router = router_creator();

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    match serve(listener, app.into_make_service()).await{
        Ok(_) => eprintln!("Server started on port 8080"),
        Err(e) => println!("Error starting server: {}", e),
    }
}
