use leptos::*;
use leptos_router::*;
use crate::components::*;
use crate::pages::{PageTitle, SectionTitle};
use crate::hooks::use_api;
use shared::MarketOverview;

#[component]
pub fn Markets() -> impl IntoView {
    let (markets, _loading, _error) = use_api::<Vec<MarketOverview>>("/api/v1/markets/overview");

    let symbols = vec!["BTC-USD", "ETH-USD", "SOL-USD", "ARB-USD"];

    view! {
        <div>
            <PageTitle title=move || "Markets".to_string()/>
            
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                {symbols.into_iter().map(|symbol| view! {
                    <MarketCard symbol=symbol.to_string()/>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[component]
fn MarketCard(#[prop(into)] symbol: String) -> impl IntoView {
    view! {
        <A href=format!("/markets/{}", symbol) class="block">
            <Card>
                <div class="flex items-center justify-between mb-4">
                    <h3 class="font-bold text-lg">{symbol.clone()}</h3>
                    <span class="text-xs bg-emerald-500/20 text-emerald-400 px-2 py-1 rounded">
                        "+2.45%"
                    </span>
                </div>
                <div class="space-y-2">
                    <div class="flex justify-between text-sm">
                        <span class="text-zinc-500">"Price"</span>
                        <span class="font-mono">"$65,432.10"</span>
                    </div>
                    <div class="flex justify-between text-sm">
                        <span class="text-zinc-500">"24h Volume"</span>
                        <span class="font-mono">"$1.2B"</span>
                    </div>
                    <div class="flex justify-between text-sm">
                        <span class="text-zinc-500">"24h High"</span>
                        <span class="font-mono">"$66,000.00"</span>
                    </div>
                    <div class="flex justify-between text-sm">
                        <span class="text-zinc-500">"24h Low"</span>
                        <span class="font-mono">"$64,000.00"</span>
                    </div>
                </div>
            </Card>
        </A>
    }
}
