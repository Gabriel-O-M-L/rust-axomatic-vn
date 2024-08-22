use axum::{Json, Router, routing};
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use crate::views::user_views;
use crate::views::user_views::{UserCreate, UserLogin};

pub fn router_creator(database: PgPool) -> Router {
    let database = Arc::new(database);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/home", routing::post(|| async { "Hello, world!" }))
        .route("/SignUp", {
            let db = Arc::clone(&database);
            routing::post(move |payload: Json<UserCreate>| {
                user_views::sign_up(db.clone(), payload)
            })
        })
        .route("/Login", {
            let db = Arc::clone(&database);
            routing::post(move |payload: Json<UserLogin>| {
                user_views::sign_in(db.clone(), payload)
            })
        })
        .layer(cors)
}