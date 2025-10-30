mod data;
mod strategy;
mod simulation;
mod backtest;

use std::sync::Arc;

use backtest::Backtester;
use data::{bar::Bar, loader::CsvLoader};
use strategy::{always_buy::AlwaysBuy, always_sell::AlwaysSell};
use simulation::run_simulation;

fn main() {
    
    let loader = CsvLoader::new("data/historical_data.csv");
    let bars = Arc::new(loader.load().expect("Failed to load CSV data"));

    run_strategy_simulations(Arc::clone(&bars));
    run_backtest(Arc::clone(&bars));
}

fn run_strategy_simulations(bars: Arc<Vec<Bar>>) {

    let buy_strategy = AlwaysBuy;
    let sell_strategy = AlwaysSell;

    println!("Running Always Buy Simulation...");
    let buy_orders = run_simulation(&buy_strategy,  Arc::clone(&bars));
    for o in buy_orders {
        println!("{:?}", o);
    }

    println!("\nRunning Always Sell Simulation...");
    let sell_orders = run_simulation(&sell_strategy, Arc::clone(&bars));
    for o in sell_orders {
        println!("{:?}", o);
    }
}

fn run_backtest(bars: Arc<Vec<Bar>>) {
    
    let buy = AlwaysBuy;
    let sell = AlwaysSell;

    let mut backtester = Backtester::new(vec![&buy, &sell], Arc::clone(&bars), 10_000.0);

    println!("\n--- Running Combined Backtest ---");
    backtester.run();
    backtester.summary();
}
