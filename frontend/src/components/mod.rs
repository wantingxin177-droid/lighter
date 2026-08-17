use leptos::*;

// 重新导出所有组件
mod block_card;
mod loading;
mod tabs;
mod tx_list;

pub use block_card::*;
pub use loading::*;
pub use tabs::*;
pub use tx_list::*;

// 通用UI组件

#[component]
pub fn Button(
    #[prop(into)] on_click: Callback<()>,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            class=format!("px-4 py-2 bg-emerald-600 hover:bg-emerald-700 text-white rounded-lg transition-colors {}", class)
            on:click=move |_| on_click.call(())
        >
            {children()}
        </button>
    }
}

#[component]
pub fn Card(children: Children) -> impl IntoView {
    view! {
        <div class="bg-zinc-900 border border-zinc-800 rounded-lg p-4">
            {children()}
        </div>
    }
}

#[component]
pub fn Badge(
    #[prop(into)] text: String,
    #[prop(optional)] color: String,
) -> impl IntoView {
    let color_class = match color.as_str() {
        "green" => "bg-emerald-500/20 text-emerald-400",
        "red" => "bg-red-500/20 text-red-400",
        "blue" => "bg-blue-500/20 text-blue-400",
        "yellow" => "bg-yellow-500/20 text-yellow-400",
        _ => "bg-zinc-700 text-zinc-300",
    };
    
    view! {
        <span class=format!("px-2 py-0.5 rounded text-xs {}", color_class)>
            {text}
        </span>
    }
}

#[component]
pub fn DataTable(
    #[prop(into)] headers: Vec<String>,
    rows: Vec<Vec<String>>,
) -> impl IntoView {
    view! {
        <div class="overflow-x-auto">
            <table class="w-full text-sm">
                <thead class="bg-zinc-800/50">
                    <tr>
                        {headers.into_iter().map(|h| view! {
                            <th class="px-4 py-2 text-left text-zinc-400 font-medium">{h}</th>
                        }).collect::<Vec<_>>()}
                    </tr>
                </thead>
                <tbody class="divide-y divide-zinc-800">
                    {rows.into_iter().map(|row| view! {
                        <tr class="hover:bg-zinc-800/30">
                            {row.into_iter().map(|cell| view! {
                                <td class="px-4 py-3">{cell}</td>
                            }).collect::<Vec<_>>()}
                        </tr>
                    }).collect::<Vec<_>>()}
                </tbody>
            </table>
        </div>
    }
}
