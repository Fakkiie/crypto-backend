extern crate dotenv;
use dotenv::dotenv;

use axum::{
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use rust_decimal::Decimal;

use std::net::SocketAddr;
use std::sync::Arc;
use tokio_postgres::NoTls;

use tower_http::{
    cors::{Any, CorsLayer},
    limit,
};

use serde::{Deserialize, Serialize};
// Add this line to include the cornucopia module

use cornucopia::queries::limit_orders::insert_limitOrder;
use uuid::Uuid;

use crate::cornucopia;

#[derive(Serialize, Deserialize)]
pub struct LimitOrder {
    limit_order_id: String,
    wallet_address: String,
    buy_token_address: String,
    sell_token_address: String,
    sell_token_amount: Decimal,
    token_value: Decimal,
    sell_type: String,
    limit_order_type: String,
}

#[derive(Deserialize)]
pub struct AddLimitOrder {
    //  limit_order_id: String,
    wallet_address: String,
    buy_token_address: String,
    sell_token_address: String,
    sell_token_amount: Decimal,
    token_value: Decimal,
    sell_type: String,
    //  limit_order_type: String,
}

pub async fn create_buy_order(
    Extension(client): Extension<Arc<tokio_postgres::Client>>,
    Json(payload): Json<AddLimitOrder>,
) -> Json<LimitOrder> {
    let mut stmt = insert_limitOrder();
    let row = stmt
        .bind(
            &*client,
            &Uuid::new_v4().to_string(),
            &payload.wallet_address,
            &payload.buy_token_address,
            &payload.sell_token_address,
            &payload.sell_token_amount,
            &payload.token_value,
            &payload.sell_type,
            &String::from("buy"),
        )
        .one()
        .await
        .expect("Failed to execute query");
    let limit_order = LimitOrder {
        limit_order_id: row.limitorderid.clone(),
        wallet_address: row.walletaddress.clone(),
        buy_token_address: row.buytokenaddress.clone(),
        sell_token_address: row.selltokenaddress.clone(),
        sell_token_amount: row.selltokenamount,
        token_value: row.tokenvalue,
        sell_type: row.selltype.clone(),
        limit_order_type: row.limitordertype.clone(),
    };
    Json(limit_order)
}
