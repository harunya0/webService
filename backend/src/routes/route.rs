use axum::{Router, routing::post};
use sqlx::PgPool;
use crate::handlers::handler::register;

pub fn auth_routes() -> Router<PgPool> {
    Router::new()
        .route("/register", post(register))
}