use actix_web::{error::{self, ErrorInternalServerError}, web, Error, HttpResponse};
use chrono::{DateTime, Utc, TimeZone};
use futures::stream::StreamExt;
use mongodb::{bson::{self, doc, from_document, Bson, Document}, options::FindOptions, Client};
use crate::models::stock_models::{Financials, KeyStatistics, PriceData, PriceDataDetails, Profile, StockData, StockDetailsResponse, StockListingPayload, StockListingResponse};
use log::{debug, error, info, warn};
use sentry::capture_message;

pub async fn stockList(
    payload: StockListingPayload, 
    client: web::Data<Client>
) -> Result<StockListingResponse, Error> {
    if payload.user_id.is_none() {
        warn!("Missing user ID in payload");
        capture_message("Missing user ID in payload", sentry::Level::Warning);
        return Err(error::ErrorBadRequest("Missing user ID"));
    }

    info!("Fetching stock data for industry: '{}', sector: '{}'", payload.industry.clone().unwrap_or_default(), payload.sector);
    capture_message(&format!("Fetching stock data for industry: '{}', sector: '{}'", payload.industry.clone().unwrap_or_default(), payload.sector), sentry::Level::Info);

    let companies_collection = client.database("trading_simulator").collection::<Document>("companies");
    let stock_data_collection = client.database("stock_data");

    let mut filter = doc! {
        "profile.sector": &payload.sector
    };
    if let Some(industry) = payload.industry {
        filter.insert("profile.industry", industry);
    }
    let find_options = FindOptions::builder()
        .skip(Some(payload.page.unwrap_or(0) as u64 * payload.items_per_page.unwrap_or(10) as u64))
        .limit(Some(payload.items_per_page.unwrap_or(10) as i64))
        .build();

    info!("Query Filter: {:?}", filter);
    capture_message(&format!("Query Filter: {:?}", filter), sentry::Level::Info);

    info!("Find Options: {:?}", find_options);
    capture_message(&format!("Find Options: {:?}", find_options), sentry::Level::Info);

    let mut cursor = companies_collection.find(filter, find_options).await.map_err(|err| {
        error!("Error querying companies collection: {}", err);
        capture_message(&format!("Error querying companies collection: {}", err), sentry::Level::Error);
        error::ErrorInternalServerError(err)
    })?;

    let mut documents = Vec::new();
    while let Some(result) = cursor.next().await {
        let doc = result.map_err(|err| {
            error!("Error fetching document from cursor: {}", err);
            capture_message(&format!("Error fetching document from cursor: {}", err), sentry::Level::Error);
            error::ErrorInternalServerError(err)
        })?;

        let ticker = doc.get_str("ticker").unwrap_or_default();
        info!("Processing ticker: {}", ticker);
        capture_message(&format!("Processing ticker: {}", ticker), sentry::Level::Info);

        let price_data_filter = doc! {"symbol": ticker, "period": "1mo", "interval": "1d"};
        let price_doc_option = stock_data_collection.collection::<Document>(ticker).find_one(price_data_filter, None).await.map_err(|err| {
            error!("Error fetching price data for ticker '{}': {}", ticker, err);
            capture_message(&format!("Error fetching price data for ticker '{}': {}", ticker, err), sentry::Level::Error);
            error::ErrorInternalServerError(err)
        })?;

        if let Some(price_doc) = price_doc_option {
            let closes: Vec<f32> = price_doc.get_array("closes")
                .unwrap_or(&Vec::new())
                .iter()
                .map(|b| b.as_f64().unwrap() as f32)
                .collect();

            let timestamps: Vec<DateTime<Utc>> = price_doc.get_array("timestamps")
                .unwrap_or(&Vec::new())
                .iter()
                .map(|b| {
                    let timestamp = b.as_i64().unwrap(); 
                    match Utc.timestamp_opt(timestamp, 0) {
                        chrono::LocalResult::Single(dt) => dt,
                        _ => Utc.timestamp(0, 0) 
                    }
                })
                .collect();

            let price_data: Vec<PriceData> = closes.into_iter().zip(timestamps.into_iter())
                .map(|(price, timestamp)| PriceData { 
                    date: timestamp.to_rfc3339(),
                    price 
                })
                .collect();

            documents.push(StockData {
                name: doc.get_str("name").unwrap_or_default().to_string(),
                ticker: ticker.to_string(),
                price_data,
            });
        }
    }

    Ok(StockListingResponse { documents })
}

pub async fn stock_details(client: web::Data<Client>, ticker: String) -> Result<StockDetailsResponse, Error> {
    let companies_collection = client.database("trading_simulator").collection::<Document>("companies");
    let stock_data_collection = client.database("stock_data").collection::<Document>(&ticker);

    let company_filter = doc! { "ticker": &ticker };
    let company_doc = companies_collection
        .find_one(Some(company_filter), None)
        .await
        .map_err(|err| {
            error!("Error querying company data for ticker {}: {}", &ticker, err);
            capture_message(&format!("Error querying company data for ticker {}: {}", &ticker, err), sentry::Level::Error);
            actix_web::error::ErrorInternalServerError("Database query failed for company data")
        })?;

    let mut cursor = stock_data_collection
        .find(None, None)
        .await
        .map_err(|err| {
            error!("Error querying stock price data for ticker {}: {}", &ticker, err);
            capture_message(&format!("Error querying stock price data for ticker {}: {}", &ticker, err), sentry::Level::Error);
            actix_web::error::ErrorInternalServerError("Database query failed for stock price data")
        })?;

    let mut price_data = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(doc) => {
                debug!("Price data document retrieved: {:?}", doc);
                match from_document::<PriceDataDetails>(doc.clone()) {
                    Ok(price_detail) => {
                        debug!("Parsed price data: {:?}", price_detail);
                        price_data.push(price_detail);
                    }
                    Err(err) => {
                        error!("Failed to parse price data for ticker {}: {:?}", &ticker, err);
                        capture_message(&format!("Failed to parse price data for ticker {}: {:?}", &ticker, err), sentry::Level::Error);
                        error!("Document content: {:?}", doc);
                        return Err(actix_web::error::ErrorInternalServerError("Data parsing failed for price data"));
                    }
                }
            }
            Err(err) => {
                error!("Cursor iteration failed for stock price data: {:?}", err);
                capture_message(&format!("Cursor iteration failed for stock price data: {:?}", err), sentry::Level::Error);
                return Err(actix_web::error::ErrorInternalServerError("Cursor failed during stock price data retrieval"));
            }
        }
    }

    let financials: Option<Financials> = company_doc
        .as_ref()
        .and_then(|doc| doc.get("financials").and_then(Bson::as_document))
        .cloned()
        .and_then(|doc| from_document(doc).ok());

    let key_statistics: Option<KeyStatistics> = company_doc
        .as_ref()
        .and_then(|doc| doc.get("keyStatistics").and_then(Bson::as_document))
        .cloned()
        .and_then(|doc| from_document(doc).ok());

    let profile: Option<Profile> = company_doc
        .as_ref()
        .and_then(|doc| doc.get("profile").and_then(Bson::as_document))
        .cloned()
        .and_then(|doc| from_document(doc).ok());

    let response = StockDetailsResponse {
        ticker: ticker.clone(),
        financials,
        key_statistics,
        price_data: Some(price_data),
        profile,
    };

    Ok(response)
}
