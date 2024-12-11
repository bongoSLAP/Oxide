mod financial_api_client;
use clap::{Parser, Subcommand};
use std::str::FromStr;
use crate::financial_api_client::{FinancialApiClient, SmaData, TechnicalAnalysisResponse};

#[derive(Parser)]
#[command(name = "ox")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    Sma {
        #[arg(short = 's', long = "symbol", help = "The symbol of the stock to be analysed")]
        symbol: String,
        #[arg(short = 'i', long = "interval", help = "The interval (daily, weekly, monthly)")]
        interval: String,
        #[arg(short = 'p', long = "time-period", help = "The time period of the moving average")]
        time_period: usize
    }
}

async fn handle_sma(symbol: &str, interval: &str, time_period: usize) {
    let client = FinancialApiClient::new();
    match client.get_sma(symbol, &interval, time_period).await {
        Ok(response) => {
            display_analysis_summary(&response);
            display_sma_data(&response);
        },
        Err(e) => println!("Error fetching data: {}", e),
    }
}

fn display_analysis_summary(response: &TechnicalAnalysisResponse) {
    println!("\nTechnical Analysis Summary");
    println!("------------------------");
    println!("Symbol: {}", response.meta_data.symbol);
    println!("Indicator: {}", response.meta_data.indicator);
    println!("Interval: {}", response.meta_data.interval);
    println!("Time Period: {}", response.meta_data.time_period);
    println!("Series Type: {}", response.meta_data.series_type);
    println!("Last Refreshed: {}", response.meta_data.last_refreshed);
}

fn display_sma_data(response: &TechnicalAnalysisResponse) {
    let mut data_points: Vec<(&String, &SmaData)> = response.technical_analysis.iter().collect();
    data_points.sort_by(|a, b| b.0.cmp(a.0));

    println!("\nSMA Analysis");
    println!("-----------");
    println!("Date           | SMA Value");
    println!("---------------|----------");

    for (date, sma) in data_points.iter().take(20) {
        if let Ok(sma_value) = sma.sma.parse::<f64>() {
            println!("{} | {:>8.2}", date, sma_value);
        }
    }

    display_trend_analysis(&data_points);
}

fn display_trend_analysis(data_points: &[(&String, &SmaData)]) {
    if let (Some(last), Some(previous)) = (data_points.first(), data_points.get(1)) {
        if let (Ok(current), Ok(prev)) = (last.1.sma.parse::<f64>(), previous.1.sma.parse::<f64>()) {
            let change = current - prev;
            let change_percent = (change / prev) * 100.0;

            println!("\nRecent Movement");
            println!("--------------");
            println!("Last Change: {:+.2} ({:+.2}%)", change, change_percent);
            println!("Trend: {}", if change > 0.0 { "⬆ Upward" } else { "⬇ Downward" });
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Sma { symbol, interval, time_period } =>
            handle_sma(&symbol, &interval, time_period).await,
    }
}