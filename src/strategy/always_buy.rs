use crate::data::{bar::Bar, order::{Order, OrderSide}};
use super::Strategy;

pub struct AlwaysBuy;

impl Strategy for AlwaysBuy {
    fn generate_signal(&self, bar: &Bar) -> Option<Order> {
        if bar.close > bar.open && bar.volume > 1000.0{
            Some(Order {
                side: OrderSide::Buy,
                price: bar.close,
                quantity: 1,
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
    fn test_buy_on_bullish_bar() {
        let bar = Bar {
            timestamp: "2025-10-24T00:15:00Z".to_string(),
            open: 100.0,
            high: 105.0,
            low: 99.0,
            close: 104.0,
            volume: 23000.0,
        };
        let strategy = AlwaysBuy;
        let order = strategy.generate_signal(&bar);
        assert!(order.is_some());
    }

    #[test]
    fn test_no_buy_on_bearish_bar() {
        let bar = Bar {
            timestamp: "2025-10-24T00:15:00Z".to_string(),
            open: 100.0,
            high: 102.0,
            low: 98.0,
            close: 99.0,
            volume: 9000.0,
        };
        let strategy = AlwaysBuy;
        let order = strategy.generate_signal(&bar);
        assert!(order.is_none());
    }

    #[test]
    fn test_no_buy_on_low_volume() {
        let bar = Bar {
            timestamp: "2025-10-24T00:15:00Z".to_string(),
            open: 100.0,
            high: 105.0,
            low: 99.0,
            close: 104.0,
            volume: 8700.0,
        };
        let strategy = AlwaysBuy;
        let order = strategy.generate_signal(&bar);
        assert!(order.is_none());
    }

}