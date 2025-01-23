use crate::custom_types::types::MatchingToken;
use base64;
use bincode;
use reqwest;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::{Decimal, MathematicalOps};
use serde_json::Value;

use solana_client::rpc_client;

use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::VersionedTransaction,
};

use std::error::Error;

pub async fn process_swap(quote_response: Value) -> Result<(), Box<dyn std::error::Error>> {
    let rpc_network_url: String =
        std::env::var("RPC_NETWORK_URL").expect("RPC_NETWORK_URL must be set");
    let rpc_network_key: String =
        std::env::var("RPC_NETWORK_KEY").expect("RPC_NETWORK_URL must be set");

    let endpoint = format!("{}{}", rpc_network_url, rpc_network_key);
    let connection = rpc_client::RpcClient::new(endpoint.to_string());

    let keypair =
        Keypair::from_base58_string(&std::env::var("PRIVATE_KEY").expect("Missing PRIVATE_KEY"));

    // Step 1: Send a POST request to Jupiter's /v6/swap endpoint
    let client = reqwest::Client::new();
    let public_key = keypair.pubkey().to_string();
    let swap_request_body = serde_json::json!({
        "quoteResponse": quote_response,
        "userPublicKey": public_key,
        "wrapAndUnwrapSol": true,
        "prioritizationFeeLamports": {
            "priorityLevelWithMaxLamports": {
                "maxLamports": 4000000,
                "global": false,
                "priorityLevel": "high"
            }
        }
    });

    let swap_response = client
        .post("https://quote-api.jup.ag/v6/swap")
        .header("Content-Type", "application/json")
        .body(swap_request_body.to_string())
        .send()
        .await?;

    if !swap_response.status().is_success() {
        return Err(format!("Swap request failed: {}", swap_response.text().await?).into());
    }

    let swap_data: Value = swap_response.json().await?;
    let swap_transaction_b64 = swap_data["swapTransaction"]
        .as_str()
        .ok_or("Missing swapTransaction field in swap response")?;
    let last_valid_block_height = swap_data["lastValidBlockHeight"]
        .as_u64()
        .ok_or("Missing lastValidBlockHeight field in swap response")?;

    // Step 2: Deserialize the transaction
    let swap_transaction_bytes = base64::decode(swap_transaction_b64)?;

    let transaction: VersionedTransaction = bincode::deserialize(&swap_transaction_bytes)
        .map_err(|err| format!("Failed to deserialize transaction: {}", err))?;

    // // Step 3: Sign the transaction
    let signed_transaction = VersionedTransaction::try_new(transaction.message, &[&keypair])?;

    // // Step 4: Simulate the transaction
    // let simulation_result = client.simulate_transaction(&transaction).await?;
    let simulation_result = connection.simulate_transaction(&signed_transaction)?;

    if let Some(err) = simulation_result.value.err {
        eprintln!("Simulation failed: {:?}", err);
        eprintln!("Logs: {:?}", simulation_result.value.logs);
        return Err("Transaction simulation failed".into());
    }

    println!("Broadcasting transaction");

    // // Step 5: Send the transaction and wait for confirmation details
    let tx_signature = connection.send_and_confirm_transaction_with_spinner(&signed_transaction)?;

    println!(
        "Transaction successful: {}. Valid until block height: {}",
        tx_signature, last_valid_block_height
    );

    Ok(())
}

pub async fn send_transaction(token: MatchingToken) -> Result<(), Box<dyn Error + Send + Sync>> {
    let base = Decimal::from(10);
    let multiplier = base.powi(token.sell_token_decimals as i64);
    let token_amount = token.sell_token_amount * multiplier;
    let token_amount_int: u64 = token_amount
        .to_u64()
        .expect("Failed to convert token_amount to integer");
    let url = format!(
        "https://quote-api.jup.ag/v6/quote?inputMint={}&outputMint={}&amount={}&slippageBps=50",
        token.sell_token_address, token.buy_token_address, token_amount_int
    );

    println!("Fetching quote from: {}", url);
    let response = reqwest::get(&url)
        .await
        .expect("Failed to fetch data from Jup Quote API");
    let quote_response: Value = response.json().await.expect("Failed to parse JSON");

    // Match on the JSON response
    match quote_response.as_object() {
        Some(obj) if obj.contains_key("error") => {
            let error_message = obj
                .get("error")
                .and_then(|val| val.as_str())
                .unwrap_or("Unknown error");
            let error_code = obj
                .get("errorCode")
                .and_then(|val| val.as_str())
                .unwrap_or("Unknown error code");

            return Err(format!(
                "API returned an error: {} (Error Code: {})",
                error_message, error_code
            )
            .into());

            // Return error here
        }
        Some(_) => {
            // Send swap request
            let response = process_swap(quote_response).await;
            match response {
                Ok(_) => {
                    println!("Swap request successful");
                }
                Err(err) => {
                    return Err(format!("Swap request failed: {:?}", err).into());
                }
            }
        }
        None => {
            return Err("Unexpected response format from quote API".into());
        }
    }
    Ok(())
}
