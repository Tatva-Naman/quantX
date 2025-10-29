mod data;
mod strategy;
mod simulation;
mod backtest;

use std::sync::Arc;

use backtest::Backtester;
use data::simulator::DataSimulator;
use strategy::{always_buy::AlwaysBuy, always_sell::AlwaysSell};
use simulation::run_simulation;

fn main() {
    run_strategy_simulations();
    run_backtest();
}


fn run_strategy_simulations() {
    let buy_strategy = AlwaysBuy;
    let sell_strategy = AlwaysSell;

    println!("Running Always Buy Simulation...");
    let buy_orders = run_simulation(&buy_strategy, 10);
    for o in buy_orders {
        println!("{:?}", o);
    }

    println!("\nRunning Always Sell Simulation...");
    let sell_orders = run_simulation(&sell_strategy, 10);
    for o in sell_orders {
        println!("{:?}", o);
    }
}


fn run_backtest() {
    let simulator = DataSimulator::new(50, 100.0); 
    let bars = Arc::new(simulator.generate());

    let buy = AlwaysBuy;
    let sell = AlwaysSell;

    let mut backtester = Backtester::new(vec![&buy, &sell], Arc::clone(&bars), 10_000.0);

    println!("\n--- Running Combined Backtest ---");
    backtester.run();
    backtester.summary();
}
