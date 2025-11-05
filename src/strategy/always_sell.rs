use crate::data::{bar::Bar, order::{Order, OrderSide}};

use super::Strategy;

pub struct AlwaysSell;

impl Strategy for AlwaysSell {
    fn generate_signal(&self, bar: &Bar) -> Option<Order> {
        if bar.close < bar.open && bar.volume > 1000.0{
            Some(Order {
                side: OrderSide::Sell,
                price: bar.close,
                quantity: 10,
                timestamp: bar.timestamp.clone(),
            })
        }else{
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::bar::Bar;

    #[test]
    fn test_sell_on_bearish_bar() {
        let bar = Bar {
            timestamp: "2025-10-24T00:15:00Z".to_string(),
            open: 100.0,
            high: 102.0,
            low: 95.0,
            close: 97.0,
            volume: 13000.0,
        };
        let strategy = AlwaysSell;
        let order = strategy.generate_signal(&bar);
        assert!(order.is_some());
    }

    #[test]
    fn test_no_sell_on_bulish_bar() {
        let bar = Bar {
            timestamp: "2025-10-24T00:15:00Z".to_string(),
            open: 100.0,
            high: 105.0,
            low: 99.0,
            close: 104.0,
            volume: 23000.0,
        };
        let strategy = AlwaysSell;
        let order = strategy.generate_signal(&bar);
        assert!(order.is_none());
    }

    #[test]
    fn test_no_sell_on_low_volume() {
        let bar = Bar {
            timestamp: "2025-10-24T00:15:00Z".to_string(),
            open: 100.0,
            high: 102.0,
            low: 95.0,
            close: 97.0,
            volume: 3000.0,
        };
        let strategy = AlwaysSell;
        let order = strategy.generate_signal(&bar);
        assert!(order.is_none());
    }
}