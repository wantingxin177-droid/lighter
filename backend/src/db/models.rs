use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// 区块模型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Block {
    pub height: i64,
    pub hash: String,
    pub prev_hash: String,
    pub timestamp: DateTime<Utc>,
    pub tx_count: i32,
    pub l1_batch_number: Option<i64>,
    pub created_at: DateTime<Utc>,
}

// 交易模型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
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
    pub created_at: DateTime<Utc>,
}

// 市场行情
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MarketData {
    pub id: i64,
    pub symbol: String,
    pub price: String,
    pub volume_24h: String,
    pub change_24h: String,
    pub high_24h: String,
    pub low_24h: String,
    pub timestamp: DateTime<Utc>,
}

// 订单簿
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

// 成交记录
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Trade {
    pub id: i64,
    pub symbol: String,
    pub price: String,
    pub amount: String,
    pub side: String, // buy/sell
    pub timestamp: DateTime<Utc>,
}

// 资金费率
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FundingRate {
    pub id: i64,
    pub symbol: String,
    pub rate: String,
    pub next_funding_time: DateTime<Utc>,
    pub timestamp: DateTime<Utc>,
}

// L1 批次
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct L1Batch {
    pub batch_number: i64,
    pub l1_tx_hash: String,
    pub block_range_start: i64,
    pub block_range_end: i64,
    pub timestamp: DateTime<Utc>,
    pub status: String,
}

// 统计概览
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsOverview {
    pub total_blocks: i64,
    pub total_transactions: i64,
    pub latest_height: i64,
    pub ws_connected: bool,
    pub last_sync_time: DateTime<Utc>,
}

// WebSocket消息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WsMessage {
    Block(Block),
    Transaction(Transaction),
    Trade(Trade),
    OrderBook(OrderBook),
    FundingRate(FundingRate),
    Ping,
    Pong,
}
