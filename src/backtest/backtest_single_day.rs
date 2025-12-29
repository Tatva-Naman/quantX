use std::sync::Arc;

use crate::data::bar::Bar;
use crate::data::downloader::download_and_extract_for_date;
use crate::data::loader::CsvLoader;
use crate::data::order::OrderSide;
use crate::strategy::Strategy;
use chrono::{Duration, Utc};
use tokio::task::JoinHandle;

#[derive(Debug)]
pub struct DailyResult {
    pub date: String,
    pub pnl: f64,
    pub trades: usize,
}

/// Single-day backtest (EOD square-off)
pub fn backtest_single_day(
    strategies: &[Arc<dyn Strategy>],
    bars: &[Bar],
    date: &str,
) -> DailyResult {
    let mut cash = 10_00000.0;
    let mut position: i64 = 0;
    let mut trades = 0usize;

    for bar in bars {
        for strat in strategies {
            if let Some(order) = strat.generate_signal(bar) {
                trades += 1;
                match order.side {
                    OrderSide::Buy => {
                        if cash > order.price {
                            cash -= order.price * order.quantity as f64;
                            position += order.quantity;
                        }
                    }
                    OrderSide::Sell => {
                        cash += order.price * order.quantity as f64;
                        position -= order.quantity;
                    }
                }
            }
        }
    }

    // EOD square-off using last bar
    if position != 0 {
        if let Some(last) = bars.last() {
            // if position>0 we sell, else buy to cover
            if position > 0 {
                cash += last.close * (position.abs() as f64);
            } else {
                cash -= last.close * (position.abs() as f64);
            }
            position = 0;
        }
    }

    let pnl = cash - 10_00000.0;
    print!("Date: {}, PnL: {:.2}, Trades: {}\n", date, pnl, trades);
    DailyResult {
        date: date.to_string(),
        pnl,
        trades,
    }
}
