use leptos::*;

#[component]
pub fn Loading() -> impl IntoView {
    view! {
        <div class="flex items-center justify-center py-8">
            <div class="w-8 h-8 border-2 border-zinc-700 border-t-emerald-500 rounded-full animate-spin"/>
        </div>
    }
}

#[component]
pub fn Skeleton(#[prop(into)] class: String) -> impl IntoView {
    view! {
        <div class={format!("bg-zinc-800 animate-pulse {}", class)}/>
    }
}
