mod data;
mod models;
mod strategy;
mod simulation;

use strategy::{always_buy::AlwaysBuy, always_sell::AlwaysSell};
use simulation::run_simulation;


fn main() {
    
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