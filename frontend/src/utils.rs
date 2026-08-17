use wasm_bindgen::prelude::*;

// 格式化数字
pub fn format_number(num: i64) -> String {
    if num >= 1_000_000_000 {
        format!("{:.2}B", num as f64 / 1_000_000_000.0)
    } else if num >= 1_000_000 {
        format!("{:.2}M", num as f64 / 1_000_000.0)
    } else if num >= 1_000 {
        format!("{:.2}K", num as f64 / 1_000.0)
    } else {
        num.to_string()
    }
}

// 格式化地址（缩短）
pub fn format_address(addr: &str) -> String {
    if addr.len() > 12 {
        format!("{}...{}", &addr[..6], &addr[addr.len()-4..])
    } else {
        addr.to_string()
    }
}

// 格式化时间差
pub fn format_time_ago(timestamp: i64) -> String {
    let now = js_sys::Date::now() as i64 / 1000;
    let diff = now - timestamp;
    
    if diff < 60 {
        format!("{}s ago", diff)
    } else if diff < 3600 {
        format!("{}m ago", diff / 60)
    } else if diff < 86400 {
        format!("{}h ago", diff / 3600)
    } else {
        format!("{}d ago", diff / 86400)
    }
}

// 复制到剪贴板
pub async fn copy_to_clipboard(text: &str) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let navigator = window.navigator();
    let clipboard = navigator.clipboard();
    
    let promise = clipboard.write_text(text);
    wasm_bindgen_futures::JsFuture::from(promise).await?;
    
    Ok(())
}
