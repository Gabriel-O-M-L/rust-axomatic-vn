use axum::{Json, Router, routing};
use sqlx::PgPool;
use std::sync::Arc;

use crate::views::user_views;
use crate::views::user_views::User;

pub fn router_creator(database: PgPool) -> Router {
    let database = Arc::new(database);

    Router::new()
        .route("/home", routing::post(|| async { "Hello, world!" }))
        .route("/SignUp", {
            let db = Arc::clone(&database);
            routing::post(move |payload: Json<User>| {
                user_views::sign_up(db.clone(), payload)
            })
        })
        .route("/Login", {
            let db = Arc::clone(&database);
            routing::post(move |payload: Json<User>| {
                user_views::sign_in(db.clone(), payload)
            })
        })
}