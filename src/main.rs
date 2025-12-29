mod backtest;
mod data;
mod simulation;
mod strategy;

use chrono::{Duration, Utc};
use std::sync::Arc;
use tokio::task::JoinHandle;

use backtest::{backtest_single_day, backtest_ema_crossover};
use data::{bar::Bar, downloader::download_and_extract_for_date, loader::CsvLoader};
use simulation::run_simulation;
use strategy::{always_buy::AlwaysBuy, always_sell::AlwaysSell};

#[tokio::main]
async fn main() {
    // run_sync_backtest().await;
    run_continous_backtest().await;
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

async fn run_sync_backtest() {
    println!("Starting async daily EOD backtest (batched concurrency = 5)");

    let symbol = "BTCUSDT";
    let interval = "1h";
    let days = 365 * 2;
    let concurrency = 30usize;

    let buy = Arc::new(AlwaysBuy);
    let sell = Arc::new(AlwaysSell);
    let strategies: Vec<Arc<dyn strategy::Strategy>> = vec![buy.clone(), sell.clone()];

    let mut results: Vec<backtest_single_day::DailyResult> = Vec::new();

    let sem = Arc::new(tokio::sync::Semaphore::new(concurrency));
    let mut handles: Vec<JoinHandle<Option<backtest_single_day::DailyResult>>> = Vec::new();

    for days_ago in 1..=days {
        let permit = Arc::clone(&sem).acquire_owned().await.unwrap();
        let s = symbol.to_string();
        let i = interval.to_string();
        let date = (Utc::now() - Duration::days(days_ago as i64))
            .format("%Y-%m-%d")
            .to_string();

        let strategies_refs = strategies.clone();

        let handle = tokio::spawn(async move {
            let _permit = permit;
            if let Ok(csv_path) = download_and_extract_for_date(&s, &i, &date).await {
                let csv_path_for_load = csv_path.clone();
                if let Ok(Ok(bars)) = tokio::task::spawn_blocking(move || {
                    let loader = CsvLoader::new(&csv_path_for_load);
                    loader.load()
                })
                .await
                {
                    let result =
                        backtest_single_day::backtest_single_day(&strategies_refs, &bars, &date);

                    let _ = tokio::fs::remove_file(&csv_path).await;
                    let zip_path = format!("data/market_data/{}-{}-{}.zip", s, i, date);
                    let _ = tokio::fs::remove_file(zip_path).await;
                    Some(result)
                } else {
                    None
                }
            } else {
                None
            }
        });
        handles.push(handle);
    }

    for h in handles {
        if let Ok(opt) = h.await {
            if let Some(dr) = opt {
                results.push(dr);
            }
        }
    }

    let total_days = results.len();
    let total_pnl: f64 = results.iter().map(|r| r.pnl).sum();
    let wins = results.iter().filter(|r| r.pnl > 0.0).count();
    let losses = results.iter().filter(|r| r.pnl < 0.0).count();
    let total_trades: usize = results.iter().map(|r| r.trades).sum();

    println!("\n=== Backtest Summary (aggregated) ===");
    println!("Days processed: {}", total_days);
    println!("Winning days: {}", wins);
    println!("Losing days: {}", losses);
    println!("Total PnL: {:.4}", total_pnl);
    println!("Total trades: {}", total_trades);
}

async fn run_continous_backtest() {
    let symbol = "BTCUSDT";
    let interval = "1h";
    let days = 365 * 2 ;
    let concurrency = 30usize;

    println!("🚀 Downloading ~{} days (≈2 years) of {} {} data...", days, symbol, interval);

    let sem = Arc::new(tokio::sync::Semaphore::new(concurrency));
    let mut handles = Vec::new();

    for days_ago in 1..=days {
        let permit = Arc::clone(&sem).acquire_owned().await.unwrap();
        let s = symbol.to_string();
        let i = interval.to_string();
        let date = (Utc::now() - Duration::days(days_ago as i64))
            .format("%Y-%m-%d")
            .to_string();

        let handle = tokio::spawn(async move {
            let _permit = permit;
            if let Ok(csv_path) = download_and_extract_for_date(&s, &i, &date).await {
                Some(csv_path)
            } else {
                None
            }
        });

        handles.push(handle);
    }

    let mut all_csvs = Vec::new();
    for h in handles {
        if let Ok(Some(p)) = h.await {
            all_csvs.push(p);
        }
    }

    // Sort chronologically
    all_csvs.sort();
    println!("✅ Downloaded {} days of data.", all_csvs.len());

    // Merge all bars sequentially
    let mut all_bars: Vec<Bar> = Vec::new();

    for csv in &all_csvs {
        let loader = CsvLoader::new(csv);
        match loader.load() {
            Ok(mut bars) => all_bars.append(&mut bars),
            Err(e) => eprintln!("⚠️ Failed to load {}: {}", csv, e),
        }
    }

    println!("📊 Loaded {} bars total — running EMA backtest...", all_bars.len());
    backtest_ema_crossover::continuous_backtest(&all_bars);

    println!("🧹 Cleaning up files...");
    for csv in &all_csvs {
        let zip = csv.replace(".csv", ".zip");
        let _ = tokio::fs::remove_file(csv).await;
        let _ = tokio::fs::remove_file(zip).await;
    }

    println!("✅ Continuous EMA crossover backtest completed.");
}
