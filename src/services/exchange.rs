use crate::models::Price;
use async_trait::async_trait;

/// 交易所客户端 trait
#[async_trait]
pub trait ExchangeClient: Send + Sync {
    /// 获取交易对价格
    async fn get_price(&self, symbol: &str) -> anyhow::Result<Price>;
    
    /// 获取所有可用的交易对价格（用于支持多 DEX 返回）
    /// 默认实现返回单个价格
    async fn get_all_prices(&self, symbol: &str) -> anyhow::Result<Vec<Price>> {
        let price = self.get_price(symbol).await?;
        Ok(vec![price])
    }
    
    /// 获取交易所名称
    fn name(&self) -> &str;
}
