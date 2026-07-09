use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
    pub debug: bool,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set"),
            port: env::var("PORT")
                .expect("PORT must be set")
                .parse()
                .expect("PORT must be a number"),
            debug: env::var("DEBUG")
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(false),
        }
    }
}