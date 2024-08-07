use axum::{
    extract::Json,
    response::{Html, IntoResponse},
    http::{StatusCode, Uri, header::{self, HeaderMap, HeaderName}},
};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use bcrypt::{hash,DEFAULT_COST};

use crate::tools::create_jwt;

#[derive(Deserialize, Clone)]
pub struct User {
    username: String,
    email: String,
    password: String,
}

pub async fn sign_up(database: Arc<PgPool>, Json(payload): Json<User>) -> impl IntoResponse {
    match hash(&payload.password, DEFAULT_COST) {
        Ok(password_hash) => {
            match sqlx::query("INSERT INTO \"user\" (username, email, password_hash) VALUES ($1, $2, $3)")
                .bind(&payload.username)
                .bind(&payload.email)
                .bind(&password_hash)
                .execute(&*database)
                .await
            {
                Ok(_) => StatusCode::CREATED.into_response(),
                Err(e) => {
                    eprintln!("Failed to insert user: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to hash password: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
pub async fn sign_in(database: Arc<PgPool>, Json(payload) : Json<User>) -> impl IntoResponse {
    match hash(&payload.password, DEFAULT_COST) {
        Ok(password_hash) => {
            match sqlx::query("SELECT email, password_hash FROM \"user\" WHERE password_hash = $2 AND email = $1")
                .bind(&payload.email)
                .bind(&password_hash)
                .fetch_one(&*database)
                .await
            {
                Ok(_) => {
                    match create_jwt(&payload.email) {
                        Ok(token) => {
                            let mut headers = HeaderMap::new();
                            headers.insert(header::AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());
                            (StatusCode::OK, headers).into_response()
                        },
                        Err(e) => {
                            eprintln!("Failed to create JWT: {}", e);
                            StatusCode::INTERNAL_SERVER_ERROR.into_response()
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Failed to find user: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to hash password: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
