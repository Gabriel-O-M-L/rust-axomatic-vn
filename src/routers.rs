use axum::{Router, routing};
use crate::li
pub fn router_creator() -> Router {
    Router::new()
        .route("/home", routing::get(|| async { "Hello, world!" }))
        .route("/SignUp", routing::get(|| async { "Sign up page" }))
}