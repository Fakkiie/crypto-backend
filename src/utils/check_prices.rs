use reqwest;
use serde_json::Value;

use std::sync::Arc;

use crate::custom_types::types::TokenPrice;

use crate::cornucopia;
use cornucopia::queries::limit_orders::get_limitOrders_tokens_of_interest;

pub async fn get_token_ids_of_interest(client: Arc<tokio_postgres::Client>) -> Vec<String> {
    let mut stmt = get_limitOrders_tokens_of_interest();
    let rows = stmt
        .bind(&*client)
        .all()
        .await
        .expect("Failed to execute query");

    let token_addresses = rows.iter().map(|row| row.clone()).collect::<Vec<String>>();
    token_addresses
}

pub async fn check_prices(client: Arc<tokio_postgres::Client>) -> Vec<TokenPrice> {
    let token_ids = get_token_ids_of_interest(client.clone()).await;
    if token_ids.is_empty() {
        return vec![];
    }

    let ids = token_ids.join(",");
    let url = format!("https://api.jup.ag/price/v2?ids={}", ids);

    let response = match reqwest::get(&url).await {
        Ok(res) => res,
        Err(err) => {
            eprintln!("Error fetching from API: {}", err);
            return vec![];
        }
    };

    let json: Value = match response.json().await {
        Ok(j) => j,
        Err(err) => {
            eprintln!("Error parsing JSON: {}", err);
            return vec![];
        }
    };

    let data = match json.get("data").and_then(|d| d.as_object()) {
        Some(d) => d,
        None => {
            eprintln!("Missing 'data' field in JSON response.");
            return vec![];
        }
    };

    token_ids
        .iter()
        .map(|id| {
            let price = data
                .get(id)
                .and_then(|token_data| token_data.get("price"))
                .and_then(|p| p.as_str())
                .unwrap_or("N/A")
                .to_string();

            TokenPrice {
                id: id.clone(),
                price,
            }
        })
        .collect()
}
