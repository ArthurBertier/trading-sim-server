use actix_web::{post, get, web, HttpResponse, Responder};
use serde::Deserialize;
use crate::services::trade_service::{create_trade, update_user_balance_and_trades, get_user_trades};
use crate::models::trade_models::TradeData;
use sentry::capture_message;
use log::{debug, error, info};
use serde_json::json;


#[derive(Deserialize)]
pub struct UserIdQuery {
    pub user_id: String,
}
#[post("/trade_submit")]
pub async fn submit_trade(trade_data: web::Json<TradeData>) -> impl Responder {
    info!("Received trade submission request: {:?}", trade_data);

    match create_trade(&trade_data).await {
        Ok(trade_id) => {
            if let Err(e) = update_user_balance_and_trades(&trade_data.user_id, &trade_id, trade_data.amount).await {
                error!("Failed to update user balance and trades: {}", e);
                return HttpResponse::InternalServerError().json(json!({
                    "error": "Failed to update user balance and trades"
                }));
            }

            HttpResponse::Ok().json(json!({
                "message": "Trade submitted successfully",
                "trade_id": trade_id.to_hex()
            }))
        },
        Err(e) => {
            error!("Failed to create trade: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to create trade"
            }))
        }
    }
}
#[get("/user_trades")]
pub async fn get_trades(query: web::Query<UserIdQuery>) -> HttpResponse {
    let user_id = &query.user_id;
    debug!("Received request to get trades for user_id: {}", user_id);

    match get_user_trades(user_id).await {
        Ok(trades) => {
            info!("Successfully fetched trades for user_id: {}", user_id);
            HttpResponse::Ok().json(trades)
        },
        Err(err) => {
            error!("Failed to fetch trades for user_id: {}: {}", user_id, err);
            HttpResponse::InternalServerError().body(format!("Failed to fetch trades: {}", err))
        }
    }
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(submit_trade);
    cfg.service(get_trades);
}
