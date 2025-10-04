use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// 价格信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    pub symbol: String,
    pub price: f64,
    pub exchange: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub timestamp: DateTime<Utc>,
    /// 是否来自缓存
    #[serde(default)]
    pub cached: bool,
}

/// 价格查询请求
#[derive(Debug, Deserialize)]
pub struct PriceRequest {
    /// 交易对符号，如 BTCUSDT（可选，如果提供了 base 和 quote 则忽略此字段）
    #[serde(default)]
    pub symbol: Option<String>,
    /// 基础货币，如 BTC
    #[serde(default)]
    pub base: Option<String>,
    /// 报价货币，如 USDT
    #[serde(default)]
    pub quote: Option<String>,
    /// 交易所名称
    #[serde(default)]
    pub exchange: Option<String>,
}

impl PriceRequest {
    /// 获取交易对符号
    /// 优先使用 base + quote，如果不存在则使用 symbol
    pub fn get_symbol(&self) -> Result<String, String> {
        // 如果提供了 base 和 quote，则组合成 symbol
        if let (Some(base), Some(quote)) = (&self.base, &self.quote) {
            Ok(format!("{}{}", base.to_uppercase(), quote.to_uppercase()))
        } else if let Some(symbol) = &self.symbol {
            Ok(symbol.to_uppercase())
        } else {
            Err("Either 'symbol' or both 'base' and 'quote' must be provided".to_string())
        }
    }
}

/// 价格查询响应
#[derive(Debug, Serialize)]
pub struct PriceResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Price>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 多个交易所价格响应
#[derive(Debug, Serialize)]
pub struct MultiPriceResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<Price>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 健康检查响应
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub version: String,
}
