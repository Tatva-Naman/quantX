use crate::strategy::Strategy;
use crate::data::{bar::Bar, order::Order};
use rand::Rng;
use chrono::Utc;

pub struct Market {
    pub last_close: f64,
}

impl Market {
    pub fn new(initial_price: f64) -> Self {
        Self { last_close: initial_price }
    }

    pub fn generate_bar(&mut self) -> Bar {
        let mut rng = rand::thread_rng();
        let open = self.last_close;
        let high = open + rng.gen_range(0.0..5.0);
        let low = open - rng.gen_range(0.0..5.0);
        let close = rng.gen_range(low..=high);
        let volume = rng.gen_range(100..20000);
        self.last_close = close;

        Bar {
            timestamp: Utc::now().to_rfc3339(),
            open,
            high,
            low, 
            close,
            volume,
        }
    }
}

pub fn run_simulation(strategy: &dyn Strategy, ticks: usize) -> Vec<Order> {
    let mut market = Market::new(100.0);
    let mut orders = Vec::new();

    let mut index =  1;
    for _ in 0..ticks {
        let bar = market.generate_bar();
        if let Some(order) = strategy.generate_signal(&bar) {
            orders.push(order);
        }
        println!("{} : {:?}", index, bar);
        index = index + 1;
    }
    orders
}