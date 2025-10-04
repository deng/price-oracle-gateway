use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub exchanges: ExchangesConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExchangesConfig {
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
    pub binance: ExchangeConfig,
    pub bitget: ExchangeConfig,
    pub apininjas: ApiNinjasConfig,
    pub dexscreener: ExchangeConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExchangeConfig {
    pub enabled: bool,
    pub base_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiNinjasConfig {
    pub enabled: bool,
    pub base_url: String,
    pub api_key: String,
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    3000
}

fn default_timeout() -> u64 {
    10
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        // 加载 .env 文件
        let _ = dotenvy::dotenv();

        let config = config::Config::builder()
            .add_source(config::File::with_name("config/default").required(false))
            .add_source(config::Environment::with_prefix("APP").separator("__"))
            .build()?;

        let app_config: AppConfig = config.try_deserialize()?;
        Ok(app_config)
    }
}

pub type SharedConfig = Arc<AppConfig>;
