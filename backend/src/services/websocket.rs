use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tracing::{debug, error, info, warn};

use crate::db::models::WsMessage;
use crate::AppState;

// 频道类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Channel {
    Blocks,
    Transactions,
    Trades(String), // symbol
    OrderBook(String), // symbol
    FundingRates,
}

pub struct WebSocketManager {
    // 广播通道
    tx: broadcast::Sender<WsMessage>,
    // 订阅计数
    subscriptions: Arc<RwLock<HashMap<Channel, usize>>>,
}

impl WebSocketManager {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(10000);
        
        Self {
            tx,
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<WsMessage> {
        self.tx.subscribe()
    }

    pub async fn publish(&self, message: WsMessage) {
        let _ = self.tx.send(message);
    }

    pub async fn subscribe_channel(&self, channel: Channel) {
        let mut subs = self.subscriptions.write().await;
        *subs.entry(channel).or_insert(0) += 1;
        debug!("Subscribed to {:?}", channel);
    }

    pub async fn unsubscribe_channel(&self, channel: Channel) {
        let mut subs = self.subscriptions.write().await;
        if let Some(count) = subs.get_mut(&channel) {
            *count -= 1;
            if *count == 0 {
                subs.remove(&channel);
            }
        }
        debug!("Unsubscribed from {:?}", channel);
    }
}

// WebSocket处理函数
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    info!("New WebSocket connection");
    
    let mut rx = state.ws_manager.subscribe();
    
    loop {
        tokio::select! {
            // 接收客户端消息
            Some(Ok(msg)) = socket.recv() => {
                match msg {
                    Message::Text(text) => {
                        debug!("Received: {}", text);
                        // 处理订阅请求
                        if let Err(e) = handle_client_message(&text, &state.ws_manager).await {
                            warn!("Failed to handle client message: {}", e);
                        }
                    }
                    Message::Close(_) => {
                        info!("Client disconnected");
                        break;
                    }
                    _ => {}
                }
            }
            
            // 广播消息
            Ok(msg) = rx.recv() => {
                let json = match serde_json::to_string(&msg) {
                    Ok(s) => s,
                    Err(e) => {
                        error!("Failed to serialize message: {}", e);
                        continue;
                    }
                };
                
                if let Err(e) = socket.send(Message::Text(json)).await {
                    error!("Failed to send message: {}", e);
                    break;
                }
            }
        }
    }
}

async fn handle_client_message(
    text: &str,
    _manager: &WebSocketManager,
) -> anyhow::Result<()> {
    let msg: serde_json::Value = serde_json::from_str(text)?;
    
    if let Some(method) = msg.get("method").and_then(|m| m.as_str()) {
        match method {
            "subscribe" => {
                // 处理订阅
                debug!("Subscribe request: {:?}", msg);
            }
            "unsubscribe" => {
                // 处理取消订阅
                debug!("Unsubscribe request: {:?}", msg);
            }
            "ping" => {
                // 处理ping
            }
            _ => {
                warn!("Unknown method: {}", method);
            }
        }
    }
    
    Ok(())
}
