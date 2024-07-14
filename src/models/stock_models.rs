use serde::{Deserialize, Deserializer, Serialize};
use chrono::prelude::*;

#[derive(Deserialize)]
pub struct StockListingPayload {
    pub user_id: Option<String>,
    pub industry: Option<String>,
    pub sector: String,
    pub page: Option<u32>,
    pub items_per_page: Option<u32>, 
}

#[derive(Serialize)]
pub struct StockListingResponse {
    pub documents: Vec<StockData>,
}

#[derive(Serialize)]
pub struct StockData {
    pub name: String,
    pub ticker: String,
    pub price_data: Vec<PriceData>,
}

#[derive(Serialize)]
pub struct PriceData {
    pub date: String,
    pub price: f32,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct StockDetailsResponse {
    pub ticker: String,
    pub financials: Option<Financials>,
    pub key_statistics: Option<KeyStatistics>,
    pub price_data: Option<Vec<PriceDataDetails>>,
    pub profile: Option<Profile>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    address: Option<String>,
    city: Option<String>,
    state: Option<String>,
    country: Option<String>,
    industry: Option<String>,
    sector: Option<String>,
    #[serde(rename = "longBusinessSummary")]
    long_business_summary: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceDataDetails {
    period: String,
    interval: String,
    closes: Option<Vec<f64>>,
    highs: Option<Vec<f64>>,
    lows: Option<Vec<f64>>,
    opens: Option<Vec<f64>>,
    #[serde(deserialize_with = "deserialize_timestamps")]
    timestamps: Option<Vec<DateTime<Utc>>>,
    volumes: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Financials {
    beta: Option<Metric>,
    #[serde(rename = "dayHigh")]
    day_high: Option<Metric>,
    #[serde(rename = "dayLow")]
    day_low: Option<Metric>,
    #[serde(rename = "dividendRate")]
    dividend_rate: Option<Metric>,
    #[serde(rename = "dividendYield")]
    dividend_yield: Option<Metric>,
    #[serde(rename = "forwardPE")]
    forward_pe: Option<Metric>,
    #[serde(rename = "marketCap")]
    market_cap: Option<MarketCap>,
    open: Option<Metric>,
    #[serde(rename = "previousClose")]
    previous_close: Option<Metric>,
    #[serde(rename = "trailingPE")]
    trailing_pe: Option<Metric>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metric {
    fmt: Option<String>,
    raw: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketCap {
    fmt: Option<String>,
    raw: Option<f64>,
}

pub struct EarningsHistory {
    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyStatistics {
    #[serde(rename = "enterpriseValue")]
    enterprise_value: Option<EnterpriseValue>,
    #[serde(rename = "forwardEPS")]
    forward_eps: Option<Metric>,
    #[serde(rename = "pegRatio")]
    peg_ratio: Option<Metric>,
    #[serde(rename = "profitMargins")]
    profit_margins: Option<Metric>,
    #[serde(rename = "sharesOutstanding")]
    shares_outstanding: Option<SharesOutstanding>,
    #[serde(rename = "trailingEPS")]
    trailing_eps: Option<Metric>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnterpriseValue {
    fmt: Option<String>,
    raw: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SharesOutstanding {
    fmt: Option<String>,
    raw: Option<i64>,
}

fn deserialize_timestamps<'de, D>(deserializer: D) -> Result<Option<Vec<DateTime<Utc>>>, D::Error>
where
    D: Deserializer<'de>,
{
    let timestamps: Option<Vec<i64>> = Option::deserialize(deserializer)?;
    Ok(timestamps.map(|ts| ts.into_iter().map(|t| Utc.timestamp(t, 0)).collect()))
}