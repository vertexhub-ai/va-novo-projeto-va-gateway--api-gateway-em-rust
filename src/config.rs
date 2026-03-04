use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server_port: u16,
    pub database_url: String,
    pub upstream_service_url: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, dotenvy::Error> {
        dotenvy::dotenv().ok(); // Load .env file if it exists

        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .expect("SERVER_PORT must be a valid port number");

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        let upstream_service_url = env::var("UPSTREAM_SERVICE_URL")
            .expect("UPSTREAM_SERVICE_URL must be set");

        Ok(AppConfig {
            server_port,
            database_url,
            upstream_service_url,
        })
    }
}
