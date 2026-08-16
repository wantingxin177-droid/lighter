use leptos::*;
use leptos_router::*;
use shared::Block;
use crate::components::Card;

#[component]
pub fn BlockCard(block: Block) -> impl IntoView {
    view! {
        <A href=format!("/block/{}", block.height) class="block">
            <Card>
                <div class="flex items-center justify-between">
                    <div>
                        <p class="font-bold text-lg">{"Block #"} {block.height}</p>
                        <p class="text-sm text-zinc-500 font-mono truncate max-w-[200px]">
                            {block.hash.clone()}
                        </p>
                    </div>
                    <div class="text-right">
                        <p class="text-sm">{block.tx_count} {" transactions"}</p>
                        <p class="text-xs text-zinc-500">
                            {block.timestamp.format("%Y-%m-%d %H:%M").to_string()}
                        </p>
                    </div>
                </div>
            </Card>
        </A>
    }
}
