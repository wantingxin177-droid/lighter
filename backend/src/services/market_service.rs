use crate::db::{models::*, Database, RedisCache};
use std::sync::Arc;

pub struct MarketService {
    db: Arc<Database>,
    cache: Arc<RedisCache>,
}

impl MarketService {
    pub fn new(db: Arc<Database>, cache: Arc<RedisCache>) -> Self {
        Self { db, cache }
    }

    pub async fn get_orderbook(&self, symbol: &str) -> anyhow::Result<OrderBook> {
        let cache_key = format!("orderbook:{}", symbol);
        
        // 从Redis获取（订单簿需要实时性）
        if let Some(ob) = self.cache.get::<OrderBook>(&cache_key).await? {
            return Ok(ob);
        }

        // 如果没有缓存，返回空订单簿
        Ok(OrderBook {
            symbol: symbol.to_string(),
            bids: vec![],
            asks: vec![],
            timestamp: chrono::Utc::now(),
        })
    }

    pub async fn update_orderbook(&self, orderbook: &OrderBook) -> anyhow::Result<()> {
        let cache_key = format!("orderbook:{}", orderbook.symbol);
        // 订单簿缓存1秒
        self.cache.set(&cache_key, orderbook, 1).await?;
        Ok(())
    }

    pub async fn get_recent_trades(&self, symbol: &str, limit: i64) -> anyhow::Result<Vec<Trade>> {
        let cache_key = format!("trades:{}:{}", symbol, limit);
        
        // 尝试从缓存获取
        if let Some(trades) = self.cache.get::<Vec<Trade>>(&cache_key).await? {
            return Ok(trades);
        }

        // 查询数据库
        let trades = self.db.get_recent_trades(symbol, limit).await?;
        
        // 缓存5秒
        self.cache.set(&cache_key, &trades, 5).await?;
        
        Ok(trades)
    }

    pub async fn get_market_overview(&self) -> anyhow::Result<serde_json::Value> {
        let cache_key = "market:overview";
        
        if let Some(overview) = self.cache.get::<serde_json::Value>(cache_key).await? {
            return Ok(overview);
        }

        // 构造市场概览数据
        let overview = serde_json::json!({
            "status": "ok",
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });

        self.cache.set(cache_key, &overview, 10).await?;
        
        Ok(overview)
    }
}
