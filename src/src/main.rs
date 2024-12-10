mod financial_api_client;

use std::collections::VecDeque;

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

#[tokio::main]
async fn main() {
    let client = financial_api_client::FinancialApiClient::new();
    let window_size = 5;
    let series_size = 30;

    match client.get_daily_price_data("IONQ").await {
        Ok(response) => {
            println!("Data for symbol: {}", response.meta_data.symbol);

            let mut dates: Vec<_> = response.time_series.iter().collect();
            dates.sort_by(|a, b| b.0.cmp(a.0));

            let mut prices_for_sma: Vec<f32> = Vec::new();
            for (_, prices) in dates.iter().take(series_size) {
                prices_for_sma.push(prices.close.parse::<f32>().unwrap());
            }

            let mut calculator = SmaCalculator::new(window_size);

            for price in prices_for_sma {
                calculator.add_price(price);
                match calculator.get_simple_moving_average() {
                    Some(sma) => println!("SMA after adding {}: {}", price, sma),
                    None => println!("Not enough prices yet for SMA calculation"),
                }
            }
        },
        Err(e) => println!("Error fetching data: {}", e),
    }
}