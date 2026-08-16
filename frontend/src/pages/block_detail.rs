use leptos::*;
use leptos_router::*;
use crate::components::*;
use crate::pages::PageTitle;
use crate::hooks::use_api;
use shared::{Block, Transaction};

#[component]
pub fn BlockDetail() -> impl IntoView {
    let params = use_params_map();
    let height = move || {
        params.with(|p| p.get("height").cloned().unwrap_or_default())
            .parse::<i64>()
            .unwrap_or(0)
    };

    let block_url = move || format!("/api/v1/blocks/{}", height());
    let txs_url = move || format!("/api/v1/blocks/{}/txs", height());

    let (block, _block_loading, _block_error) = use_api::<Block>(&block_url());
    let (txs, _txs_loading, _txs_error) = use_api::<Vec<Transaction>>(&txs_url());

    view! {
        <div>
            <PageTitle title=move || format!("Block #{}", height())/>
            
            {move || block.get().map(|block| view! {
                <div class="space-y-6">
                    <Card>
                        <h2 class="text-lg font-semibold mb-4">"Block Details"</h2>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm">
                            <DetailRow label="Height" value=block.height.to_string()/>
                            <DetailRow label="Hash" value=block.hash/>
                            <DetailRow label="Previous Hash" value=block.prev_hash/>
                            <DetailRow label="Timestamp" value=block.timestamp.to_string()/>
                            <DetailRow label="Transactions" value=block.tx_count.to_string()/>
                            <DetailRow 
                                label="L1 Batch" 
                                value=block.l1_batch_number.map(|n| n.to_string()).unwrap_or_else(|| "-".to_string())
                            />
                        </div>
                    </Card>

                    <Card>
                        <h2 class="text-lg font-semibold mb-4">"Transactions"</h2>
                        <div class="space-y-2">
                            {txs.get().map(|txs| {
                                if txs.is_empty() {
                                    view! { <p class="text-zinc-500">"No transactions"</p> }.into_view()
                                } else {
                                    txs.into_iter().map(|tx| view! {
                                        <TransactionRow tx=tx/>
                                    }).collect::<Vec<_>>().into_view()
                                }
                            }).unwrap_or_else(|| view! { <Loading/> })}
                        </div>
                    </Card>
                </div>
            })}
        </div>
    }
}

#[component]
fn DetailRow(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
) -> impl IntoView {
    view! {
        <div>
            <p class="text-zinc-500">{label}</p>
            <p class="font-mono mt-1 break-all">{value}</p>
        </div>
    }
}

#[component]
fn TransactionRow(tx: Transaction) -> impl IntoView {
    view! {
        <A href=format!("/tx/{}", tx.hash) class="block">
            <div class="flex items-center justify-between p-3 hover:bg-zinc-800/50 rounded-lg transition-colors">
                <div class="min-w-0">
                    <p class="font-medium truncate">{tx.hash.clone()}</p>
                    <p class="text-xs text-zinc-500 truncate">
                        {"From: "} {tx.from_addr.clone()}
                    </p>
                </div>
                <div class="text-right ml-4">
                    <p class="text-sm">{tx.value.clone()}</p>
                </div>
            </div>
        </A>
    }
}
