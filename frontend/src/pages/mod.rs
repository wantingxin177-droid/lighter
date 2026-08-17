use leptos::*;
use leptos_router::*;

pub mod home;
pub mod explorer;
pub mod block_detail;
pub mod tx_detail;
pub mod markets;
pub mod market_detail;
pub mod not_found;

pub use home::*;
pub use explorer::*;
pub use block_detail::*;
pub use tx_detail::*;
pub use markets::*;
pub use market_detail::*;
pub use not_found::*;

// 共享的页面组件

#[component]
pub fn PageTitle(#[prop(into)] title: Signal<String>) -> impl IntoView {
    view! {
        <h1 class="text-2xl font-bold mb-6">{title}</h1>
    }
}

#[component]
pub fn SectionTitle(#[prop(into)] title: Signal<String>) -> impl IntoView {
    view! {
        <h2 class="text-lg font-semibold mb-4 text-zinc-300">{title}</h2>
    }
}

#[component]
pub fn StatCard(
    #[prop(into)] label: String,
    #[prop(into)] value: Signal<String>,
    #[prop(optional)] change: Option<String>,
) -> impl IntoView {
    view! {
        <div class="bg-zinc-900 border border-zinc-800 rounded-lg p-4">
            <p class="text-sm text-zinc-500">{label}</p>
            <p class="text-2xl font-bold mt-1">{value}</p>
            {change.map(|c| view! {
                <p class="text-sm mt-1 text-emerald-400">{c}</p>
            })}
        </div>
    }
}
