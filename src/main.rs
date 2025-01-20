extern crate dotenv;
use dotenv::dotenv;

mod routes;
use create_buy_order::create_buy_order;
use create_sell_order::create_sell_order;
use get_token_balance::get_token_balance;
use get_token_price::get_token_price;
use routes::{create_buy_order, create_sell_order, get_token_balance, get_token_price};

use axum::{
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use rust_decimal::Decimal;

use std::net::SocketAddr;
use std::sync::Arc;
use tokio_postgres::NoTls;

use tower_http::{
    cors::{Any, CorsLayer},
    limit,
};

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TimestampSeconds};
use time::PrimitiveDateTime;

use uuid::Uuid;

mod cornucopia;
use cornucopia::queries::limit_orders::{
    delete_limitOrder as db_delete_limitOrder, get_all_limitOrders, get_limitOrder,
    get_limitOrders_by_walletAddress, insert_limitOrder, update_limitOrder,
};

#[derive(Serialize, Deserialize)]
struct LimitOrder {
    limit_order_id: String,
    wallet_address: String,
    buy_token_address: String,
    sell_token_address: String,
    sell_token_amount: Decimal,
    token_value: Decimal,
    sell_type: String,
    limit_order_type: String,
    token_address_of_interest: String,
    created_at: PrimitiveDateTime,
}

#[derive(Deserialize)]
struct AddLimitOrder {
    limit_order_id: String,
    wallet_address: String,
    buy_token_address: String,
    sell_token_address: String,
    sell_token_amount: Decimal,
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
    token_value: Decimal,
    sell_type: String,
    limit_order_type: String,
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
        .layer(Extension(client))
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
            token_value: row.tokenvalue,
            sell_type: row.selltype.clone(),
            limit_order_type: row.limitordertype.clone(),
            token_address_of_interest: row.tokenaddressofinterest.clone(),
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
            &payload.token_value,
            &payload.sell_type,
            &payload.limit_order_type,
            &payload.token_address_of_interest,
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
        token_value: row.tokenvalue,
        sell_type: row.selltype.clone(),
        limit_order_type: row.limitordertype.clone(),
        token_address_of_interest: row.tokenaddressofinterest.clone(),
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
        token_value: row.tokenvalue,
        sell_type: row.selltype.clone(),
        limit_order_type: row.limitordertype.clone(),
        token_address_of_interest: row.tokenaddressofinterest.clone(),
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
        token_value: row.tokenvalue,
        sell_type: row.selltype.clone(),
        limit_order_type: row.limitordertype.clone(),
        token_address_of_interest: row.tokenaddressofinterest.clone(),
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
        token_value: row.tokenvalue,
        sell_type: row.selltype.clone(),
        limit_order_type: row.limitordertype.clone(),
        token_address_of_interest: row.tokenaddressofinterest.clone(),
        created_at: row.createdat.clone(),
    };
    Json(limit_order)
}
