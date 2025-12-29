use std::sync::Arc;

use crate::strategy::Strategy;
use crate::data::{bar::Bar, order::Order};

pub struct Market {
    pub last_close: f64,
}

impl Market {
    pub fn new(initial_price: f64) -> Self {
        Self { last_close: initial_price }
    }
}

pub fn run_simulation(strategy: &dyn Strategy, bars: Arc<Vec<Bar>>) -> Vec<Order> {
    let mut orders = Vec::new();

    let mut index =  1;
    let bars = Arc::clone(&bars);
    for bar in bars.iter() {
        if let Some(order) = strategy.generate_signal(bar) {
            orders.push(order);
        }
        println!("{} : {:?}", index, bar);
        index = index + 1;
    }

    orders
}