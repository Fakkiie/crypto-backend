use crate::cornucopia::queries::limit_orders::complete_limitOrder;
use crate::custom_types::types::MatchingToken;
use crate::utils::send_transaction::send_transaction;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::mpsc;

async fn complete_limit_order(
    client: Arc<tokio_postgres::Client>,
    limit_order_id: String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut stmt = complete_limitOrder();

    // Attempt to execute the query and handle potential errors
    stmt.bind(&*client, &limit_order_id)
        .one()
        .await
        .map_err(|e| format!("Failed to complete limit order {}: {}", limit_order_id, e))?;

    println!(
        "Limit order {} successfully completed in the database.",
        limit_order_id
    );
    Ok(())
}

pub async fn transaction_processor(
    mut rx_queue: mpsc::Receiver<MatchingToken>,
    client: Arc<tokio_postgres::Client>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    while let Some(matching_token) = rx_queue.recv().await {
        println!(
            "Processing transaction for order: {}",
            matching_token.limit_order_id.clone()
        );

        // Call `send_transaction` and handle the result
        match send_transaction(matching_token.clone()).await {
            Ok(_) => {
                println!(
                    "Transaction successfully processed for order: {}",
                    matching_token.limit_order_id.clone()
                );

                if let Err(err) =
                    complete_limit_order(client.clone(), matching_token.limit_order_id.clone())
                        .await
                {
                    eprintln!(
                        "Failed to close limit order {}: {:?}",
                        matching_token.limit_order_id.clone(),
                        err
                    );
                } else {
                    println!(
                        "Limit order {} successfully closed.",
                        matching_token.limit_order_id
                    );
                }
            }
            Err(err) => {
                eprintln!(
                    "Failed to process transaction for order {}: {:?}",
                    matching_token.limit_order_id.clone(),
                    err
                );
            }
        }
    }

    Ok(())
}
