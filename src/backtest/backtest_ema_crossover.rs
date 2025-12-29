use crate::data::bar::Bar;
use crate::data::order::OrderSide;
use crate::strategy::ema_switch::EmaSwitchStrategy;

pub fn continuous_backtest(bars: &[Bar]) {
    let mut strategy = EmaSwitchStrategy::new(9 * 24, 20 * 24);

    let starting_cash = 150_000.0;
    let mut cash = starting_cash;
    let mut position_qty: f64 = 0.0;
    let mut entry_price: f64 = 0.0;

    let mut trades = 0usize;
    let mut wins = 0usize;
    let mut losses = 0usize;

    let commission_rate = 0.001; // 0.1%
    let slippage_rate = 0.0005;  // 0.05%
    let min_cash_threshold = 5000.0; // don't trade below this

    for bar in bars {
        let orders = strategy.generate_signal(bar);

        for order in orders {
            trades += 1;

            match order.side {
                OrderSide::Buy => {
                    // Skip if already long
                    if position_qty > 0.0 {
                        continue;
                    }

                    // Close short first (if any)
                    if position_qty < 0.0 {
                        let close_price = order.price * (1.0 + slippage_rate);
                        let pnl = (entry_price - close_price) * position_qty.abs();
                        cash += pnl;
                        let fee = close_price * position_qty.abs() * commission_rate;
                        cash -= fee;
                        if pnl > 0.0 { wins += 1; } else { losses += 1; }
                        println!("📈 CLOSE SHORT @ {:.2} | PnL = {:.2}", close_price, pnl);
                        position_qty = 0.0;
                    }

                    // Open new long only if enough cash
                    if cash < min_cash_threshold {
                        continue;
                    }

                    let fill_price = order.price * (1.0 + slippage_rate);
                    let investable_cash = cash * 0.999; // keep tiny buffer
                    let quantity = investable_cash / fill_price;
                    if quantity <= 0.0 {
                        continue;
                    }

                    let cost = fill_price * quantity;
                    let fee = cost * commission_rate;
                    cash -= cost + fee;
                    entry_price = fill_price;
                    position_qty = quantity;

                    println!(
                        "BUY  {:.4} BTC @ {:.2} (cost {:.2}, fee {:.2})",
                        position_qty, fill_price, cost, fee
                    );
                }

                OrderSide::Sell => {
                    // Skip if already short
                    if position_qty < 0.0 {
                        continue;
                    }

                    // Close long position first
                    if position_qty > 0.0 {
                        let close_price = order.price * (1.0 - slippage_rate);
                        let revenue = close_price * position_qty;
                        let fee = revenue * commission_rate;
                        let pnl = (close_price - entry_price) * position_qty;
                        cash += revenue - fee;

                        if pnl > 0.0 { wins += 1; } else { losses += 1; }

                        println!(
                            "📉 CLOSE LONG {:.4} BTC @ {:.2} | PnL = {:.2}",
                            position_qty, close_price, pnl
                        );

                        position_qty = 0.0;
                    }
                }
            }
        }
    }

    // Final square-off if needed
    if position_qty > 0.0 {
        if let Some(last) = bars.last() {
            let close_price = last.close * (1.0 - slippage_rate);
            let pnl = (close_price - entry_price) * position_qty;
            let fee = close_price * position_qty * commission_rate;
            cash += close_price * position_qty - fee;
            println!(
                "🔚 FINAL SQUAREOFF {:.4} BTC @ {:.2} | PnL = {:.2}",
                position_qty, close_price, pnl
            );
            position_qty = 0.0;
        }
    }

    let final_pnl = cash - starting_cash;
    let return_pct = (cash / starting_cash - 1.0) * 100.0;

    println!("\n----------------------------");
    println!("✅ Final Summary (Dynamic Qty, Realistic, fees + slippage)");
    println!("Starting Cash: {:.2}", starting_cash);
    println!("Final Cash:    {:.2}", cash);
    println!("Net PnL:       {:.2}", final_pnl);
    println!("Return:        {:.2}%", return_pct);
    println!("Total Trades:  {}", trades);
    println!("Winning Trades: {}", wins);
    println!("Losing Trades:  {}", losses);
    if losses > 0 {
        println!("Win/Loss Ratio: {:.2}", wins as f64 / losses as f64);
    }
    println!("----------------------------");
}
