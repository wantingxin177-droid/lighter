use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// 共享的数据模型 - 用于前后端通信

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub height: i64,
    pub hash: String,
    pub prev_hash: String,
    pub timestamp: DateTime<Utc>,
    pub tx_count: i32,
    pub l1_batch_number: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub hash: String,
    pub block_height: i64,
    pub from_addr: String,
    pub to_addr: String,
    pub value: String,
    pub gas_price: String,
    pub gas_used: i64,
    pub status: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    pub symbol: String,
    pub bids: Vec<OrderBookLevel>,
    pub asks: Vec<OrderBookLevel>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookLevel {
    pub price: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub id: i64,
    pub symbol: String,
    pub price: String,
    pub amount: String,
    pub side: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketOverview {
    pub symbol: String,
    pub price: String,
    pub volume_24h: String,
    pub change_24h: String,
    pub high_24h: String,
    pub low_24h: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingRate {
    pub symbol: String,
    pub rate: String,
    pub next_funding_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub total_blocks: i64,
    pub total_transactions: i64,
    pub latest_height: i64,
    pub ws_connected: bool,
}

// API 响应类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(msg: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.into()),
        }
    }
}
