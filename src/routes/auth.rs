use actix_web::{web, HttpResponse, post};
use crate::services::auth_service::{login, register};
use crate::models::users::{AuthPayload, AuthResponse};
use sentry::capture_message;
use log::info;

#[post("/login")]
async fn login_route(form: web::Json<AuthPayload>) -> HttpResponse {
    let payload = form.into_inner();
    info!("Received login request for user: {}", payload.email);
    capture_message(&format!("Received login request for user: {}", payload.email), sentry::Level::Info);

    match login(payload).await {
        Ok(user) => {
            info!("Login successful");
            capture_message(&format!("Login successful "), sentry::Level::Info);
            HttpResponse::Ok().json(user)
        },
        Err(err) => {
            let error_message = format!("Login failed for user. Error: {:?}", err);
            capture_message(&error_message, sentry::Level::Error);
            HttpResponse::BadRequest().body(err)
        },
    }
}

#[post("/register")]
async fn register_route(form: web::Json<AuthPayload>) -> HttpResponse {
    let payload = form.into_inner();
    info!("Received registration request for user: {}", payload.email);
    capture_message(&format!("Received registration request for user: {}", payload.email), sentry::Level::Info);

    match register(payload).await {
        Ok(user) => {
            info!("Registration successful");
            capture_message(&format!("Registration successful"), sentry::Level::Info);
            HttpResponse::Ok().json(user)
        },
        Err(err) => {
            let error_message = format!("Registration failed for user. Error: {:?}", err);
            capture_message(&error_message, sentry::Level::Error);
            HttpResponse::BadRequest().body(err)
        },
    }
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login_route);
    cfg.service(register_route);
}
