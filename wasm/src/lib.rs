use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

// 初始化
#[wasm_bindgen(start)]
pub fn start() {
    console_log("🚀 Kin WASM Module Loaded");
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn console_log(s: &str) {
    log(s);
}

// ========== 数据验证模块 ==========

/// 验证区块哈希
#[wasm_bindgen]
pub fn verify_block_hash(height: i64, hash: &str, prev_hash: &str) -> bool {
    use sha2::{Sha256, Digest};
    
    let data = format!("{}:{}", height, prev_hash);
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    let computed_hash = format!("0x{}", hex::encode(result));
    
    computed_hash.to_lowercase() == hash.to_lowercase()
}

/// 批量验证交易哈希
#[wasm_bindgen]
pub fn batch_verify_tx_hashes(tx_hashes_json: &str) -> JsValue {
    #[derive(Deserialize)]
    struct TxInput {
        hash: String,
        data: String,
    }
    
    let txs: Vec<TxInput> = match serde_json::from_str(tx_hashes_json) {
        Ok(txs) => txs,
        Err(_) => return JsValue::from_bool(false),
    };
    
    let results: Vec<bool> = txs.iter().map(|tx| {
        verify_tx_hash(&tx.hash, &tx.data)
    }).collect();
    
    serde_wasm_bindgen::to_value(&results).unwrap_or(JsValue::NULL)
}

fn verify_tx_hash(hash: &str, data: &str) -> bool {
    use sha2::{Sha256, Digest};
    
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    let computed_hash = format!("0x{}", hex::encode(result));
    
    computed_hash.to_lowercase() == hash.to_lowercase()
}

// ========== 数据分析模块 ==========

/// 计算移动平均线
#[wasm_bindgen]
pub fn calculate_ma(prices_json: &str, period: usize) -> JsValue {
    let prices: Vec<f64> = match serde_json::from_str(prices_json) {
        Ok(p) => p,
        Err(_) => return JsValue::NULL,
    };
    
    if prices.len() < period {
        return JsValue::NULL;
    }
    
    let mut ma: Vec<f64> = Vec::new();
    
    for i in period..=prices.len() {
        let sum: f64 = prices[i-period..i].iter().sum();
        ma.push(sum / period as f64);
    }
    
    serde_wasm_bindgen::to_value(&ma).unwrap_or(JsValue::NULL)
}

/// 计算RSI (相对强弱指标)
#[wasm_bindgen]
pub fn calculate_rsi(prices_json: &str, period: usize) -> JsValue {
    let prices: Vec<f64> = match serde_json::from_str(prices_json) {
        Ok(p) => p,
        Err(_) => return JsValue::NULL,
    };
    
    if prices.len() < period + 1 {
        return JsValue::NULL;
    }
    
    let mut gains: Vec<f64> = Vec::new();
    let mut losses: Vec<f64> = Vec::new();
    
    for i in 1..prices.len() {
        let change = prices[i] - prices[i-1];
        if change > 0.0 {
            gains.push(change);
            losses.push(0.0);
        } else {
            gains.push(0.0);
            losses.push(-change);
        }
    }
    
    let mut rsi: Vec<f64> = Vec::new();
    
    for i in period..gains.len() {
        let avg_gain: f64 = gains[i-period..i].iter().sum::<f64>() / period as f64;
        let avg_loss: f64 = losses[i-period..i].iter().sum::<f64>() / period as f64;
        
        if avg_loss == 0.0 {
            rsi.push(100.0);
        } else {
            let rs = avg_gain / avg_loss;
            rsi.push(100.0 - (100.0 / (1.0 + rs)));
        }
    }
    
    serde_wasm_bindgen::to_value(&rsi).unwrap_or(JsValue::NULL)
}

// ========== 订单簿处理模块 ==========

#[derive(Serialize, Deserialize)]
struct OrderBookLevel {
    price: f64,
    amount: f64,
}

/// 聚合订单簿
#[wasm_bindgen]
pub fn aggregate_orderbook(bids_json: &str, asks_json: &str, tick_size: f64) -> JsValue {
    let bids: Vec<OrderBookLevel> = match serde_json::from_str(bids_json) {
        Ok(b) => b,
        Err(_) => return JsValue::NULL,
    };
    
    let asks: Vec<OrderBookLevel> = match serde_json::from_str(asks_json) {
        Ok(a) => a,
        Err(_) => return JsValue::NULL,
    };
    
    let aggregated_bids = aggregate_levels(bids, tick_size);
    let aggregated_asks = aggregate_levels(asks, tick_size);
    
    #[derive(Serialize)]
    struct Result {
        bids: Vec<OrderBookLevel>,
        asks: Vec<OrderBookLevel>,
    }
    
    serde_wasm_bindgen::to_value(&Result {
        bids: aggregated_bids,
        asks: aggregated_asks,
    }).unwrap_or(JsValue::NULL)
}

fn aggregate_levels(levels: Vec<OrderBookLevel>, tick_size: f64) -> Vec<OrderBookLevel> {
    use std::collections::HashMap;
    
    let mut aggregated: HashMap<i64, f64> = HashMap::new();
    
    for level in levels {
        let tick = (level.price / tick_size) as i64;
        *aggregated.entry(tick).or_insert(0.0) += level.amount;
    }
    
    let mut result: Vec<OrderBookLevel> = aggregated
        .into_iter()
        .map(|(tick, amount)| OrderBookLevel {
            price: tick as f64 * tick_size,
            amount,
        })
        .collect();
    
    result.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
    result
}

// ========== 压缩/解压模块 ==========

/// 压缩数据 (使用简单的RLE算法)
#[wasm_bindgen]
pub fn compress_data(data: &[u8]) -> Vec<u8> {
    let mut compressed: Vec<u8> = Vec::new();
    let mut i = 0;
    
    while i < data.len() {
        let mut count = 1u8;
        while i + 1 < data.len() && data[i] == data[i + 1] && count < 255 {
            count += 1;
            i += 1;
        }
        compressed.push(data[i]);
        compressed.push(count);
        i += 1;
    }
    
    compressed
}

/// 解压数据
#[wasm_bindgen]
pub fn decompress_data(data: &[u8]) -> Vec<u8> {
    let mut decompressed: Vec<u8> = Vec::new();
    let mut i = 0;
    
    while i < data.len() {
        if i + 1 < data.len() {
            let value = data[i];
            let count = data[i + 1];
            for _ in 0..count {
                decompressed.push(value);
            }
            i += 2;
        } else {
            break;
        }
    }
    
    decompressed
}

// ========== 性能测试 ==========

#[wasm_bindgen]
pub fn benchmark_hash(iterations: usize) -> f64 {
    use sha2::{Sha256, Digest};
    use js_sys::Date;
    
    let start = Date::now();
    
    let data = b"benchmark_data_123456789";
    
    for _ in 0..iterations {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let _ = hasher.finalize();
    }
    
    let end = Date::now();
    (end - start) / 1000.0 // 返回秒数
}
