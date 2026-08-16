use leptos::*;
use leptos_router::*;
use crate::components::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center py-20">
            <h1 class="text-6xl font-bold text-zinc-700 mb-4">"404"</h1>
            <p class="text-xl text-zinc-500 mb-8">"Page not found"</p>
            <A href="/" class="text-emerald-400 hover:text-emerald-300">
                "← Back to Home"
            </A>
        </div>
    }
}
