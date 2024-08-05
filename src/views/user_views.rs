use axum::{
    Json,
    response::{Html, IntoResponse},
    http::{StatusCode, Uri, header::{self, HeaderMap, HeaderName}},
};
use sqlx::PgPool;

pub fn sign_up(database: PgPool) -> impl IntoResponse {
    Html("<h1>Sign up page</h1>".to_string())
}
pub fn sign_in(database: PgPool) -> impl IntoResponse {
    Html("<h1>Sign in page</h1>".to_string())
}
