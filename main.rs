use reqwest; 
use serde_json::Value; 

fn get_token_price(id: &str) {
    let url = format!("https://api.jup.ag/price/v2?ids={}", id);

    let response = reqwest::blocking::get(&url)
        .expect("Failed to fetch data from API")
        .json::<Value>()
        .expect("Failed to parse JSON");

    let data = response["data"].as_object().expect("Missing data field");
    if let Some(token_data) = data.get(id) {
        let price = token_data["price"].as_str().unwrap_or("N/A");
        println!("Token ID: {}, Price: {}", id, price);
    } else {
        println!("No data found for token ID: {}", id);
    }
}

fn main() {
    let id = "So11111111111111111111111111111111111111112";
    get_token_price(id);
}
