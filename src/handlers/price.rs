use crate::cache::PriceCache;
use crate::models::{PriceRequest, PriceResponse, MultiPriceResponse};
use crate::services::ExchangeClient;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use std::sync::Arc;

pub type ExchangeClients = Arc<Vec<Box<dyn ExchangeClient>>>;

#[derive(Clone)]
pub struct AppState {
    pub clients: ExchangeClients,
    pub cache: PriceCache,
}

/// GET /api/v1/price?symbol=BTCUSDT&exchange=binance
/// 或 GET /api/v1/price?base=BTC&quote=USDT&exchange=binance
pub async fn get_price(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PriceRequest>,
) -> Response {
    // 获取交易对符号
    let symbol = match params.get_symbol() {
        Ok(s) => s,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(PriceResponse {
                    success: false,
                    data: None,
                    error: Some(e),
                }),
            ).into_response();
        }
    };

    // 如果指定了交易所
    if let Some(exchange_name) = params.exchange {
        // 先查缓存
        if let Some(cached_price) = state.cache.get(&exchange_name, &symbol).await {
            return (
                StatusCode::OK,
                Json(PriceResponse {
                    success: true,
                    data: Some(cached_price),
                    error: None,
                }),
            ).into_response();
        }

        // 缓存未命中，查询 API
        let client = state.clients
            .iter()
            .find(|c| c.name() == exchange_name.to_lowercase());

        match client {
            Some(client) => match client.get_price(&symbol).await {
                Ok(price) => {
                    // 将结果存入缓存
                    state.cache.set(price.clone()).await;
                    
                    (
                        StatusCode::OK,
                        Json(PriceResponse {
                            success: true,
                            data: Some(price),
                            error: None,
                        }),
                    ).into_response()
                },
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(PriceResponse {
                        success: false,
                        data: None,
                        error: Some(e.to_string()),
                    }),
                ).into_response(),
            },
            None => (
                StatusCode::BAD_REQUEST,
                Json(PriceResponse {
                    success: false,
                    data: None,
                    error: Some(format!("Exchange '{}' not found", exchange_name)),
                }),
            ).into_response(),
        }
    } else {
        // 从所有交易所获取价格
        let mut prices = Vec::new();
        let mut errors = Vec::new();

        for client in state.clients.iter() {
            let exchange_name = client.name();
            
            // DexScreener 特殊处理：先检查缓存（30秒），如果有就直接返回
            if exchange_name == "dexscreener" {
                if let Some(cached_prices) = state.cache.get_dex_prices(&symbol).await {
                    // 找到缓存，直接使用
                    prices.extend(cached_prices);
                    continue;
                }
            } else {
                // 其他交易所（Binance, Bitget）：先检查缓存（10秒）
                if let Some(cached_price) = state.cache.get(exchange_name, &symbol).await {
                    prices.push(cached_price);
                    continue;  // 缓存命中，跳过 API 调用
                }
            }
            
            // 缓存未命中，调用 API
            match client.get_all_prices(&symbol).await {
                Ok(price_list) => {
                    if exchange_name == "dexscreener" {
                        // DexScreener：将所有 DEX 价格一起缓存 30 秒
                        state.cache.set_dex_prices(&symbol, price_list.clone()).await;
                        prices.extend(price_list);
                    } else {
                        // 其他交易所：单独缓存每个价格（10秒）
                        for price in price_list {
                            state.cache.set(price.clone()).await;
                            prices.push(price);
                        }
                    }
                },
                Err(e) => {
                    // API 失败时尝试从缓存获取（作为降级）
                    if exchange_name == "dexscreener" {
                        if let Some(cached_prices) = state.cache.get_dex_prices(&symbol).await {
                            prices.extend(cached_prices);
                        } else {
                            errors.push(format!("{}: {}", exchange_name, e));
                        }
                    } else {
                        if let Some(cached_price) = state.cache.get(exchange_name, &symbol).await {
                            prices.push(cached_price);
                        } else {
                            errors.push(format!("{}: {}", exchange_name, e));
                        }
                    }
                },
            }
        }

        if prices.is_empty() {
            // 所有数据源都失败了，返回错误
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(MultiPriceResponse {
                    success: false,
                    data: vec![],
                    error: Some(errors.join("; ")),
                }),
            ).into_response()
        } else {
            // 有成功的结果，不返回部分失败的错误信息
            // 前端不需要知道某些数据源失败了
            (
                StatusCode::OK,
                Json(MultiPriceResponse {
                    success: true,
                    data: prices,
                    error: None,
                }),
            ).into_response()
        }
    }
}
