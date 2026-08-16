use leptos::*;
use leptos_router::*;
use crate::components::*;
use crate::pages::{PageTitle, SectionTitle};
use crate::hooks::{use_api, use_websocket};
use shared::{Block, Transaction};

#[component]
pub fn Explorer() -> impl IntoView {
    let (blocks, _blocks_loading, _blocks_error) = use_api::<Vec<Block>>("/api/v1/blocks/recent?limit=20");
    let (txs, _txs_loading, _txs_error) = use_api::<Vec<Transaction>>("/api/v1/txs/recent?limit=20");

    view! {
        <div>
            <PageTitle title="Block Explorer".to_string()/>
            
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                // 最新区块
                <div>
                    <SectionTitle title="Latest Blocks".to_string()/>
                    <Card>
                        <div class="space-y-2">
                            {move || blocks.get().map(|blocks| {
                                blocks.into_iter().map(|block| view! {
                                    <BlockRow block=block/>
                                }).collect::<Vec<_>>()
                            }).unwrap_or_default()}
                        </div>
                    </Card>
                </div>

                // 最新交易
                <div>
                    <SectionTitle title="Latest Transactions".to_string()/>
                    <Card>
                        <div class="space-y-2">
                            {move || txs.get().map(|txs| {
                                txs.into_iter().map(|tx| view! {
                                    <TransactionRow tx=tx/>
                                }).collect::<Vec<_>>()
                            }).unwrap_or_default()}
                        </div>
                    </Card>
                </div>
            </div>
        </div>
    }
}

#[component]
fn BlockRow(block: Block) -> impl IntoView {
    view! {
        <A href=format!("/block/{}", block.height) class="block">
            <div class="flex items-center justify-between p-3 hover:bg-zinc-800/50 rounded-lg transition-colors">
                <div>
                    <p class="font-medium">{"Block #"} {block.height}</p>
                    <p class="text-xs text-zinc-500">{block.hash.clone()}</p>
                </div>
                <div class="text-right">
                    <p class="text-sm">{block.tx_count} {" txs"}</p>
                    <p class="text-xs text-zinc-500">
                        {block.timestamp.format("%Y-%m-%d %H:%M:%S").to_string()}
                    </p>
                </div>
            </div>
        </A>
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
                    <p class="text-xs text-zinc-500">
                        {"Block #"} {tx.block_height}
                    </p>
                </div>
            </div>
        </A>
    }
}
