use axum::{
    body::Body,
    extract::Request,
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use tracing::{info, warn};

// 请求日志中间件
pub async fn request_logger(req: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();
    
    let response = next.run(req).await;
    
    let duration = start.elapsed();
    let status = response.status();
    
    info!(
        method = %method,
        uri = %uri,
        status = %status,
        duration = ?duration,
        "Request completed"
    );
    
    response
}

// 限流中间件 (基于IP)
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, Instant as TokioInstant};

#[derive(Clone)]
pub struct RateLimiter {
    requests: Arc<RwLock<HashMap<String, Vec<TokioInstant>>>>
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            requests: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub async fn check(&self, key: &str, max_requests: usize, window: Duration) -> bool {
        let now = TokioInstant::now();
        let mut requests = self.requests.write().await;
        
        let entry = requests.entry(key.to_string()).or_insert_with(Vec::new);
        
        // 清理过期请求
        entry.retain(|&time| now.duration_since(time) < window);
        
        if entry.len() >= max_requests {
            return false;
        }
        
        entry.push(now);
        true
    }
}

// CORS预检中间件
pub async fn cors_preflight(req: Request, next: Next) -> Response {
    if req.method() == axum::http::Method::OPTIONS {
        return Response::builder()
            .status(axum::http::StatusCode::NO_CONTENT)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
            .header("Access-Control-Allow-Headers", "Content-Type, Authorization")
            .body(Body::empty())
            .unwrap();
    }
    
    next.run(req).await
}
