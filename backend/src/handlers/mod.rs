use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::db::models::*;
use crate::AppState;

// 健康检查
pub async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

// Prometheus 指标
pub async fn metrics_handler() -> Result<String, StatusCode> {
    // 这里可以集成 metrics-exporter-prometheus
    Ok("# metrics".to_string())
}

// ========== 区块处理器 ==========

pub async fn get_latest_block(
    State(state): State<AppState>,
) -> Result<Json<Block>, StatusCode> {
    state
        .block_service
        .get_latest_block()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn get_block_by_height(
    State(state): State<AppState>,
    Path(height): Path<i64>,
) -> Result<Json<Block>, StatusCode> {
    state
        .block_service
        .get_block_by_height(height)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

#[derive(Debug, Deserialize)]
pub struct RangeQuery {
    pub start: i64,
    pub end: i64,
}

pub async fn get_blocks_range(
    State(state): State<AppState>,
    Query(params): Query<RangeQuery>,
) -> Result<Json<Vec<Block>>, StatusCode> {
    state
        .block_service
        .get_blocks_range(params.start, params.end)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_block_transactions(
    State(state): State<AppState>,
    Path(height): Path<i64>,
) -> Result<Json<Vec<Transaction>>, StatusCode> {
    state
        .db
        .get_block_transactions(height)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// ========== 交易处理器 ==========

pub async fn get_transaction(
    State(state): State<AppState>,
    Path(hash): Path<String>,
) -> Result<Json<Transaction>, StatusCode> {
    state
        .db
        .get_transaction(&hash)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

#[derive(Debug, Deserialize)]
pub struct LimitQuery {
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_limit() -> i64 {
    20
}

pub async fn get_recent_transactions(
    State(state): State<AppState>,
    Query(params): Query<LimitQuery>,
) -> Result<Json<Vec<Transaction>>, StatusCode> {
    state
        .db
        .get_recent_transactions(params.limit)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// ========== 市场数据处理器 ==========

pub async fn get_market_overview(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    state
        .market_service
        .get_market_overview()
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_orderbook(
    State(state): State<AppState>,
    Path(symbol): Path<String>,
) -> Result<Json<OrderBook>, StatusCode> {
    state
        .market_service
        .get_orderbook(&symbol)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_recent_trades(
    State(state): State<AppState>,
    Path(symbol): Path<String>,
    Query(params): Query<LimitQuery>,
) -> Result<Json<Vec<Trade>>, StatusCode> {
    state
        .market_service
        .get_recent_trades(&symbol, params.limit)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_funding_rate(
    State(state): State<AppState>,
    Path(symbol): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // 实现资金费率查询
    Ok(Json(serde_json::json!({
        "symbol": symbol,
        "rate": "0.0001",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    })))
}

// ========== 状态处理器 ==========

#[derive(Debug, Serialize)]
pub struct StatusResponse {
    pub connected: bool,
    pub counts: Counts,
}

#[derive(Debug, Serialize)]
pub struct Counts {
    pub blocks: i64,
    pub txs: i64,
    pub latest_height: i64,
}

pub async fn get_status(State(state): State<AppState>) -> Result<Json<StatusResponse>, StatusCode> {
    let stats = state
        .db
        .get_stats()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(StatusResponse {
        connected: stats.ws_connected,
        counts: Counts {
            blocks: stats.total_blocks,
            txs: stats.total_transactions,
            latest_height: stats.latest_height,
        },
    }))
}
