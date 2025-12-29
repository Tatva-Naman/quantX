---

# QuantX — Rust Trading Backtester

QuantX is a lightweight algorithmic trading research and backtesting engine written in **Rust**.
It is designed for experimenting with trading strategies on historical crypto market data, with a focus on performance, correctness, and realistic execution modeling.

---

## 🚀 Features

* Download historical **Binance 1-hour candles**
* Parse & load CSV into typed bar structures
* Run strategies over multi-year datasets
* Support for:

  * Continuous EMA trend-switch strategy
  * Always-Buy / Always-Sell baseline models
* Realistic execution logic:

  * Position square-off
  * Dynamic position sizing
  * Fees & slippage modeling
* Generates backtest statistics:

  * Final cash
  * Net PnL
  * Win / loss count
  * Trade count

---

## ⚙️ Current Strategy: EMA Trend Switch

Trend bias is determined using rolling EMAs.

| Condition                            | Action                  |
| ------------------------------------ | ----------------------- |
| Short EMA **crosses above** Long EMA | Close short → Open long |
| Short EMA **crosses below** Long EMA | Close long → Open short |

Only **one position is active at a time**.

Fees & slippage are applied on every execution.

---

## 🧩 Project Architecture (Compact)

```
quantX
│
├── main.rs
│   ├── async downloader (Tokio)
│   ├── multi-day CSV aggregation
│   ├── continuous strategy backtest
│   └── cleanup & reporting
│
├── data/
│   ├── bar.rs          → Market candle model
│   ├── loader.rs       → CSV → Vec<Bar>
│   └── downloader.rs   → Download + unzip Binance data
│
├── strategy/
│   ├── strategy.rs     → Strategy trait
│   ├── always_buy.rs
│   ├── always_sell.rs
│   └── ema_switch.rs   → EMA crossover switching logic
│
└── backtest/
    └── backtest_ema_crossover.rs
        → Execution engine + PnL + fees + slippage
```

---

## ⚡ Quick Start

### 1️⃣ Install Rust

```bash
https://rustup.rs
```

### 2️⃣ Clone Repository

```bash
git clone <repo-url>
cd quantX
```

### 3️⃣ Build

```bash
cargo build
```

### 4️⃣ Run Continuous Multi-Year Backtest

```bash
cargo run
```

The program will:

1. Download Binance hourly candles
2. Merge historical CSV files
3. Run EMA crossover strategy
4. Apply fees + slippage
5. Print detailed performance summary

---

## 🧠 Why Rust for Quant Backtesting?

* Zero-cost abstractions
* Deterministic memory behavior
* High-performance loops for bar processing
* Safe concurrency with Tokio
* Great foundation for **future live-trading bots**

This engine is intentionally lightweight and modular to encourage experimentation.

---

## 📌 Future Roadmap

* Portfolio support (multi-symbol)
* Risk models & position sizing modes
* Advanced metrics (Sharpe, Sortino, DD)
* Strategy parameter optimization
* Live trading gateway (paper → real)

---