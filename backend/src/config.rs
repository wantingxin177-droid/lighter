use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub lighter_ws_url: String,
    pub lighter_rest_url: String,
    pub sync_interval_ms: u64,
    pub max_connections: u32,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Self {
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()?,
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/kin".to_string()),
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),
            lighter_ws_url: std::env::var("LIGHTER_WS_URL")
                .unwrap_or_else(|_| "wss://ws.lighter.xyz".to_string()),
            lighter_rest_url: std::env::var("LIGHTER_REST_URL")
                .unwrap_or_else(|_| "https://api.lighter.xyz".to_string()),
            sync_interval_ms: std::env::var("SYNC_INTERVAL_MS")
                .unwrap_or_else(|_| "1000".to_string())
                .parse()?,
            max_connections: std::env::var("MAX_CONNECTIONS")
                .unwrap_or_else(|_| "100".to_string())
                .parse()?,
        })
    }
}
