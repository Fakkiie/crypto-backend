use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenPrice {
    pub id: String,
    pub price: String,
}
