use axum::{Router, routing::post};
use crate::state::AppState;
use crate::handlers::handler::{register, login};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}