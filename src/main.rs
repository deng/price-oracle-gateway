mod cache;
mod config;
mod error;
mod handlers;
mod models;
mod services;

use crate::config::AppConfig;
use crate::handlers::{health_check, get_price};
use crate::services::{ApiNinjasClient, BinanceClient, BitgetClient, DexScreenerClient, ExchangeClient};
use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "gateway=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 加载配置
    let config = AppConfig::load()?;
    tracing::info!("Configuration loaded successfully");

    // 初始化交易所客户端
    let timeout = Duration::from_secs(config.exchanges.timeout_secs);
    let mut clients: Vec<Box<dyn ExchangeClient>> = Vec::new();

    if config.exchanges.binance.enabled {
        let binance = BinanceClient::new(
            config.exchanges.binance.base_url.clone(),
            timeout,
        );
        clients.push(Box::new(binance));
        tracing::info!("Binance client initialized");
    }

    if config.exchanges.bitget.enabled {
        let bitget = BitgetClient::new(
            config.exchanges.bitget.base_url.clone(),
            timeout,
        );
        clients.push(Box::new(bitget));
        tracing::info!("Bitget client initialized");
    }

    if config.exchanges.apininjas.enabled {
        let apininjas = ApiNinjasClient::new(
            config.exchanges.apininjas.base_url.clone(),
            config.exchanges.apininjas.api_key.clone(),
            timeout,
        );
        clients.push(Box::new(apininjas));
        tracing::info!("API Ninjas client initialized");
    }

    if config.exchanges.dexscreener.enabled {
        let dexscreener = DexScreenerClient::new(
            config.exchanges.dexscreener.base_url.clone(),
            timeout,
        );
        clients.push(Box::new(dexscreener));
        tracing::info!("DexScreener client initialized");
    }

    let shared_clients = Arc::new(clients);

    // 初始化价格缓存（10 秒 TTL，最多 1000 个条目）
    let price_cache = crate::cache::PriceCache::new(10, 1000);
    tracing::info!("Price cache initialized with 10s TTL");

    // 创建应用状态
    let app_state = Arc::new(crate::handlers::price::AppState {
        clients: shared_clients,
        cache: price_cache,
    });

    // 配置 CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 构建路由
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/price", get(get_price))
        .with_state(app_state)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // 启动服务器
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    tracing::info!("Gateway server listening on {}", addr);
    tracing::info!("Health check: http://{}/health", addr);
    tracing::info!("Price API: http://{}/api/v1/price?symbol=BTCUSDT", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
