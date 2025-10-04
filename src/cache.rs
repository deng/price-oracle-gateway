use crate::models::Price;
use moka::future::Cache;
use std::sync::Arc;
use std::time::Duration;

/// 价格缓存的 key 格式："{exchange}:{symbol}"
/// 例如："binance:BTCUSDT", "dexscreener:pancakeswap:ASTERUSDT"
#[derive(Clone)]
pub struct PriceCache {
    // 主缓存：用于 Binance, Bitget 等交易所（10秒 TTL）
    cache: Cache<String, Arc<Price>>,
    // DexScreener 专用缓存（30秒 TTL）
    dex_cache: Cache<String, Arc<Vec<Price>>>,
}

impl PriceCache {
    /// 创建新的价格缓存
    /// 
    /// # 参数
    /// - `ttl`: 缓存过期时间（秒）- 用于常规交易所
    /// - `max_capacity`: 最大缓存条目数
    pub fn new(ttl_secs: u64, max_capacity: u64) -> Self {
        let cache = Cache::builder()
            .max_capacity(max_capacity)
            .time_to_live(Duration::from_secs(ttl_secs))
            .build();

        // DexScreener 缓存使用 30 秒 TTL
        let dex_cache = Cache::builder()
            .max_capacity(max_capacity)
            .time_to_live(Duration::from_secs(30))
            .build();

        Self { cache, dex_cache }
    }

    /// 生成缓存 key
    fn make_key(exchange: &str, symbol: &str) -> String {
        format!("{}:{}", exchange, symbol)
    }

    /// 获取缓存的价格
    pub async fn get(&self, exchange: &str, symbol: &str) -> Option<Price> {
        let key = Self::make_key(exchange, symbol);
        self.cache.get(&key).await.map(|arc| {
            let mut price = (*arc).clone();
            price.cached = true;  // 标记为来自缓存
            price
        })
    }

    /// 设置价格到缓存
    pub async fn set(&self, price: Price) {
        let key = Self::make_key(&price.exchange, &price.symbol);
        self.cache.insert(key, Arc::new(price)).await;
    }

    /// 获取 DexScreener 的所有 DEX 价格（从缓存）
    /// key 格式："{symbol}" 例如："ASTERUSDT"
    pub async fn get_dex_prices(&self, symbol: &str) -> Option<Vec<Price>> {
        self.dex_cache.get(symbol).await.map(|arc| {
            // 标记所有价格为来自缓存
            arc.iter().map(|p| {
                let mut price = p.clone();
                price.cached = true;
                price
            }).collect()
        })
    }

    /// 设置 DexScreener 的所有 DEX 价格到缓存（30秒 TTL）
    /// key 格式："{symbol}" 例如："ASTERUSDT"
    pub async fn set_dex_prices(&self, symbol: &str, prices: Vec<Price>) {
        self.dex_cache.insert(symbol.to_string(), Arc::new(prices)).await;
    }

    /// 获取所有交易所的缓存价格
    pub async fn get_all(&self, exchanges: &[&str], symbol: &str) -> Vec<Price> {
        let mut prices = Vec::new();
        
        for exchange in exchanges {
            if let Some(price) = self.get(exchange, symbol).await {
                prices.push(price);
            }
        }
        
        prices
    }

    /// 获取缓存统计信息
    pub async fn stats(&self) -> CacheStats {
        CacheStats {
            entry_count: self.cache.entry_count(),
            weighted_size: self.cache.weighted_size(),
        }
    }

    /// 清空缓存
    pub async fn clear(&self) {
        self.cache.invalidate_all();
        // 等待清空操作完成
        self.cache.run_pending_tasks().await;
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub entry_count: u64,
    pub weighted_size: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_cache_set_and_get() {
        let cache = PriceCache::new(10, 100);
        
        let price = Price {
            symbol: "BTCUSDT".to_string(),
            price: 65432.10,
            exchange: "binance".to_string(),
            timestamp: Utc::now(),
            cached: false,
        };

        cache.set(price.clone()).await;
        
        let cached = cache.get("binance", "BTCUSDT").await;
        assert!(cached.is_some());
        
        let cached_price = cached.unwrap();
        assert_eq!(cached_price.symbol, "BTCUSDT");
        assert_eq!(cached_price.price, 65432.10);
        assert_eq!(cached_price.exchange, "binance");
        assert_eq!(cached_price.cached, true); // 验证缓存标记
    }

    #[tokio::test]
    async fn test_cache_expiry() {
        let cache = PriceCache::new(1, 100); // 1 秒过期
        
        let price = Price {
            symbol: "ETHUSDT".to_string(),
            price: 3456.78,
            exchange: "binance".to_string(),
            timestamp: Utc::now(),
            cached: false,
        };

        cache.set(price).await;
        
        // 立即获取应该有值
        let cached = cache.get("binance", "ETHUSDT").await;
        assert!(cached.is_some());
        
        // 等待 2 秒后应该过期
        tokio::time::sleep(Duration::from_secs(2)).await;
        let cached = cache.get("binance", "ETHUSDT").await;
        assert!(cached.is_none());
    }

    #[tokio::test]
    async fn test_cache_get_all() {
        let cache = PriceCache::new(10, 100);
        
        let prices = vec![
            Price {
                symbol: "BTCUSDT".to_string(),
                price: 65432.10,
                exchange: "binance".to_string(),
                timestamp: Utc::now(),
                cached: false,
            },
            Price {
                symbol: "BTCUSDT".to_string(),
                price: 65430.50,
                exchange: "bitget".to_string(),
                timestamp: Utc::now(),
                cached: false,
            },
        ];

        for price in &prices {
            cache.set(price.clone()).await;
        }
        
        let cached = cache.get_all(&["binance", "bitget", "dexscreener"], "BTCUSDT").await;
        assert_eq!(cached.len(), 2); // dexscreener 没有缓存，所以只有 2 个
    }

    #[tokio::test]
    async fn test_cache_clear() {
        let cache = PriceCache::new(10, 100);
        
        let price = Price {
            symbol: "BTCUSDT".to_string(),
            price: 65432.10,
            exchange: "binance".to_string(),
            timestamp: Utc::now(),
            cached: false,
        };

        cache.set(price).await;
        assert!(cache.get("binance", "BTCUSDT").await.is_some());
        
        cache.clear().await;
        assert!(cache.get("binance", "BTCUSDT").await.is_none());
    }
}
