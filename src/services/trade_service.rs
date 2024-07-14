use futures::TryStreamExt;
use mongodb::{bson::{doc, oid::ObjectId}, Client, Collection, Database};
use crate::models::trade_models::{TradeData, Trade, TradeStatus};
use crate::models::users::User;
use std::env;
use std::error::Error;
use log::{info, warn, error};
use sentry::capture_message;
use crate::db::mongo;


pub async fn get_database() -> Result<Database, String> {
    let client: Client = mongo::init().await.map_err(|e| e.to_string())?;
    Ok(client.database("trading_simulator"))
}

pub async fn create_trade(trade_data: &TradeData) -> Result<ObjectId, Box<dyn Error>> {
    let db = get_database().await?;
    let collection: Collection<Trade> = db.collection("trades");

    let user_id = ObjectId::parse_str(&trade_data.user_id)?;
    
    let new_trade = Trade {
        id: None,
        ticker: trade_data.ticker.clone(),
        position: trade_data.position.clone(),
        quantity: trade_data.quantity,
        price: trade_data.price,
        take_profit: trade_data.take_profit,
        stop_loss: trade_data.stop_loss,
        status: TradeStatus::InProgress,
        user_id,
        amount: trade_data.amount,
        trade_type: trade_data.trade_type.clone(),

    };

    let insert_result = collection.insert_one(new_trade, None).await?;
    let trade_id = insert_result.inserted_id.as_object_id().unwrap();
    
    info!("Trade created successfully: {:?}", trade_id);
    capture_message(&format!("Trade created successfully: {:?}", trade_id), sentry::Level::Info);

    Ok(trade_id)
}

pub async fn update_user_balance_and_trades(user_id: &str, trade_id: &ObjectId, amount: f64) -> Result<(), Box<dyn Error>> {
    let db = get_database().await?;
    let users_collection: Collection<User> = db.collection("users");

    let user_id = ObjectId::parse_str(user_id)?;

    let filter = doc! { "_id": user_id };
    let update = doc! {
        "$addToSet": { "trades": trade_id },
        "$inc": { "balance": -amount },
    };

    match users_collection.update_one(filter, update, None).await {
        Ok(_) => {
            info!("User balance and trades updated successfully for user: {:?}", user_id);
            capture_message(&format!("User balance and trades updated successfully for user: {:?}", user_id), sentry::Level::Info);
            Ok(())
        },
        Err(e) => {
            error!("Failed to update user balance and trades for user: {}. Error: {}", user_id, e);
            capture_message(&format!("Failed to update user balance and trades for user: {}. Error: {}", user_id, e), sentry::Level::Error);
            Err(Box::new(e))
        }
    }
}

pub async fn get_user_trades(user_id: &str) -> Result<Vec<Trade>, Box<dyn std::error::Error>> {
    let db = get_database().await?;
    let collection: Collection<Trade> = db.collection("trades");

    let user_id = ObjectId::parse_str(user_id)?;
    let filter = doc! { "user_id": user_id };

    let mut cursor = collection.find(filter, None).await?;
    let mut trades = Vec::new();

    while let Some(trade) = cursor.try_next().await? {
        trades.push(trade);
    }

    Ok(trades)
}
