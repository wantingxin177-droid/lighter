use leptos::*;
use leptos_router::*;
use crate::components::*;
use crate::pages::{PageTitle, SectionTitle, StatCard};
use crate::hooks::use_api;
use shared::Stats;

#[component]
pub fn Home() -> impl IntoView {
    let (stats, stats_loading, _stats_error) = use_api::<Stats>("/api/v1/node/status");

    view! {
        <div>
            <PageTitle title="Dashboard".to_string()/>
            
            // 统计卡片
            <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8">
                <StatCard 
                    label="Total Blocks".to_string()
                    value=move || stats.get().map(|s| s.total_blocks.to_string()).unwrap_or_else(|| "-".to_string())
                />
                <StatCard 
                    label="Total Transactions".to_string()
                    value=move || stats.get().map(|s| s.total_transactions.to_string()).unwrap_or_else(|| "-".to_string())
                />
                <StatCard 
                    label="Latest Height".to_string()
                    value=move || stats.get().map(|s| s.latest_height.to_string()).unwrap_or_else(|| "-".to_string())
                />
                <StatCard 
                    label="WebSocket".to_string()
                    value=move || stats.get().map(|s| if s.ws_connected { "Connected".to_string() } else { "Disconnected".to_string() }).unwrap_or_else(|| "-".to_string())
                />
            </div>

            // 快速导航
            <SectionTitle title="Quick Navigation".to_string()/>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <NavigationCard 
                    title="Block Explorer"
                    description="Browse blocks and transactions"
                    href="/explorer"
                    icon="🔍"
                />
                <NavigationCard 
                    title="Markets"
                    description="View market data and orderbooks"
                    href="/markets"
                    icon="📈"
                />
                <NavigationCard 
                    title="API Docs"
                    description="Explore the API"
                    href="/api/v1/health"
                    icon="📚"
                />
            </div>
        </div>
    }
}

#[component]
fn NavigationCard(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    #[prop(into)] href: String,
    #[prop(into)] icon: String,
) -> impl IntoView {
    view! {
        <A href=href class="block">
            <Card>
                <div class="flex items-start gap-4">
                    <span class="text-3xl">{icon}</span>
                    <div>
                        <h3 class="font-semibold text-lg">{title}</h3>
                        <p class="text-sm text-zinc-500 mt-1">{description}</p>
                    </div>
                </div>
            </Card>
        </A>
    }
}
