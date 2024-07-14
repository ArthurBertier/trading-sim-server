use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Trade {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub ticker: String,
    pub position: String,
    pub quantity: u32,
    pub price: f64,
    pub take_profit: Option<f64>,
    pub stop_loss: Option<f64>,
    pub status: TradeStatus,
    pub user_id: ObjectId,
    pub amount: f64,
    pub trade_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TradeStatus {
    InProgress,
    Closed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeData {
    pub ticker: String,
    pub position: String,
    pub quantity: u32,
    pub price: f64,
    pub take_profit: Option<f64>,
    pub stop_loss: Option<f64>,
    pub user_id: String,
    pub amount: f64,
    pub trade_type: String,
    pub user_balance: f64,  // Add this field
}
