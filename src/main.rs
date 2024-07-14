use actix_web::{web, App, HttpServer, middleware::Logger};
use std::io;
use crate::routes::configure_routes;
use crate::db::mongo::init;
use env_logger::Env;
use sentry::ClientOptions;
use sentry_actix::Sentry;
use log::info;

mod routes;
mod db;
mod models;
mod services;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Initialize env_logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Initialize Sentry
    let _guard = sentry::init((
        "https://9887c9e734c125f670d197754fa7e2a2@o4507585839890432.ingest.de.sentry.io/4507585841463376",
        ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    info!("Starting the application.");

    // Initialize MongoDB
    let mongo_data = web::Data::new(init().await.expect("Failed to initialize MongoDB client"));

    // Start the Actix Web server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())  // Logger middleware
            .wrap(Sentry::new())  // Sentry middleware
            .app_data(mongo_data.clone())  // Share MongoDB data
            .configure(configure_routes)  // Configure routes
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
