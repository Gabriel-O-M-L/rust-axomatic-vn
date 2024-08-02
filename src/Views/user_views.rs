use axum::{
    Json,
    response::{Html, IntoResponse},
    http::{StatusCode, Uri, header::{self, HeaderMap, HeaderName}},
};

pub fn sign_up() -> impl IntoResponse {
    Html("<h1>Sign up page</h1>".to_string())
}
pub fn sign_in() -> impl IntoResponse {
    Html("<h1>Sign in page</h1>".to_string())
}
