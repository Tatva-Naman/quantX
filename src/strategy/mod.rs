pub mod always_buy;
pub mod always_sell;
pub mod ema_switch;

use crate::data::{bar::Bar, order::Order};

pub trait Strategy: Send + Sync{
    fn generate_signal(&self, bar: &Bar) -> Option<Order>;
}