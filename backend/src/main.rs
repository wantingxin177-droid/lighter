use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    trace::TraceLayer,
};
use tracing::{info, warn};

mod config;
mod db;
mod error;
mod handlers;
mod middleware;
mod services;

use crate::config::AppConfig;
use crate::db::{Database, RedisCache};
use crate::services::{websocket, BlockService, MarketService, WebSocketManager};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub cache: Arc<RedisCache>,
    pub block_service: Arc<BlockService>,
    pub market_service: Arc<MarketService>,
    pub ws_manager: Arc<WebSocketManager>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,kin_backend=debug".into()),
        )
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    info!("🚀 Starting Kin High-Performance Backend");

    // 加载配置
    let config = AppConfig::from_env()?;
    info!("✅ Configuration loaded");

    // 初始化数据库连接池
    let db = Arc::new(Database::new(&config.database_url).await?);
    info!("✅ Database connected");

    // 初始化Redis缓存
    let cache = Arc::new(RedisCache::new(&config.redis_url).await?);
    info!("✅ Redis connected");

    // 初始化内存缓存
    let memory_cache = moka::future::Cache::builder()
        .max_capacity(100_000)
        .time_to_live(std::time::Duration::from_secs(60))
        .build();

    // 初始化服务
    let block_service = Arc::new(BlockService::new(db.clone(), cache.clone(), memory_cache.clone()));
    let market_service = Arc::new(MarketService::new(db.clone(), cache.clone()));
    let ws_manager = Arc::new(WebSocketManager::new());
    info!("✅ Services initialized");

    // 启动后台同步任务
    let sync_db = db.clone();
    let sync_cache = cache.clone();
    tokio::spawn(async move {
        services::start_sync_loop(sync_db, sync_cache).await;
    });
    info!("✅ Background sync started");

    // 构建应用状态
    let state = AppState {
        db,
        cache,
        block_service,
        market_service,
        ws_manager,
    };

    // 构建路由
    let app = create_router(state);

    // 启动服务器
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("🌐 Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("👋 Server shutdown complete");
    Ok(())
}

fn create_router(state: AppState) -> Router {
    // API路由
    let api_routes = Router::new()
        // 区块相关
        .route("/blocks/latest", get(handlers::get_latest_block))
        .route("/blocks/:height", get(handlers::get_block_by_height))
        .route("/blocks/:height/txs", get(handlers::get_block_transactions))
        .route("/blocks/range", get(handlers::get_blocks_range))
        // 交易相关
        .route("/txs/:hash", get(handlers::get_transaction))
        .route("/txs/recent", get(handlers::get_recent_transactions))
        // 市场数据
        .route("/markets/overview", get(handlers::get_market_overview))
        .route("/markets/:symbol/orderbook", get(handlers::get_orderbook))
        .route("/markets/:symbol/trades", get(handlers::get_recent_trades))
        .route("/markets/:symbol/funding", get(handlers::get_funding_rate))
        // WebSocket升级
        .route("/ws", get(services::websocket::ws_handler))
        // 健康检查
        .route("/health", get(handlers::health_check))
        .route("/metrics", get(handlers::metrics_handler))
        .with_state(state);

    // 根路由
    Router::new()
        .nest("/api/v1", api_routes)
        .layer(CompressionLayer::new().gzip(true).br(true))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => warn!("Received Ctrl+C, shutting down..."),
        _ = terminate => warn!("Received SIGTERM, shutting down..."),
    }
}
