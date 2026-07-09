use axum::{extract::State, Json};
use axum::http::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;
use crate::utils::to_hash::{hash_password, verify_password};
use crate::utils::sql_request::create_user;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterRequest>,
) -> Result<StatusCode, StatusCode> {
    let password_hash = hash_password(&payload.password)
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_id = create_user(&pool, &payload.username, &password_hash)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    println!("User created with ID: {}", user_id);
    
    Ok(StatusCode::CREATED)
}