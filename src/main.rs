mod financial_api_client;

use std::collections::VecDeque;
use clap::{Parser, Subcommand};

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
        #[arg(short = 'w', long = "window-size", help = "The window size of the moving average")]
        window_size: usize,
        #[arg(short = 'd', long = "days", help = "The number of working days to analyse")]
        days: usize,
    }
}

pub struct SmaCalculator {
    prices: VecDeque<f32>,
    window_size: usize,
}

impl SmaCalculator {
    pub fn new(window_size: usize) -> Self {
        Self {
            prices: VecDeque::new(),
            window_size,
        }
    }

    pub fn add_price(&mut self, price: f32) {
        if self.prices.len() == self.window_size {
            self.prices.pop_front();
        }

        self.prices.push_back(price);
    }

    pub fn get_simple_moving_average(&self) -> Option<f32> {
        if self.prices.len() == self.window_size {
            Some(self.prices.iter().sum::<f32>() / self.window_size as f32)
        }
        else {
            None
        }
    }
}

struct SmaDataPoint {
    date: String,
    price: f32,
    sma: f32
}

impl SmaDataPoint {
    pub fn new(date: String, price: f32, sma: f32) -> Self {
        Self {
            date,
            price,
            sma,
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Sma { symbol, window_size, days } => {
            let client = financial_api_client::FinancialApiClient::new();

            match client.get_daily_price_data(&symbol).await {
                Ok(response) => {
                    println!("Data for symbol: {}", response.meta_data.symbol);

                    let mut dates: Vec<_> = response.time_series.iter().collect();
                    dates.sort_by(|a, b| b.0.cmp(a.0));

                    let mut calculator = SmaCalculator::new(window_size);
                    let mut data_points: Vec<SmaDataPoint> = Vec::new();

                    for (dates, prices) in dates.iter().take(days) {
                        let price = prices.close.parse::<f32>().unwrap();

                        calculator.add_price(price);
                        match calculator.get_simple_moving_average() {
                            Some(sma) => data_points.push(SmaDataPoint::new(
                                dates.to_string(),
                                price,
                                sma
                            )),
                            None => println!("Not enough prices yet for SMA calculation"),
                        }
                    }

                    println!("\nDate         | Price   | SMA     | Signal");
                    println!("---------------|---------|---------|---------");

                    for point in data_points {
                        let date = chrono::NaiveDate::parse_from_str(&point.date.to_string(), "%Y-%m-%d")
                            .unwrap()
                            .format("%a %d/%m/%Y")
                            .to_string();
                        let signal = if point.price > point.sma { "↑" } else { "↓" };
                        println!("{} | {:7.2} | {:7.2} | {}", date, point.price, point.sma, signal);
                    }
                },
                Err(e) => println!("Error fetching data: {}", e),
            }
        }
    }
}