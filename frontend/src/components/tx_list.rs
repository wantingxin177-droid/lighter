use leptos::*;
use leptos_router::*;
use shared::Transaction;
use crate::components::Card;

#[component]
pub fn TransactionList(txs: Vec<Transaction>) -> impl IntoView {
    view! {
        <div class="space-y-2">
            {txs.into_iter().map(|tx| view! {
                <TransactionItem tx=tx/>
            }).collect::<Vec<_>>()}
        </div>
    }
}

#[component]
fn TransactionItem(tx: Transaction) -> impl IntoView {
    view! {
        <A href=format!("/tx/{}", tx.hash) class="block">
            <div class="flex items-center justify-between p-3 hover:bg-zinc-800/50 rounded-lg transition-colors border border-zinc-800">
                <div class="min-w-0 flex-1">
                    <p class="font-medium truncate">{tx.hash.clone()}</p>
                    <div class="flex gap-4 text-xs text-zinc-500 mt-1">
                        <span class="truncate">{"From: "} {tx.from_addr.clone()}</span>
                        <span class="truncate">{"To: "} {tx.to_addr.clone()}</span>
                    </div>
                </div>
                <div class="text-right ml-4 shrink-0">
                    <p class="font-mono">{tx.value.clone()}</p>
                    <p class="text-xs text-zinc-500">{"Block #"} {tx.block_height}</p>
                </div>
            </div>
        </A>
    }
}
