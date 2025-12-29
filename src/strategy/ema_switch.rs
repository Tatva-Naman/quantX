use crate::data::{
    bar::Bar,
    order::{Order, OrderSide},
};

pub struct EmaSwitchStrategy {
    ema_short_period: usize,
    ema_long_period: usize,
    ema_short: Vec<f64>,
    ema_long: Vec<f64>,
    current_trend: Option<OrderSide>,
    position: i64, // +1 for long, -1 for short, 0 for flat
}

impl EmaSwitchStrategy {
    pub fn new(short: usize, long: usize) -> Self {
        Self {
            ema_short_period: short,
            ema_long_period: long,
            ema_short: Vec::new(),
            ema_long: Vec::new(),
            current_trend: None,
            position: 0,
        }
    }

    /// Pure helper: doesn't borrow `self`. Computes EMA over `prices` slice using `period`.
    fn calc_ema_from_slice(prices: &[f64], period: usize) -> f64 {
        // Simple iterative EMA with the first value of the slice used as the seed.
        // prices.len() should be >= period when called.
        let alpha = 2.0 / (period as f64 + 1.0);
        let mut ema = prices[0];
        for &p in &prices[1..] {
            ema = alpha * p + (1.0 - alpha) * ema;
        }
        ema
    }

    /// Returns 0..n orders for this bar (0 when no crossover)
    pub fn generate_signal(&mut self, bar: &Bar) -> Vec<Order> {
        let mut orders = Vec::new();

        // MUTATE self first (mutable borrow only here)
        self.ema_short.push(bar.close);
        self.ema_long.push(bar.close);

        // If we don't have long-period history yet, skip
        if self.ema_long.len() < self.ema_long_period {
            return orders;
        }

        // Prepare slices to pass into pure helper (no borrowing of self required by helper)
        let short_len = self.ema_short.len();
        let long_len = self.ema_long.len();

        // safe slice indices (we checked lengths above)
        let short_slice_start = short_len.saturating_sub(self.ema_short_period);
        let long_slice_start = long_len.saturating_sub(self.ema_long_period);

        let short_slice = &self.ema_short[short_slice_start..short_len];
        let long_slice = &self.ema_long[long_slice_start..long_len];

        // Calculate EMAs using the pure helper (no &self borrow)
        let short_ema = Self::calc_ema_from_slice(short_slice, self.ema_short_period);
        let long_ema = Self::calc_ema_from_slice(long_slice, self.ema_long_period);

        // Determine new trend
        let new_trend = if short_ema > long_ema {
            Some(OrderSide::Buy)
        } else if short_ema < long_ema {
            Some(OrderSide::Sell)
        } else {
            self.current_trend.clone()
        };

        // If trend flipped, generate square-off + open orders
        if new_trend != self.current_trend {
            match (&self.current_trend, &new_trend) {
                (Some(OrderSide::Buy), Some(OrderSide::Sell)) => {
                    // close long
                    if self.position > 0 {
                        orders.push(Order {
                            side: OrderSide::Sell,
                            price: bar.close,
                            quantity: self.position.abs().max(1),
                            timestamp: bar.timestamp.clone(),
                        });
                    }
                    // open short
                    orders.push(Order {
                        side: OrderSide::Sell,
                        price: bar.close,
                        quantity: 1,
                        timestamp: bar.timestamp.clone(),
                    });
                    self.position = -1;
                }
                (Some(OrderSide::Sell), Some(OrderSide::Buy)) => {
                    // close short
                    if self.position < 0 {
                        orders.push(Order {
                            side: OrderSide::Buy,
                            price: bar.close,
                            quantity: self.position.abs().max(1),
                            timestamp: bar.timestamp.clone(),
                        });
                    }
                    // open long
                    orders.push(Order {
                        side: OrderSide::Buy,
                        price: bar.close,
                        quantity: 1,
                        timestamp: bar.timestamp.clone(),
                    });
                    self.position = 1;
                }
                (None, Some(OrderSide::Buy)) => {
                    orders.push(Order {
                        side: OrderSide::Buy,
                        price: bar.close,
                        quantity: 1,
                        timestamp: bar.timestamp.clone(),
                    });
                    self.position = 1;
                }
                (None, Some(OrderSide::Sell)) => {
                    orders.push(Order {
                        side: OrderSide::Sell,
                        price: bar.close,
                        quantity: 1,
                        timestamp: bar.timestamp.clone(),
                    });
                    self.position = -1;
                }
                _ => {}
            }

            self.current_trend = new_trend;
        }

        orders
    }
}
