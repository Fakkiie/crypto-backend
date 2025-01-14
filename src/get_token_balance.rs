use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use solana_client::client_error::ClientError;
use solana_client::rpc_client;
use solana_sdk::pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address;
use std::env;
use std::str::FromStr;

#[derive(Serialize)]
pub struct TokenBalance {
    balance: f64,
    token_mint_address: String,
}

#[derive(Deserialize)]
pub struct GetTokenBalanceRequest {
    token_mint_address: String,
    wallet_address: String,
}

pub async fn get_token_balance(
    Json(payload): Json<GetTokenBalanceRequest>,
) -> Result<Json<TokenBalance>, (StatusCode, &'static str)> {
    let rpc_network_url: String = env::var("RPC_NETWORK_URL").unwrap();
    let rpc_network_key: String = env::var("RPC_NETWORK_KEY").unwrap();

    println!("RPC_NETWORK_URL: {}", rpc_network_url);
    println!("RPC_NETWORK_KEY: {}", rpc_network_key);

    if payload.wallet_address.is_empty() || payload.token_mint_address.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Wallet address or token mint address is empty",
        ));
    }

    // Corrected to use debug print for struct
    let endpoint = format!("{}{}", rpc_network_url, rpc_network_key);
    let connection = rpc_client::RpcClient::new(endpoint.to_string());

    let wallet_address = Pubkey::from_str(&payload.wallet_address).unwrap();

    if payload.token_mint_address == "So11111111111111111111111111111111111111112" {
        let balance_in_lamports = connection.get_balance(&wallet_address).unwrap();
        return Ok(Json(TokenBalance {
            balance: balance_in_lamports as f64,
            token_mint_address: payload.token_mint_address.clone(),
        }));
    }

    let token_mint_address = Pubkey::from_str(&payload.token_mint_address).unwrap();

    let associated_token_address =
        get_associated_token_address(&wallet_address, &token_mint_address);
    let balance = match connection.get_token_account_balance(&associated_token_address) {
        Ok(balance) => balance,
        Err(e) => {
            if e.to_string().contains("RPC response error -32602") {
                println!("Zero balance");
                return Ok(Json(TokenBalance {
                    balance: 0.0,
                    token_mint_address: payload.token_mint_address.clone(),
                }));
            } else {
                println!("Unexpected error: {}", e);
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Failed to get token account balance",
                ));
            }
        }
    };

    Ok(Json(TokenBalance {
        balance: balance.decimals as f64,
        token_mint_address: "Hello!".to_string(),
    }))
}
