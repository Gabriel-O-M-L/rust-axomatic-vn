use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::{Request, StatusCode},
    response::IntoResponse,
    middleware::Next,
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn auth<B>(req: Request<B>, next: Next) -> impl IntoResponse {
    let headers = req.headers();
    if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                let decoding_key = DecodingKey::from_secret("your_secret_key".as_ref());
                let validation = Validation::new(Algorithm::HS256);
                match decode::<Claims>(token, &decoding_key, &validation) {
                    Ok(_) => {
                        next.run(req).await;
                    }
                    Err(_) => {
                        StatusCode::UNAUTHORIZED.into_response();
                    }
                }
            }
        }
    }
    StatusCode::UNAUTHORIZED.into_response()
}




pub fn create_jwt(email: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(1))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: email.to_owned(),
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref()))
}