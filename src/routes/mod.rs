pub mod auth;
pub mod stock_listing;
pub mod stock_details;
use actix_web::web;
pub mod trade_route;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(auth::configure_routes)
            .configure(stock_listing::configure_routes)
            .configure(stock_details::configure_routes)
            .configure(trade_route::configure_routes)

    );
}
