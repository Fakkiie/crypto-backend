extern crate dotenv;
use dotenv::dotenv;

mod routes;
use create_buy_order::create_buy_order;
use create_sell_order::create_sell_order;
use get_token_balance::get_token_balance;
use get_token_price::get_token_price;
use routes::{create_buy_order, create_sell_order, get_token_balance, get_token_price};

mod utils;
use check_prices::check_prices;
use process_orders::process_orders;
use transaction_processor::transaction_processor;
use utils::{check_prices, process_orders, transaction_processor};

mod custom_types;
use custom_types::types::LimitOrder;

use axum::{
    routing::{get, post},
    Extension, Json, Router,
};
use rust_decimal::Decimal;

use std::net::SocketAddr;
use std::sync::Arc;
use tokio_postgres::NoTls;

use tower_http::cors::{Any, CorsLayer};

use serde::Deserialize;

mod cornucopia;
use cornucopia::queries::limit_orders::{
    delete_limitOrder as db_delete_limitOrder, get_all_limitOrders, get_limitOrder,
    insert_limitOrder, update_limitOrder,
};

// #[derive(Serialize, Deserialize)]
// struct LimitOrder {
//     limit_order_id: String,
//     wallet_address: String,
//     buy_token_address: String,
//     sell_token_address: String,
//     sell_token_amount: Decimal,
//     sell_token_decimals: i32,
//     token_value: Decimal,
//     sell_type: String,
//     limit_order_type: String,
//     token_address_of_interest: String,
//     order_status: String,
//     created_at: PrimitiveDateTime,
// }

#[derive(Deserialize)]
struct AddLimitOrder {
    limit_order_id: String,
    wallet_address: String,
    buy_token_address: String,
    sell_token_address: String,
    sell_token_amount: Decimal,
    sell_token_decimals: i32,
    token_value: Decimal,
    sell_type: String,
    limit_order_type: String,
    token_address_of_interest: String,
}
#[derive(Deserialize)]
struct UpdateLimitOrder {
    limit_order_id: String,
    wallet_address: String,
    buy_token_address: String,
    sell_token_address: String,
    sell_token_amount: Decimal,
    sell_token_decimals: i32,
    token_value: Decimal,
    sell_type: String,
    limit_order_type: String,
    order_status: String,
    token_address_of_interest: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let (client, connection) = tokio_postgres::connect(
        &std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        NoTls,
    )
    .await
    .expect("Failed to connect to database");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let client = Arc::new(client);

    // Create an MPSC channel with a buffer size of 100 for transactions
    let (tx_queue, rx_queue) =
        tokio::sync::mpsc::channel::<crate::custom_types::types::MatchingToken>(100);

    // Spawn the transaction processor task
    let client_clone_2 = client.clone();
    tokio::spawn(async move {
        if let Err(err) = transaction_processor(rx_queue, client_clone_2.clone()).await {
            eprintln!("Transaction processor error: {:?}", err);
        }
    });

    // Call check_prices and print the results
    let client_clone = client.clone();
    tokio::spawn(async move {
        loop {
            let token_prices = check_prices(client_clone.clone()).await;
            println!("Token Prices: {:?}", token_prices);

            // Process orders using the fetched token prices
            process_orders(
                Extension(client_clone.clone()),
                token_prices,
                tx_queue.clone(),
            )
            .await;

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // Check prices every second
        }
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/token_price/:id", get(get_token_price))
        .route("/token_balance", post(get_token_balance))
        .route(
            "/get_limitOrders_by_walletAddress/:id",
            get(get_limit_orders),
        )
        .route("/get_limitOrder/:id", get(get_single_limit_order))
        .route("/add_limitOrder", post(add_limit_order))
        .route("/create_sell_order", post(create_sell_order))
        .route("/create_buy_order", post(create_buy_order))
        .route("/", get(|| async { "api test" }))
        .layer(Extension(client.clone()))
        .layer(cors);

    // run server on port 8080
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Server running at http://{addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_limit_orders(
    Extension(client): Extension<Arc<tokio_postgres::Client>>,
) -> Json<Vec<LimitOrder>> {
    let mut stmt = get_all_limitOrders();
    let rows = stmt
        .bind(&*client)
        .all()
        .await
        .expect("Failed to execute query");
    let limit_orders = rows
        .iter()
        .map(|row| LimitOrder {
            limit_order_id: row.limitorderid.clone(),
            wallet_address: row.walletaddress.clone(),
            buy_token_address: row.buytokenaddress.clone(),
            sell_token_address: row.selltokenaddress.clone(),
            sell_token_amount: row.selltokenamount,
            sell_token_decimals: row.selltokendecimals,
            token_value: row.tokenvalue,
            sell_type: row.selltype.clone(),
            limit_order_type: row.limitordertype.clone(),
            token_address_of_interest: row.tokenaddressofinterest.clone(),
            order_status: row.orderstatus.clone(),
            created_at: row.createdat.clone(),
        })
        .collect();
    Json(limit_orders)
}

async fn add_limit_order(
    Extension(client): Extension<Arc<tokio_postgres::Client>>,
    Json(payload): Json<AddLimitOrder>,
) -> Json<LimitOrder> {
    let mut stmt = insert_limitOrder();
    let row = stmt
        .bind(
            &*client,
            // &Uuid::new_v4().to_string(),
            &payload.limit_order_id,
            &payload.wallet_address,
            &payload.buy_token_address,
            &payload.sell_token_address,
            &payload.sell_token_amount,
            &payload.sell_token_decimals,
            &payload.token_value,
            &payload.sell_type,
            &payload.limit_order_type,
            &payload.token_address_of_interest,
            &String::from("open"),
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
        sell_token_decimals: row.selltokendecimals,
        token_value: row.tokenvalue,
        sell_type: row.selltype.clone(),
        limit_order_type: row.limitordertype.clone(),
        token_address_of_interest: row.tokenaddressofinterest.clone(),
        order_status: row.orderstatus.clone(),
        created_at: row.createdat.clone(),
    };
    Json(limit_order)
}

async fn get_single_limit_order(
    Extension(client): Extension<Arc<tokio_postgres::Client>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Json<LimitOrder> {
    let mut stmt = get_limitOrder();
    let row = stmt
        .bind(&*client, &id)
        .one()
        .await
        .expect("Failed to execute query");
    let limit_order = LimitOrder {
        limit_order_id: row.limitorderid.clone(),
        wallet_address: row.walletaddress.clone(),
        buy_token_address: row.buytokenaddress.clone(),
        sell_token_address: row.selltokenaddress.clone(),
        sell_token_amount: row.selltokenamount,
        sell_token_decimals: row.selltokendecimals,
        token_value: row.tokenvalue,
        sell_type: row.selltype.clone(),
        limit_order_type: row.limitordertype.clone(),
        token_address_of_interest: row.tokenaddressofinterest.clone(),
        order_status: row.orderstatus.clone(),
        created_at: row.createdat.clone(),
    };
    Json(limit_order)
}

async fn delete_limit_order(
    Extension(client): Extension<Arc<tokio_postgres::Client>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Json<LimitOrder> {
    let mut stmt = db_delete_limitOrder();
    let row = stmt
        .bind(&*client, &id)
        .one()
        .await
        .expect("Failed to execute query");
    let limit_order = LimitOrder {
        limit_order_id: row.limitorderid.clone(),
        wallet_address: row.walletaddress.clone(),
        buy_token_address: row.buytokenaddress.clone(),
        sell_token_address: row.selltokenaddress.clone(),
        sell_token_amount: row.selltokenamount,
        sell_token_decimals: row.selltokendecimals,
        token_value: row.tokenvalue,
        sell_type: row.selltype.clone(),
        limit_order_type: row.limitordertype.clone(),
        token_address_of_interest: row.tokenaddressofinterest.clone(),
        order_status: row.orderstatus.clone(),
        created_at: row.createdat.clone(),
    };
    Json(limit_order)
}

async fn edit_limit_order(
    Extension(client): Extension<Arc<tokio_postgres::Client>>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(payload): Json<UpdateLimitOrder>,
) -> Json<LimitOrder> {
    let mut stmt = update_limitOrder();
    let row = stmt
        .bind(
            &*client,
            &payload.buy_token_address,
            &payload.sell_token_address,
            &payload.sell_token_amount,
            &payload.token_value,
            &payload.sell_type,
            &payload.limit_order_type,
            &payload.token_address_of_interest,
            &payload.order_status,
            &id,
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
        sell_token_decimals: row.selltokendecimals,
        token_value: row.tokenvalue,
        sell_type: row.selltype.clone(),
        limit_order_type: row.limitordertype.clone(),
        token_address_of_interest: row.tokenaddressofinterest.clone(),
        order_status: row.orderstatus.clone(),
        created_at: row.createdat.clone(),
    };
    Json(limit_order)
}
