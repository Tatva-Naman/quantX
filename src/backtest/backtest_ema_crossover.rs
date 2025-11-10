use crate::data::bar::Bar;
use crate::data::order::OrderSide;
use crate::strategy::ema_switch::EmaSwitchStrategy;

pub fn continuous_backtest(bars: &[Bar]) {
    let mut strategy = EmaSwitchStrategy::new(9 * 24, 20 * 24);

    let starting_cash = 150_000.0;
    let mut cash = starting_cash;
    let mut position: i64 = 0;
    let mut entry_price: f64 = 0.0;
    let mut trades = 0usize;
    let mut wins = 0usize;
    let mut losses = 0usize;

    let commission_rate = 0.001; // 0.1%
    let slippage_rate = 0.0005;  // 0.05%

    for bar in bars {
        let orders = strategy.generate_signal(bar);

        for order in orders {
            trades += 1;

            match order.side {
                OrderSide::Buy => {
                    // If currently short → close it first
                    if position < 0 {
                        let close_price = order.price * (1.0 + slippage_rate);
                        let pnl = (entry_price - close_price) * position.abs() as f64;
                        cash += pnl; // realized pnl
                        if pnl > 0.0 { wins += 1; } else { losses += 1; }
                        println!("📈 CLOSE SHORT @ {:.2} | PnL = {:.2}", close_price, pnl);
                        position = 0;
                    }

                    // Open long
                    let fill_price = order.price * (1.0 + slippage_rate);
                    let cost = fill_price * order.quantity as f64;
                    let fee = cost * commission_rate;
                    cash -= cost + fee;
                    entry_price = fill_price;
                    position = 1;

                    println!("BUY  @ {:.2} [{}] (fee {:.2})", fill_price, bar.timestamp, fee);
                }

                OrderSide::Sell => {
                    // If currently long → close it first
                    if position > 0 {
                        let close_price = order.price * (1.0 - slippage_rate);
                        let pnl = (close_price - entry_price) * position.abs() as f64;
                        cash += pnl; // realized pnl
                        if pnl > 0.0 { wins += 1; } else { losses += 1; }
                        println!("📉 CLOSE LONG @ {:.2} | PnL = {:.2}", close_price, pnl);
                        position = 0;
                    }

                    // Open short
                    let fill_price = order.price * (1.0 - slippage_rate);
                    let revenue = fill_price * order.quantity as f64;
                    let fee = revenue * commission_rate;
                    cash += revenue - fee;
                    entry_price = fill_price;
                    position = -1;

                    println!("SELL @ {:.2} [{}] (fee {:.2})", fill_price, bar.timestamp, fee);
                }
            }
        }
    }

    // Square off any open position
    if position != 0 {
        if let Some(last) = bars.last() {
            let close_price = last.close;
            let pnl = match position {
                1 => (close_price - entry_price) * position.abs() as f64,
                -1 => (entry_price - close_price) * position.abs() as f64,
                _ => 0.0,
            };
            cash += pnl;
            if pnl > 0.0 { wins += 1; } else { losses += 1; }
            println!("🔚 FINAL SQUAREOFF @ {:.2} | PnL = {:.2}", close_price, pnl);
        }
    }

    let final_pnl = cash - starting_cash;

    println!("\n----------------------------");
    println!("✅ Final Summary (fees + slippage)");
    println!("Starting Cash: {:.2}", starting_cash);
    println!("Final Cash:    {:.2}", cash);
    println!("Net PnL:       {:.2}", final_pnl);
    println!("Total Trades:  {}", trades);
    println!("Winning Trades: {}", wins);
    println!("Losing Trades:  {}", losses);
    if losses > 0 {
        println!("Win/Loss Ratio: {:.2}", wins as f64 / losses as f64);
    }
    println!("----------------------------");
}
