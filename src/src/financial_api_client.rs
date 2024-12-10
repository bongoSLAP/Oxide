use reqwest::Client;
use std::env;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MetaData {
    #[serde(rename = "1. Information")]
    pub information: String,
    #[serde(rename = "2. Symbol")]
    pub(crate) symbol: String,
    #[serde(rename = "3. Last Refreshed")]
    last_refreshed: String,
}

#[derive(Debug, Deserialize)]
pub struct DailyPrices {
    #[serde(rename = "1. open")]
    pub open: String,
    #[serde(rename = "2. high")]
    pub high: String,
    #[serde(rename = "3. low")]
    pub low: String,
    #[serde(rename = "4. close")]
    pub close: String,
    #[serde(rename = "5. volume")]
    pub volume: String,
}

#[derive(Debug, Deserialize)]
pub struct AlphaVantageResponse {
    #[serde(rename = "Meta Data")]
    pub(crate) meta_data: MetaData,
    #[serde(rename = "Time Series (Daily)")]
    pub time_series: std::collections::HashMap<String, DailyPrices>,
}

pub struct FinancialApiClient {
    api_key: String,
    client: Client,
    base_url: &'static str,
}

impl FinancialApiClient {
    pub fn new() -> Self {
        Self {
            api_key: env::var("ALPHA_VANTAGE_API_KEY").expect("ALPHA_VANTAGE_API_KEY must be set"),
            client: Client::new(),
            base_url: "https://www.alphavantage.co/query",
        }
    }

    pub async fn get_daily_price_data(&self, symbol: &str) -> Result<AlphaVantageResponse, reqwest::Error> {
        let url = format!(
            "{}?function=TIME_SERIES_DAILY&symbol={}&apikey={}",
            self.base_url,
            symbol,
            self.api_key
        );

        let response = self.client
            .get(&url)
            .send()
            .await?
            .json::<AlphaVantageResponse>()
            .await?;

        Ok(response)
    }
}