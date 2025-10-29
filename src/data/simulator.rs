use rand::Rng;
use crate::data::bar::Bar;

pub struct DataSimulator {
    pub num_bars: usize,
    pub start_price: f64,
}

impl DataSimulator {
    pub fn new(num_bars: usize, start_price: f64) -> Self {
        Self { num_bars, start_price }
    }

    pub fn generate(&self) -> Vec<Bar> {
        let mut bars = Vec::new();
        let mut price = self.start_price;
        let mut rng = rand::thread_rng();

        for i in 0..self.num_bars {
            let change = rng.gen_range(-0.02..0.02);
            let new_price = price * (1.0 + change);

            let high = new_price * (1.0 + rng.gen_range(0.0..0.01));
            let low = new_price * (1.0 - rng.gen_range(0.0..0.01));
            let volume = rng.gen_range(5000..20000);

            bars.push(Bar {
                timestamp: format!("2025-10-24T{:02}:00:00Z", i),
                open: price,
                high,
                low,
                close: new_price,
                volume,
            });
            println!("Generated Bar {}: Open: {:.2}, High: {:.2}, Low: {:.2}, Close: {:.2}, Volume: {}", 
                     i + 1, price, high, low, new_price, volume);
            price = new_price;

        }
        bars
    }
}
