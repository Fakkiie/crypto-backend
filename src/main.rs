use reqwest; 
mod get_token_price;
use axum::{
    routing::{get, post},
    Router,
    Json, extract::Path,
};
use std::net::SocketAddr;
use get_token_price::get_token_price;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/token_price/:id", get(get_token_price))
        .route("/", get(|| async { "api test" }));
    
    // run server on port 8000
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Server running at http://{addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}
