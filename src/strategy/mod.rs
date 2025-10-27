pub mod always_buy;
pub mod always_sell;

use crate::models::order::Order;
use crate::data::bar::Bar;

pub trait Strategy {
    fn generate_order(&self, bar: &Bar) -> Option<Order>;
}