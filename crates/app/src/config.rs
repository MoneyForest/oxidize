use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub otlp_endpoint: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost/oxidize".to_string()),
            otlp_endpoint: env::var("OTLP_ENDPOINT").ok(),
        }
    }
}
