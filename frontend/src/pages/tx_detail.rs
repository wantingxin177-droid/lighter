use leptos::*;
use leptos_router::*;
use crate::components::*;
use crate::pages::PageTitle;
use crate::hooks::use_api;
use shared::Transaction;

#[component]
pub fn TransactionDetail() -> impl IntoView {
    let params = use_params_map();
    let hash = move || {
        params.with(|p| p.get("hash").cloned().unwrap_or_default())
    };

    let url = move || format!("/api/v1/txs/{}", hash());
    let (tx, _loading, _error) = use_api::<Transaction>(&url());

    view! {
        <div>
            <PageTitle title="Transaction Details".to_string()/>
            
            {move || tx.get().map(|tx| view! {
                <Card>
                    <div class="space-y-4">
                        <DetailRow label="Hash" value=tx.hash/>
                        <DetailRow label="Block" value=format!("#{}", tx.block_height)/>
                        <DetailRow label="From" value=tx.from_addr/>
                        <DetailRow label="To" value=tx.to_addr/>
                        <DetailRow label="Value" value=tx.value/>
                        <DetailRow label="Gas Price" value=tx.gas_price/>
                        <DetailRow label="Gas Used" value=tx.gas_used.to_string()/>
                        <DetailRow label="Status" value=tx.status/>
                        <DetailRow label="Timestamp" value=tx.timestamp.to_string()/>
                    </div>
                </Card>
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
        <div class="border-b border-zinc-800 pb-4 last:border-0">
            <p class="text-sm text-zinc-500">{label}</p>
            <p class="font-mono mt-1 break-all">{value}</p>
        </div>
    }
}
