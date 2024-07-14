use actix_web::{web, post, HttpResponse, Error};
use mongodb::Client;
use serde::Deserialize;
use sentry::capture_message;
use log::info;

use crate::services::stock_service::stock_details;
use crate::models::stock_models::StockDetailsResponse;

#[derive(Deserialize)]
pub struct StockQuery {
    detail_level: Option<String>,
}

#[post("/stock-details/{ticker}")]
async fn stock_list_route(
    data: web::Data<Client>,
    path: web::Path<String>, 
    query: web::Query<StockQuery>
) -> Result<HttpResponse, Error> {
    let ticker = path.into_inner();

    info!("Received request for stock details: {}", ticker);
    capture_message(&format!("Received request for stock details: {}", ticker), sentry::Level::Info);
    let ticker_ref = &ticker;
    match stock_details(data, ticker_ref.clone()).await {
        Ok(stock_details) => {
            info!("Successfully retrieved stock details for: {}", ticker_ref);
            capture_message(&format!("Successfully retrieved stock details for: {}", ticker_ref), sentry::Level::Info);
            Ok(HttpResponse::Ok().json(stock_details))
        },
        Err(err) => {
            let error_message = format!("Failed to retrieve stock details for: {}. Error: {:?}", ticker, err);
            capture_message(&error_message, sentry::Level::Error);
            Err(err)
        },
    }
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(stock_list_route);
}
