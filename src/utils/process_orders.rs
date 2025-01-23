use serde::{Deserialize, Serialize};

use std::sync::Arc;

use axum::Extension;

use crate::custom_types::types::TokenPrice;

use rust_decimal::Decimal;
use time::PrimitiveDateTime;

use crate::cornucopia;
use cornucopia::queries::limit_orders::{get_all_open_limitOrders, pend_limitOrder};

#[derive(Serialize, Deserialize, Debug)]
struct LimitOrder {
    limit_order_id: String,
    wallet_address: String,
    buy_token_address: String,
    sell_token_address: String,
    sell_token_amount: Decimal,
    sell_token_decimals: i32,
    token_value: Decimal,
    sell_type: String,
    limit_order_type: String,
    token_address_of_interest: String,
    order_status: String,
    created_at: PrimitiveDateTime,
}

async fn get_limit_orders(
    Extension(client): Extension<Arc<tokio_postgres::Client>>,
) -> Vec<LimitOrder> {
    let mut stmt = get_all_open_limitOrders();
    let rows = stmt
        .bind(&*client)
        .all()
        .await
        .expect("Failed to execute query");
    let limit_orders = rows
        .iter()
        .map(|row| LimitOrder {
            limit_order_id: row.limitorderid.clone(),
            wallet_address: row.walletaddress.clone(),
            buy_token_address: row.buytokenaddress.clone(),
            sell_token_address: row.selltokenaddress.clone(),
            sell_token_amount: row.selltokenamount,
            sell_token_decimals: row.selltokendecimals,
            token_value: row.tokenvalue,
            sell_type: row.selltype.clone(),
            limit_order_type: row.limitordertype.clone(),
            token_address_of_interest: row.tokenaddressofinterest.clone(),
            order_status: row.orderstatus.clone(),
            created_at: row.createdat.clone(),
        })
        .collect();
    limit_orders
}

async fn set_limit_order_pending(
    client: Extension<Arc<tokio_postgres::Client>>,
    limit_order_id: String,
) -> LimitOrder {
    let mut stmt = pend_limitOrder();
    let row = stmt
        .bind(&**client, &limit_order_id)
        .one()
        .await
        .expect("Failed to execute query");

    let limit_order = LimitOrder {
        limit_order_id: row.limitorderid.clone(),
        wallet_address: row.walletaddress.clone(),
        buy_token_address: row.buytokenaddress.clone(),
        sell_token_address: row.selltokenaddress.clone(),
        sell_token_amount: row.selltokenamount,
        sell_token_decimals: row.selltokendecimals,
        token_value: row.tokenvalue,
        sell_type: row.selltype.clone(),
        limit_order_type: row.limitordertype.clone(),
        token_address_of_interest: row.tokenaddressofinterest.clone(),
        order_status: row.orderstatus.clone(),
        created_at: row.createdat.clone(),
    };
    limit_order
}

pub async fn process_orders(
    client: Extension<Arc<tokio_postgres::Client>>,
    token_prices: Vec<TokenPrice>,
    tx_queue: tokio::sync::mpsc::Sender<crate::custom_types::types::MatchingToken>,
) {
    let limit_orders = get_limit_orders(client.clone()).await;
    let token_price_map: std::collections::HashMap<&str, Decimal> = token_prices
        .iter()
        .filter_map(|tp| {
            tp.price
                .parse::<Decimal>()
                .ok()
                .map(|price| (tp.id.as_str(), price))
        })
        .collect();

    for limit_order in limit_orders {
        // println!(
        //     "Processing order: ID={} Wallet={} Token={} Price={} Type={} SellType={}",
        //     limit_order.limit_order_id,
        //     limit_order.wallet_address,
        //     limit_order.token_address_of_interest,
        //     limit_order.token_value,
        //     limit_order.limit_order_type,
        //     limit_order.sell_type
        // );
        if let Some(&current_price) =
            token_price_map.get(limit_order.token_address_of_interest.as_str())
        {
            // println!("Current price for token {}, Desired Price for token {}", current_price, limit_order.token_value);
            let matched = match limit_order.limit_order_type.as_str() {
                "buy" => {
                    // println!("Checking Buy Order: Current Price={} Limit Price={}", current_price, limit_order.token_value);

                    if current_price <= limit_order.token_value {
                        // println!("Matched Buy Order: ID={} Wallet={} Price={}", limit_order.limit_order_id, limit_order.wallet_address, current_price);
                        true
                    } else {
                        // println!("Buy Order Did Not Match: Current Price={} > Limit Price={}", current_price, limit_order.token_value);
                        false
                    }
                }
                "sell" => {
                    // println!("Checking Sell Order: Sell Type={} Current Price={} Limit Price={}", limit_order.sell_type, current_price, limit_order.token_value);
                    match limit_order.sell_type.as_str() {
                        "greater" => {
                            if current_price >= limit_order.token_value {
                                // println!("Matched Sell Greater Order: ID={} Wallet={} Price={}", limit_order.limit_order_id, limit_order.wallet_address, current_price);
                                true
                            } else {
                                // println!("Sell Greater Order Did Not Match: Current Price={} < Limit Price={}", current_price, limit_order.token_value);
                                false
                            }
                        }
                        "lesser" => {
                            if current_price <= limit_order.token_value {
                                // println!("Matched Sell Lesser Order: ID={} Wallet={} Price={}", limit_order.limit_order_id, limit_order.wallet_address, current_price);
                                true
                            } else {
                                // println!("Sell Lesser Order Did Not Match: Current Price={} > Limit Price={}", current_price, limit_order.token_value);
                                false
                            }
                        }
                        _ => {
                            // println!("Unknown Sell Type: ID={} Wallet={} SellType={}", limit_order.limit_order_id, limit_order.wallet_address, limit_order.sell_type);
                            false
                        }
                    }
                }
                _ => {
                    // println!("Unknown Order Type: ID={} Wallet={} Type={}", limit_order.limit_order_id, limit_order.wallet_address, limit_order.limit_order_type);
                    false
                }
            };
            if matched {
                println!(
                    "Order {} has been matched and queued for processing.",
                    limit_order.limit_order_id
                );

                let matching_token = crate::custom_types::types::MatchingToken {
                    limit_order_id: limit_order.limit_order_id.clone(),
                    wallet_address: limit_order.wallet_address.clone(),
                    buy_token_address: limit_order.buy_token_address.clone(),
                    sell_token_address: limit_order.sell_token_address.clone(),
                    sell_token_amount: limit_order.sell_token_amount,
                    sell_token_decimals: limit_order.sell_token_decimals,
                };

                if let Err(e) = tx_queue.send(matching_token).await {
                    eprintln!("Failed to queue transaction: {}", e);
                    // Handle transaction error out (send tokens back to user)
                } else {
                    set_limit_order_pending(client.clone(), limit_order.limit_order_id).await;
                }
            }
        } else {
            println!(
                "No price found for token: {}",
                limit_order.token_address_of_interest
            );
        }
    }
}
