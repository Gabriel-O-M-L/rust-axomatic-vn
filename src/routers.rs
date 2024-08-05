use axum::{Router, routing};
use crate::views::user_views;
use sqlx::PgPool;
pub fn router_creator(database : PgPool) -> Router {
    Router::new()
        .route("/home", routing::get(|| async { "Hello, world!" }))
        .route("/SignUp", routing::get(|| async { user_views::sign_up(database) }))
}