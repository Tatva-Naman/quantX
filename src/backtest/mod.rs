use std::sync::Arc;

use crate::{data::{bar::Bar, order::{Order, OrderSide}}, strategy::{self, Strategy}};

pub struct Backtester<'a> {
    strategies: Vec<&'a dyn Strategy>,
    bars: Arc<Vec<Bar>>,
    orders: Vec<Order>,
    cash: f64,
    position: i64,
}

impl<'a > Backtester<'a > {
    pub fn new(strategies: Vec<&'a dyn Strategy>, bars: Arc<Vec<Bar>>, starting_cash: f64) -> Self {
        Self {
            strategies,
            bars,
            orders: Vec::new(),
            cash: starting_cash,
            position: 0,
        }
    }

    pub fn run(&mut self) {
        let bars = Arc::clone(&self.bars);
        let strategies_clone = self.strategies.clone();

        for bar in bars.iter() {
            println!("Processing Bar: {:?}", bar);
            for strategy in &strategies_clone {
                if let Some(order) = strategy.generate_signal(bar) {
                    self.execute_order(order);
                }
            }
        } 

        self.square_off();
    }

    pub fn execute_order(&mut self, order: Order) {
        match order.side {
            OrderSide::Buy => {
                self.cash -= order.price * order.quantity as f64;
                self.position += order.quantity;
            }
            OrderSide::Sell => {
                self.cash += order.price * order.quantity as f64;
                self.position -= order.quantity;
            }
        }
        self.orders.push(order);
    }

    fn square_off(&mut self) {
        if self.position != 0 {
            if let Some(last_bar) = self.bars.last() {
                let side = if self.position > 0 {
                    OrderSide::Sell
                } else {
                    OrderSide::Buy
                };

                let quantity = self.position.abs();

                let square_off_order = Order {
                    side,
                    price: last_bar.close,
                    quantity,
                    timestamp: last_bar.timestamp.clone(),
                };

                self.execute_order(square_off_order);
                println!("🔄 Auto Square Off executed for {} units", quantity);
            }
        }
    }

    pub fn summary(&self) {
        println!("--- Backtest Summary ---");
        println!("Final Cash: {:.2}", self.cash);
        println!("Final Position: {}", self.position);
        println!("Total Orders: {}", self.orders.len());
    }
    
    pub fn get_orders(&self) -> &Vec<Order> {
        &self.orders
    }
}