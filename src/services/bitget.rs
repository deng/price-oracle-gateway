use crate::models::Price;
use crate::services::ExchangeClient;
use async_trait::async_trait;
use chrono::Utc;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct BitgetResponse {
    code: String,
    msg: String,
    #[serde(rename = "requestTime")]
    request_time: i64,
    data: Vec<BitgetTicker>,
}

#[derive(Debug, Deserialize)]
struct BitgetTicker {
    symbol: String,
    #[serde(rename = "lastPr")]
    last_pr: String,
    #[serde(rename = "high24h")]
    high_24h: String,
    #[serde(rename = "low24h")]
    low_24h: String,
    #[serde(rename = "baseVolume")]
    base_volume: String,
    #[serde(rename = "quoteVolume")]
    quote_volume: String,
    #[serde(rename = "openUtc")]
    open_utc: String,
    #[serde(rename = "changeUtc24h")]
    change_utc_24h: String,
    #[serde(rename = "change24h")]
    change_24h: String,
}

pub struct BitgetClient {
    client: reqwest::Client,
    base_url: String,
}

impl BitgetClient {
    pub fn new(base_url: String, timeout: Duration) -> Self {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .expect("Failed to build Bitget HTTP client");

        Self { client, base_url }
    }

    /// 格式化交易对符号为 Bitget 格式
    /// 例如：BTCUSDT -> BTCUSDT (不需要后缀)
    fn format_symbol(symbol: &str) -> String {
        symbol.to_uppercase()
    }
}

#[async_trait]
impl ExchangeClient for BitgetClient {
    async fn get_price(&self, symbol: &str) -> anyhow::Result<Price> {
        // 格式化符号
        let bitget_symbol = Self::format_symbol(symbol);
        
        // Bitget API v2: https://api.bitget.com/api/v2/spot/market/tickers
        let url = format!("{}/api/v2/spot/market/tickers", self.base_url);
        
        let response = self
            .client
            .get(&url)
            .query(&[("symbol", bitget_symbol.as_str())])
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Bitget API error {}: {}", status, error_text);
        }

        let bitget_response: BitgetResponse = response.json().await?;
        
        // 检查 API 响应代码
        if bitget_response.code != "00000" {
            anyhow::bail!("Bitget API error: {} - {}", bitget_response.code, bitget_response.msg);
        }

        // 获取第一个 ticker（应该只有一个）
        let ticker = bitget_response.data.first()
            .ok_or_else(|| anyhow::anyhow!("No ticker data returned from Bitget"))?;

        // 解析价格
        let price_value: f64 = ticker.last_pr.parse()
            .map_err(|e| anyhow::anyhow!("Failed to parse price: {}", e))?;

        Ok(Price {
            symbol: ticker.symbol.clone(),
            price: price_value,
            exchange: self.name().to_string(),
            timestamp: Utc::now(),
            cached: false,
        })
    }

    fn name(&self) -> &str {
        "bitget"
    }
}
