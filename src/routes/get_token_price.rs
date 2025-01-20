use axum::{extract::Path, Json};
use reqwest;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct TokenPrice {
    id: String,
    price: String,
}

pub async fn get_token_price(Path(id): Path<String>) -> Json<TokenPrice> {
    let url = format!("https://api.jup.ag/price/v2?ids={}", id);

    // Fetch the token price asynchronously.
    let response = reqwest::get(&url)
        .await
        .expect("Failed to fetch data from API");
    let json: Value = response.json().await.expect("Failed to parse JSON");

    let data = json["data"].as_object().expect("Missing data field");
    let price = if let Some(token_data) = data.get(&id) {
        token_data["price"].as_str().unwrap_or("N/A").to_string()
    } else {
        "No data found".to_string()
    };

    // Return the token price.
    Json(TokenPrice { id, price })
}
