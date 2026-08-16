use leptos::*;
use leptos_router::*;
use crate::components::*;
use crate::pages::PageTitle;
use crate::hooks::{use_api, use_websocket};
use shared::{OrderBook, Trade};

#[component]
pub fn MarketDetail() -> impl IntoView {
    let params = use_params_map();
    let symbol = move || {
        params.with(|p| p.get("symbol").cloned().unwrap_or_else(|| "BTC-USD".to_string()))
    };

    let orderbook_url = move || format!("/api/v1/markets/{}/orderbook", symbol());
    let trades_url = move || format!("/api/v1/markets/{}/trades?limit=50", symbol());

    let (orderbook, _ob_loading, _ob_error) = use_api::<OrderBook>(&orderbook_url());
    let (trades, _trades_loading, _trades_error) = use_api::<Vec<Trade>>(&trades_url());

    // WebSocket连接获取实时数据
    let ws_url = "wss://ws.lighter.xyz";
    let (_ws_msg, _set_ws_msg) = use_websocket(ws_url);

    view! {
        <div>
            <PageTitle title=move || format!("{} Market", symbol())/>
            
            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                // 价格图表区域
                <div class="lg:col-span-2">
                    <Card>
                        <div class="h-96 flex items-center justify-center text-zinc-500">
                            "Price Chart (Coming Soon)"
                        </div>
                    </Card>
                </div>

                // 订单簿
                <div>
                    <Card>
                        <h3 class="font-semibold mb-4">"Order Book"</h3>
                        <div class="space-y-1">
                            {move || orderbook.get().map(|ob| view! {
                                <div class="grid grid-cols-2 gap-2 text-xs">
                                    <div class="text-zinc-500">"Bids"</div>
                                    <div class="text-zinc-500">"Asks"</div>
                                </div>
                                <div class="grid grid-cols-2 gap-2 text-sm">
                                    <div class="space-y-1">
                                        {ob.bids.into_iter().take(5).map(|level| view! {
                                            <div class="flex justify-between text-emerald-400">
                                                <span>{level.price}</span>
                                                <span>{level.amount}</span>
                                            </div>
                                        }).collect::<Vec<_>>()}
                                    </div>
                                    <div class="space-y-1">
                                        {ob.asks.into_iter().take(5).map(|level| view! {
                                            <div class="flex justify-between text-red-400">
                                                <span>{level.price}</span>
                                                <span>{level.amount}</span>
                                            </div>
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
                            })}
                        </div>
                    </Card>
                </div>

                // 最近成交
                <div class="lg:col-span-3">
                    <Card>
                        <h3 class="font-semibold mb-4">"Recent Trades"</h3>
                        <div class="overflow-x-auto">
                            <table class="w-full text-sm">
                                <thead class="bg-zinc-800/50">
                                    <tr>
                                        <th class="px-4 py-2 text-left">"Time"</th>
                                        <th class="px-4 py-2 text-left">"Price"</th>
                                        <th class="px-4 py-2 text-left">"Amount"</th>
                                        <th class="px-4 py-2 text-left">"Side"</th>
                                    </tr>
                                </thead>
                                <tbody class="divide-y divide-zinc-800">
                                    {move || trades.get().map(|trades| {
                                        trades.into_iter().map(|trade| view! {
                                            <tr class="hover:bg-zinc-800/30">
                                                <td class="px-4 py-2">{trade.timestamp.format("%H:%M:%S").to_string()}</td>
                                                <td class="px-4 py-2 font-mono">{trade.price}</td>
                                                <td class="px-4 py-2 font-mono">{trade.amount}</td>
                                                <td class="px-4 py-2">
                                                    <span class={if trade.side == "buy" { "text-emerald-400" } else { "text-red-400" }}>
                                                        {trade.side.to_uppercase()}
                                                    </span>
                                                </td>
                                            </tr>
                                        }).collect::<Vec<_>>()
                                    }).unwrap_or_default()}
                                </tbody>
                            </table>
                        </div>
                    </Card>
                </div>
            </div>
        </div>
    }
}
