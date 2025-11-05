mod backtest;
mod data;
mod simulation;
mod strategy;

use std::sync::Arc;

use backtest::Backtester;
use data::{bar::Bar, downloader::download_binance_csv, loader::CsvLoader};
use simulation::run_simulation;
use strategy::{always_buy::AlwaysBuy, always_sell::AlwaysSell};

fn main() {
    match download_binance_csv("BTCUSDT", "1h") {
        Ok(path) => println!("Downloaded successfully: {}", path),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    let loader = CsvLoader::new("data/market_data/BTCUSDT-1h-2025-11-04.csv");
    let bars = Arc::new(loader.load().expect("Failed to load CSV data"));

    run_strategy_simulations(Arc::clone(&bars));
    run_backtest(Arc::clone(&bars));
}

fn run_strategy_simulations(bars: Arc<Vec<Bar>>) {
    let buy_strategy = AlwaysBuy;
    let sell_strategy = AlwaysSell;

    println!("Running Always Buy Simulation...");
    let buy_orders = run_simulation(&buy_strategy, Arc::clone(&bars));
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
