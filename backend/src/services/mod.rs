use crate::db::{Database, RedisCache};
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::{error, info, warn};

pub mod block_service;
pub mod market_service;
pub mod sync_service;
pub mod websocket;

pub use block_service::BlockService;
pub use market_service::MarketService;
pub use websocket::WebSocketManager;

// 启动后台同步循环
pub async fn start_sync_loop(db: Arc<Database>, cache: Arc<RedisCache>) {
    let mut ticker = interval(Duration::from_secs(1));
    
    info!("🔄 Starting background sync loop");
    
    loop {
        ticker.tick().await;
        
        // 同步最新区块
        if let Err(e) = sync_service::sync_latest_blocks(db.clone(), cache.clone()).await {
            error!("Failed to sync blocks: {}", e);
        }
        
        // 同步市场数据
        if let Err(e) = sync_service::sync_market_data(db.clone(), cache.clone()).await {
            error!("Failed to sync market data: {}", e);
        }
    }
}
