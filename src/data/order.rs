#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub side: OrderSide,
    pub price: f64,
    pub quantity: i64,
    pub timestamp: String,
}