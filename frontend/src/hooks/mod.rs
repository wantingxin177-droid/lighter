use leptos::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

// WebSocket状态Hook
pub fn use_websocket_status() -> (ReadSignal<bool>, WriteSignal<bool>) {
    create_signal(false)
}

// WebSocket Hook
pub fn use_websocket(url: &str) -> (ReadSignal<Option<String>>, WriteSignal<Option<String>>) {
    let (message, set_message) = create_signal(None::<String>);
    let (connected, set_connected) = create_signal(false);

    let ws_url = url.to_string();
    
    create_effect(move |_| {
        let ws = WebSocket::new(&ws_url).expect("Failed to create WebSocket");
        
        ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

        // 连接成功回调
        let onopen = Closure::wrap(Box::new(move |_e: JsValue| {
            log::info!("WebSocket connected");
            set_connected.set(true);
        }) as Box<dyn FnMut(JsValue)>);
        ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
        onopen.forget();

        // 消息接收回调
        let onmessage = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(text) = e.data().dyn_into::<js_sys::JsString>() {
                let text = text.as_string().unwrap_or_default();
                set_message.set(Some(text));
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
        onmessage.forget();

        // 错误回调
        let onerror = Closure::wrap(Box::new(move |e: ErrorEvent| {
            log::error!("WebSocket error: {:?}", e);
            set_connected.set(false);
        }) as Box<dyn FnMut(ErrorEvent)>);
        ws.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onerror.forget();

        // 关闭回调
        let onclose = Closure::wrap(Box::new(move |_e: JsValue| {
            log::info!("WebSocket closed");
            set_connected.set(false);
        }) as Box<dyn FnMut(JsValue)>);
        ws.set_onclose(Some(onclose.as_ref().unchecked_ref()));
        onclose.forget();

        // 清理函数
        on_cleanup(move || {
            let _ = ws.close();
        });
    });

    (message, set_message)
}

// API Fetch Hook
pub fn use_api<T>(url: &str) -> (ReadSignal<Option<T>>, ReadSignal<bool>, ReadSignal<Option<String>>> 
where
    T: serde::de::DeserializeOwned + 'static,
{
    let (data, set_data) = create_signal(None::<T>);
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);

    let url = url.to_string();

    create_effect(move |_| {
        set_loading.set(true);
        
        wasm_bindgen_futures::spawn_local(async move {
            match fetch_json::<T>(&url).await {
                Ok(result) => {
                    set_data.set(Some(result));
                    set_error.set(None);
                }
                Err(e) => {
                    set_error.set(Some(e));
                }
            }
            set_loading.set(false);
        });
    });

    (data, loading, error)
}

async fn fetch_json<T: serde::de::DeserializeOwned>(url: &str) -> Result<T, String> {
    let response = gloo_net::http::Request::get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("HTTP error: {}", response.status()));
    }

    response.json::<T>().await.map_err(|e| e.to_string())
}

// 实时数据Hook - 使用WebSocket接收实时更新
pub fn use_realtime_data<T, F>(ws_message: ReadSignal<Option<String>>, parser: F) -> ReadSignal<Vec<T>>
where
    T: Clone + 'static,
    F: Fn(&str) -> Option<T> + 'static,
{
    let (data, set_data) = create_signal(Vec::<T>::new());

    create_effect(move |_| {
        if let Some(msg) = ws_message.get() {
            if let Some(parsed) = parser(&msg) {
                set_data.update(|d| {
                    d.push(parsed);
                    // 只保留最近100条
                    if d.len() > 100 {
                        d.remove(0);
                    }
                });
            }
        }
    });

    data
}
