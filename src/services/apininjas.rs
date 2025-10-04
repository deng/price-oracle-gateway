use crate::models::Price;
use crate::services::ExchangeClient;
use async_trait::async_trait;
use chrono::Utc;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct ApiNinjasResponse {
    symbol: String,
    price: String,
}

pub struct ApiNinjasClient {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
}

impl ApiNinjasClient {
    pub fn new(base_url: String, api_key: String, timeout: Duration) -> Self {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .expect("Failed to build API Ninjas HTTP client");

        Self {
            client,
            base_url,
            api_key,
        }
    }

    /// 从交易对符号提取基础货币和报价货币
    /// 例如：BTCUSD -> (BTC, USD)
    fn parse_symbol(symbol: &str) -> anyhow::Result<(String, String)> {
        let symbol_upper = symbol.to_uppercase();
        
        // 尝试常见的报价货币
        let quote_currencies = ["USDT", "USDC", "USD", "EUR", "GBP", "JPY", "BTC", "ETH"];
        
        for quote in &quote_currencies {
            if symbol_upper.ends_with(quote) {
                let base = symbol_upper.strip_suffix(quote)
                    .ok_or_else(|| anyhow::anyhow!("Failed to extract base currency"))?;
                if !base.is_empty() {
                    return Ok((base.to_string(), quote.to_string()));
                }
            }
        }
        
        anyhow::bail!("Unable to parse symbol '{}'. Expected format like BTCUSD, ETHUSDT", symbol)
    }
}

#[async_trait]
impl ExchangeClient for ApiNinjasClient {
    async fn get_price(&self, symbol: &str) -> anyhow::Result<Price> {
        // 解析交易对
        let (base, quote) = Self::parse_symbol(symbol)?;
        
        // API Ninjas 端点: https://api.api-ninjas.com/v1/cryptoprice?symbol={symbol}
        // 参数格式：BTCUSD (不需要分隔符)
        let api_symbol = format!("{}{}", base, quote);
        let url = format!("{}/v1/cryptoprice", self.base_url);
        
        let response = self
            .client
            .get(&url)
            .header("X-Api-Key", &self.api_key)
            .query(&[("symbol", api_symbol.as_str())])
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("API Ninjas API error {}: {}", status, error_text);
        }

        let data: ApiNinjasResponse = response.json().await?;
        let price_value: f64 = data.price.parse()
            .map_err(|e| anyhow::anyhow!("Failed to parse price: {}", e))?;

        Ok(Price {
            symbol: data.symbol,
            price: price_value,
            exchange: self.name().to_string(),
            timestamp: Utc::now(),
            cached: false,
        })
    }

    fn name(&self) -> &str {
        "apininjas"
    }
}
