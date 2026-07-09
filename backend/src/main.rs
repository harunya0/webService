mod config;
use sqlx::postgres::PgPoolOptions;
use crate::config::Config;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = Config::from_env();

    let database_url = config.database_url;

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
}
