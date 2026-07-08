mod config;
use dotenvy::dotenv;
use crate::config::Config;
fn main() {
    dotenv().ok();

    let config = Config::from_env();

    if config.debug {
        println!("Database URL: {}", config.database_url);
        println!("API Key: {}", config.api_key);
        println!("Port: {}", config.port);
        println!("Debug: {}", config.debug);
    } else {
        println!("Debug mode is off. Not printing sensitive information.");
    }
}
