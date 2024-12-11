use reqwest::Client;
use std::env;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TechnicalAnalysisResponse {
    #[serde(rename = "Meta Data")]
    pub meta_data: MetaData,

    #[serde(rename = "Technical Analysis: SMA")]
    pub technical_analysis: HashMap<String, SmaData>
}

#[derive(Debug, Deserialize)]
pub struct MetaData {
    #[serde(rename = "1: Symbol")]
    pub symbol: String,

    #[serde(rename = "2: Indicator")]
    pub indicator: String,

    #[serde(rename = "3: Last Refreshed")]
    pub last_refreshed: String,

    #[serde(rename = "4: Interval")]
    pub interval: String,

    #[serde(rename = "5: Time Period")]
    pub time_period: u32,

    #[serde(rename = "6: Series Type")]
    pub series_type: String,

    #[serde(rename = "7: Time Zone")]
    pub time_zone: String,
}

#[derive(Debug, Deserialize)]
pub struct SmaData {
    #[serde(rename = "SMA")]
    pub sma: String,
}

pub struct FinancialApiClient {
    api_key: String,
    client: Client,
    base_url: &'static str,
}

impl FinancialApiClient {
    pub fn new() -> Self {
        Self {
            api_key: env::var("ALPHA_VANTAGE_API_KEY")
                .expect("ALPHA_VANTAGE_API_KEY must be set"),
            client: Client::new(),
            base_url: "https://www.alphavantage.co/query",
        }
    }

    pub async fn get_sma(
        &self,
        symbol: &str,
        interval: &str,
        time_period: usize
    ) -> Result<TechnicalAnalysisResponse, reqwest::Error> {
        let url = format!(
            "{}?function=SMA&symbol={}&interval={}&time_period={}&series_type=close&apikey={}",
            self.base_url,
            symbol,
            interval,
            time_period,
            self.api_key
        );

        let response = self.client
            .get(&url)
            .send()
            .await?
            .json::<TechnicalAnalysisResponse>()
            .await?;

        Ok(response)
    }
}