use actix_web::{post, web, HttpResponse};
use mongodb::Client;
use crate::services::stock_service::stockList;
use crate::models::stock_models::StockListingPayload;
use sentry::capture_message;
use log::info;

#[post("/stock-list")]
async fn stock_list_route(data: web::Data<Client>, form: web::Json<StockListingPayload>) -> HttpResponse {
    let payload = form.into_inner();
    info!("Received stock list request with payload: {:?}", payload.industry);
    capture_message(&format!("Received stock list request with payload: {:?}", payload.industry), sentry::Level::Info);

    match stockList(payload, data).await {
        Ok(stock_list) => {
            info!("Successfully retrieved stock list.");
            capture_message("Successfully retrieved stock list.", sentry::Level::Info);
            HttpResponse::Ok().json(stock_list)
        },
        Err(err) => {
            let error_message = format!("Failed to retrieve stock list. Error: {:?}", err);
            capture_message(&error_message, sentry::Level::Error);
            HttpResponse::BadRequest().body(err.to_string())
        },
    }
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(stock_list_route);
    //cfg.service(performinStock_route);
}
