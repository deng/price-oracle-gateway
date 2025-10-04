use crate::models::Price;
use crate::services::ExchangeClient;
use async_trait::async_trait;
use chrono::Utc;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct DexScreenerResponse {
    #[serde(rename = "schemaVersion")]
    schema_version: String,
    pairs: Vec<DexPair>,
}

#[derive(Debug, Deserialize)]
struct DexPair {
    #[serde(rename = "chainId")]
    chain_id: String,
    #[serde(rename = "dexId")]
    dex_id: String,
    url: String,
    #[serde(rename = "pairAddress")]
    pair_address: String,
    #[serde(rename = "baseToken")]
    base_token: Token,
    #[serde(rename = "quoteToken")]
    quote_token: Token,
    #[serde(rename = "priceNative")]
    price_native: String,
    #[serde(rename = "priceUsd")]
    price_usd: Option<String>,
    liquidity: Option<Liquidity>,
}

#[derive(Debug, Deserialize)]
struct Liquidity {
    usd: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct Token {
    address: String,
    name: String,
    symbol: String,
}

pub struct DexScreenerClient {
    client: reqwest::Client,
    base_url: String,
}

impl DexScreenerClient {
    pub fn new(base_url: String, timeout: Duration) -> Self {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .expect("Failed to build DexScreener HTTP client");

        Self { client, base_url }
    }

    /// 从 pairs 列表中查找所有匹配 base/quote 的交易对
    /// 按流动性从高到低排序，去除重复的 DEX ID（保留流动性最高的）
    fn find_all_matching_pairs(pairs: Vec<DexPair>, base: &str, quote: &str) -> Vec<DexPair> {
        let base_upper = base.to_uppercase();
        let quote_upper = quote.to_uppercase();

        // 收集所有匹配的交易对
        let mut matching_pairs: Vec<DexPair> = pairs.into_iter()
            .filter(|pair| {
                pair.base_token.symbol.to_uppercase() == base_upper
                    && pair.quote_token.symbol.to_uppercase() == quote_upper
            })
            .collect();

        if matching_pairs.is_empty() {
            return Vec::new();
        }

        // 按流动性从高到低排序
        matching_pairs.sort_by(|a, b| {
            let a_liquidity = a.liquidity.as_ref().and_then(|l| l.usd).unwrap_or(0.0);
            let b_liquidity = b.liquidity.as_ref().and_then(|l| l.usd).unwrap_or(0.0);
            b_liquidity.partial_cmp(&a_liquidity).unwrap_or(std::cmp::Ordering::Equal)
        });

        // 去重：如果有多个相同的 DEX ID，只保留流动性最高的（已排序，所以第一个就是最高的）
        use std::collections::HashMap;
        let mut seen_dex_ids: HashMap<String, bool> = HashMap::new();
        let deduplicated: Vec<DexPair> = matching_pairs.into_iter()
            .filter(|pair| {
                if seen_dex_ids.contains_key(&pair.dex_id) {
                    false // 已经见过这个 DEX ID，跳过
                } else {
                    seen_dex_ids.insert(pair.dex_id.clone(), true);
                    true // 第一次见到，保留
                }
            })
            .collect();

        // 记录日志
        if deduplicated.len() > 1 {
            let dex_names: Vec<String> = deduplicated.iter()
                .map(|p| p.dex_id.clone())
                .collect();
            tracing::info!(
                "Found {} unique DEXs for {}/{}: {}",
                deduplicated.len(),
                base,
                quote,
                dex_names.join(", ")
            );
        }

        deduplicated
    }

    /// 从 pairs 列表中查找匹配 base/quote 的交易对
    /// 返回流动性最高的那个
    fn find_matching_pair(pairs: Vec<DexPair>, base: &str, quote: &str) -> Option<DexPair> {
        Self::find_all_matching_pairs(pairs, base, quote).into_iter().next()
    }
}

#[async_trait]
impl ExchangeClient for DexScreenerClient {
    async fn get_price(&self, symbol: &str) -> anyhow::Result<Price> {
        // 解析 symbol，分离 base 和 quote
        // 例如：ASTERUSDT -> ASTER/USDT
        let (base, quote) = if symbol.contains('/') {
            let parts: Vec<&str> = symbol.split('/').collect();
            if parts.len() != 2 {
                anyhow::bail!("Invalid symbol format. Expected BASE/QUOTE");
            }
            (parts[0].to_string(), parts[1].to_string())
        } else {
            // 尝试自动分离常见的 quote currencies
            let symbol_upper = symbol.to_uppercase();
            if let Some(base) = symbol_upper.strip_suffix("USDT") {
                (base.to_string(), "USDT".to_string())
            } else if let Some(base) = symbol_upper.strip_suffix("USDC") {
                (base.to_string(), "USDC".to_string())
            } else if let Some(base) = symbol_upper.strip_suffix("BNB") {
                (base.to_string(), "BNB".to_string())
            } else if let Some(base) = symbol_upper.strip_suffix("ETH") {
                (base.to_string(), "ETH".to_string())
            } else {
                anyhow::bail!("Cannot parse symbol '{}'. Use BASE/QUOTE format", symbol);
            }
        };

        // DexScreener API: https://api.dexscreener.com/latest/dex/search
        let url = format!("{}/latest/dex/search", self.base_url);
        let query = format!("{}/{}", base, quote);

        let response = self
            .client
            .get(&url)
            .query(&[("q", query.as_str())])
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("DexScreener API error {}: {}", status, error_text);
        }

        let dex_response: DexScreenerResponse = response.json().await?;

        // 查找匹配的交易对
        let pair = Self::find_matching_pair(dex_response.pairs, &base, &quote)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "No matching pair found for {}/{} on DexScreener",
                    base,
                    quote
                )
            })?;

        // 解析价格（使用 priceNative）
        let price_value: f64 = pair
            .price_native
            .parse()
            .map_err(|e| anyhow::anyhow!("Failed to parse price: {}", e))?;

        // 使用 dexId 作为 exchange 名称，格式：dexscreener:pancakeswap
        // 这样前端可以知道具体是哪个 DEX 的价格
        let exchange_name = format!("dexscreener:{}", pair.dex_id);

        // 统一 symbol 格式：移除斜杠，例如 BTC/USDT -> BTCUSDT
        let symbol = format!("{}{}", pair.base_token.symbol, pair.quote_token.symbol);

        Ok(Price {
            symbol,
            price: price_value,
            exchange: exchange_name,
            timestamp: Utc::now(),
            cached: false,
        })
    }

    /// 获取所有可用的 DEX 价格
    async fn get_all_prices(&self, symbol: &str) -> anyhow::Result<Vec<Price>> {
        // 解析 symbol，分离 base 和 quote
        let (base, quote) = if symbol.contains('/') {
            let parts: Vec<&str> = symbol.split('/').collect();
            if parts.len() != 2 {
                anyhow::bail!("Invalid symbol format. Expected BASE/QUOTE");
            }
            (parts[0].to_string(), parts[1].to_string())
        } else {
            // 尝试自动分离常见的 quote currencies
            let symbol_upper = symbol.to_uppercase();
            if let Some(base) = symbol_upper.strip_suffix("USDT") {
                (base.to_string(), "USDT".to_string())
            } else if let Some(base) = symbol_upper.strip_suffix("USDC") {
                (base.to_string(), "USDC".to_string())
            } else if let Some(base) = symbol_upper.strip_suffix("BNB") {
                (base.to_string(), "BNB".to_string())
            } else if let Some(base) = symbol_upper.strip_suffix("ETH") {
                (base.to_string(), "ETH".to_string())
            } else {
                anyhow::bail!("Cannot parse symbol '{}'. Use BASE/QUOTE format", symbol);
            }
        };

        // DexScreener API
        let url = format!("{}/latest/dex/search", self.base_url);
        let query = format!("{}/{}", base, quote);

        let response = self
            .client
            .get(&url)
            .query(&[("q", query.as_str())])
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("DexScreener API error {}: {}", status, error_text);
        }

        let dex_response: DexScreenerResponse = response.json().await?;

        // 查找所有匹配的交易对
        let pairs = Self::find_all_matching_pairs(dex_response.pairs, &base, &quote);
        
        if pairs.is_empty() {
            anyhow::bail!(
                "No matching pair found for {}/{} on DexScreener",
                base,
                quote
            );
        }

        // 转换所有 pairs 为 Price 对象
        let timestamp = Utc::now();
        let mut prices = Vec::new();

        for pair in pairs {
            // 解析价格
            if let Ok(price_value) = pair.price_native.parse::<f64>() {
                let exchange_name = format!("dexscreener:{}", pair.dex_id);
                let symbol = format!("{}{}", pair.base_token.symbol, pair.quote_token.symbol);

                prices.push(Price {
                    symbol,
                    price: price_value,
                    exchange: exchange_name,
                    timestamp,
                    cached: false,
                });
            }
        }

        if prices.is_empty() {
            anyhow::bail!("Failed to parse any prices for {}/{}", base, quote);
        }

        Ok(prices)
    }

    fn name(&self) -> &str {
        "dexscreener"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_matching_pair() {
        let pairs = vec![
            DexPair {
                chain_id: "bsc".to_string(),
                dex_id: "pancakeswap".to_string(),
                url: "https://test.com".to_string(),
                pair_address: "0x123".to_string(),
                base_token: Token {
                    address: "0xabc".to_string(),
                    name: "Aster".to_string(),
                    symbol: "ASTER".to_string(),
                },
                quote_token: Token {
                    address: "0xdef".to_string(),
                    name: "Tether USD".to_string(),
                    symbol: "USDT".to_string(),
                },
                price_native: "1.61".to_string(),
                price_usd: Some("1.61".to_string()),
            },
            DexPair {
                chain_id: "ethereum".to_string(),
                dex_id: "uniswap".to_string(),
                url: "https://test2.com".to_string(),
                pair_address: "0x456".to_string(),
                base_token: Token {
                    address: "0xghi".to_string(),
                    name: "Wrapped BTC".to_string(),
                    symbol: "WBTC".to_string(),
                },
                quote_token: Token {
                    address: "0xjkl".to_string(),
                    name: "USD Coin".to_string(),
                    symbol: "USDC".to_string(),
                },
                price_native: "95000.0".to_string(),
                price_usd: Some("95000.0".to_string()),
            },
        ];

        // 测试匹配 ASTER/USDT
        let result = DexScreenerClient::find_matching_pair(pairs.clone(), "aster", "usdt");
        assert!(result.is_some());
        let pair = result.unwrap();
        assert_eq!(pair.base_token.symbol, "ASTER");
        assert_eq!(pair.quote_token.symbol, "USDT");
        assert_eq!(pair.dex_id, "pancakeswap");

        // 测试匹配 WBTC/USDC
        let result = DexScreenerClient::find_matching_pair(pairs.clone(), "WBTC", "USDC");
        assert!(result.is_some());
        let pair = result.unwrap();
        assert_eq!(pair.base_token.symbol, "WBTC");
        assert_eq!(pair.quote_token.symbol, "USDC");
        assert_eq!(pair.dex_id, "uniswap");

        // 测试不匹配的情况
        let result = DexScreenerClient::find_matching_pair(pairs, "BTC", "ETH");
        assert!(result.is_none());
    }
}
