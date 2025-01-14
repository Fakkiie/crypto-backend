extern crate dotenv;
use dotenv::dotenv;

mod get_token_balance;
use get_token_balance::get_token_balance;

mod get_token_price;
use get_token_price::get_token_price;

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/token_price/:id", get(get_token_price))
        .route("/token_balance", post(get_token_balance))
        .route("/", get(|| async { "api test" }))
        .layer(cors);

    // run server on port 8000
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Server running at http://{addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
