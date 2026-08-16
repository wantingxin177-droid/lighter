use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
mod hooks;
mod pages;
mod utils;

use components::*;
use pages::*;

#[component]
pub fn App() -> impl IntoView {
    // 提供元数据上下文
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/style.css"/>
        <Title text="LighterAnalyzer - High Performance"/>
        <Meta name="description" content="High-performance Lighter blockchain analyzer"/>

        <Router>
            <div class="min-h-screen bg-zinc-950 text-zinc-100">
                <Header/>
                <main class="max-w-7xl mx-auto px-4 py-6">
                    <Routes>
                        <Route path="/" view=Home/>
                        <Route path="/explorer" view=Explorer/>
                        <Route path="/block/:height" view=BlockDetail/>
                        <Route path="/tx/:hash" view=TransactionDetail/>
                        <Route path="/markets" view=Markets/>
                        <Route path="/markets/:symbol" view=MarketDetail/>
                        <Route path="/*any" view=NotFound/>
                    </Routes>
                </main>
                <Footer/>
            </div>
        </Router>
    }
}

#[component]
fn Header() -> impl IntoView {
    let (ws_connected, _) = hooks::use_websocket_status();

    view! {
        <header class="border-b border-zinc-800 bg-zinc-950/80 sticky top-0 z-50 backdrop-blur">
            <div class="max-w-7xl mx-auto px-4 py-3 flex items-center gap-3">
                <div class="w-8 h-8 rounded-lg bg-emerald-500/20 flex items-center justify-center">
                    <svg class="w-5 h-5 text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/>
                    </svg>
                </div>
                <div>
                    <h1 class="font-bold text-lg leading-tight">"LighterAnalyzer"</h1>
                    <p class="text-xs text-zinc-500">"Rust + WASM High Performance Edition"</p>
                </div>
                <div class="ml-auto flex items-center gap-4 text-xs text-zinc-500">
                    <span class="flex items-center gap-1.5">
                        <span class={move || {
                            if ws_connected.get() {
                                "w-2 h-2 rounded-full bg-emerald-400 animate-pulse"
                            } else {
                                "w-2 h-2 rounded-full bg-red-400"
                            }
                        }}/>
                        {move || if ws_connected.get() { "WS Connected" } else { "WS Disconnected" }}
                    </span>
                </div>
            </div>
        </header>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="border-t border-zinc-800 mt-10 py-4 text-xs text-zinc-600 text-center">
            <p>"LighterAnalyzer High Performance Edition - Built with Rust + Leptos + WebAssembly"</p>
            <p class="mt-1">"Data does not constitute investment advice"</p>
        </footer>
    }
}
