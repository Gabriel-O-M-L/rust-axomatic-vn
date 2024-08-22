use axum::{
    extract::Json,
    response::{Html, IntoResponse},
    http::{StatusCode, Uri, header::{self, HeaderMap, HeaderName}},
};
use sqlx::{PgPool, Row};
use serde::Deserialize;
use std::sync::Arc;
use bcrypt::{hash,DEFAULT_COST,verify};

use crate::tools::create_jwt;

#[derive(Deserialize, Clone)]
pub struct UserCreate {
    username: String,
    email: String,
    password: String,
}
#[derive(Deserialize, Clone)]
pub struct UserLogin{
    email: String,
    password: String,
}

pub async fn sign_up(database: Arc<PgPool>, Json(payload): Json<UserCreate>) -> impl IntoResponse {
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
pub async fn sign_in(database: Arc<PgPool>, Json(payload) : Json<UserLogin>) -> impl IntoResponse {
    match sqlx::query("SELECT email, password_hash FROM \"user\" WHERE email = $1")
        .bind(&payload.email)
        .fetch_one(&*database)
        .await
    {
        Ok(row) => {
            let stored_password_hash: String = row.get("password_hash");
            match verify(&payload.password, &stored_password_hash) {
                Ok(true) => {
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
                Ok(false) => {
                    eprintln!("Password does not match");
                    StatusCode::UNAUTHORIZED.into_response()
                },
                Err(e) => {
                    eprintln!("Failed to verify password: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        },
        Err(sqlx::Error::RowNotFound) => {
            eprintln!("Failed to find user: no rows returned by a query that expected to return at least one row");
            StatusCode::UNAUTHORIZED.into_response()
        },
        Err(e) => {
            eprintln!("Failed to find user: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
