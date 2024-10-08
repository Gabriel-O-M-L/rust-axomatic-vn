use sqlx::postgres::PgPoolOptions;

pub async fn build_db_connection() -> Result<sqlx::PgPool, sqlx::Error> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    Ok(pool)
}