use axum::{extract::State, Json};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::utils::to_hash::{hash_password, verify_password};
use crate::utils::sql_request::{create_user, find_user_by_username};
use crate::utils::jwt::{create_jwt, verify_jwt};
use crate::config::Config;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<StatusCode, StatusCode> {
    let password_hash = hash_password(&payload.password)
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_id = create_user(&state.pool, &payload.username, &password_hash)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    println!("User created with ID: {}", user_id);
    
    Ok(StatusCode::CREATED)
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let user = find_user_by_username(&state.pool, &payload.username)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let is_valid = verify_password(&payload.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !is_valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = create_jwt(user.id, &state.jwt_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LoginResponse { token }))
}