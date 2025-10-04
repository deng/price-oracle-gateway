use crate::models::Price;
use crate::services::ExchangeClient;
use async_trait::async_trait;
use chrono::Utc;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct BinanceTickerResponse {
    symbol: String,
    price: String,
}

pub struct BinanceClient {
    client: reqwest::Client,
    base_url: String,
}

impl BinanceClient {
    pub fn new(base_url: String, timeout: Duration) -> Self {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .expect("Failed to build Binance HTTP client");

        Self { client, base_url }
    }
}

#[async_trait]
impl ExchangeClient for BinanceClient {
    async fn get_price(&self, symbol: &str) -> anyhow::Result<Price> {
        let url = format!("{}/api/v3/ticker/price", self.base_url);
        
        let response = self
            .client
            .get(&url)
            .query(&[("symbol", symbol)])
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Binance API error {}: {}", status, error_text);
        }

        let ticker: BinanceTickerResponse = response.json().await?;
        let price_value: f64 = ticker.price.parse()?;

        Ok(Price {
            symbol: ticker.symbol,
            price: price_value,
            exchange: self.name().to_string(),
            timestamp: Utc::now(),
            cached: false,
        })
    }

    fn name(&self) -> &str {
        "binance"
    }
}
