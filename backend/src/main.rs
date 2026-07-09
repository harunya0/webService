mod config;
mod handlers;
mod routes;
mod utils;
mod models;

use axum::serve::Listener;
use sqlx::database;
use sqlx::postgres::PgPoolOptions;
use crate::config::Config;
use crate::routes::route::auth_routes;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = Config::from_env();
    let database_url = config.database_url.clone();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database connection pool");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    if config.debug {
        println!("JWT Secret: {}", config.jwt_secret);
        println!("Port: {}", config.port);
        println!("Debug: {}", config.debug);
    } else {
        println!("Debug mode is off. Not printing sensitive information.");
    }

    let app = auth_routes().with_state(pool);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .expect("Failed to bind to address");

    println!("Server running on port {}", config.port);
    axum::serve(listener, app).await.unwrap();
}
