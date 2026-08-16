use leptos::*;

#[component]
pub fn Tabs(children: Children) -> impl IntoView {
    view! {
        <div class="space-y-4">
            {children()}
        </div>
    }
}

#[component]
pub fn TabList(children: Children) -> impl IntoView {
    view! {
        <div class="flex gap-2 border-b border-zinc-800 pb-2">
            {children()}
        </div>
    }
}

#[component]
pub fn Tab(
    #[prop(into)] value: String,
    #[prop(into)] label: String,
    active: bool,
    on_click: Callback<()>,
) -> impl IntoView {
    view! {
        <button
            class={if active {
                "px-4 py-2 text-sm font-medium text-emerald-400 border-b-2 border-emerald-400"
            } else {
                "px-4 py-2 text-sm font-medium text-zinc-400 hover:text-zinc-200"
            }}
            on:click=move |_| on_click.call(())
        >
            {label}
        </button>
    }
}

#[component]
pub fn TabPanel(
    active: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <div class={if active { "block" } else { "hidden" }}>
            {children()}
        </div>
    }
}
