use kin_frontend::App;
use leptos::*;

fn main() {
    // 初始化WASM panic hook
    console_error_panic_hook::set_once();
    
    // 初始化日志
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    
    log::info!("🚀 LighterAnalyzer Frontend Starting");
    
    mount_to_body(|| view! { <App/> });
}
