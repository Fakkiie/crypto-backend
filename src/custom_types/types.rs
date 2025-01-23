use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenPrice {
    pub id: String,
    pub price: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MatchingToken {
    pub limit_order_id: String,
    pub wallet_address: String,
    pub buy_token_address: String,
    pub sell_token_address: String,
    pub sell_token_amount: Decimal,
    pub sell_token_decimals: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LimitOrder {
    pub limit_order_id: String,
    pub wallet_address: String,
    pub buy_token_address: String,
    pub sell_token_address: String,
    pub sell_token_amount: Decimal,
    pub sell_token_decimals: i32,
    pub token_value: Decimal,
    pub sell_type: String,
    pub limit_order_type: String,
    pub token_address_of_interest: String,
    pub order_status: String,
    pub created_at: PrimitiveDateTime,
}
