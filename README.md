
---

# QuantX — Rust Trading Backtester

QuantX is a lightweight algorithmic trading research and backtesting engine written in **Rust**.
It focuses on correctness, execution realism, and high-performance multi-year backtesting.

---

## 🚀 Features

* Binance historical **1-hour candle data**
* Auto-download & unzip per-day CSV files
* Continuous multi-year aggregation pipeline
* Strategy engine with:

  * EMA trend-switch strategy
  * Always-Buy & Always-Sell baselines
* Realistic execution:

  * Dynamic position sizing
  * Fees & slippage
  * Position square-off handling
* Performance statistics output

---

## ⚙️ Advanced Concepts Used (Core Strength of This Project)

This project is not just a toy backtester — it intentionally uses **real-world systems engineering techniques**.

### ✅ Asynchronous & Parallel Data Pipeline

| Stage                     | Technology                 | Purpose                                  |
| ------------------------- | -------------------------- | ---------------------------------------- |
| Binance downloads         | `Tokio async + reqwest`    | Non-blocking HTTP                        |
| Multi-file parallel fetch | `tokio::spawn + Semaphore` | Controlled concurrency batch scheduling  |
| CSV parsing               | `spawn_blocking()`         | Runs parsing on dedicated worker threads |
| Backtest execution        | sync compute loop          | Efficient CPU-tight processing           |

### 🔹 Downloading = Multiprocessing-Style Concurrency

Multiple dates are downloaded concurrently:

* Each task runs independently
* Network I/O does not block the runtime
* Concurrency limit prevents overload
* Behaves similar to **process pool batching**

But implemented safely using async tasks instead of real OS processes.

### 🔹 CSV Parsing = True Multithreading

CSV parsing is CPU-bound work — so we offload it to worker threads:

```rust
tokio::task::spawn_blocking(|| loader.load())
```

This ensures:

* No blocking inside async runtime
* Parsing runs on **separate OS threads**
* CPU cores stay fully utilized
* Memory safety guaranteed by Rust

This is a real-world production-grade pattern.

---

## 🧩 Project Architecture

```
quantX
│
├── main.rs
│   ├── async downloader (Tokio)
│   ├── batched parallel file fetch
│   ├── CSV parsing workers (spawn_blocking)
│   ├── multi-year bar aggregation
│   └── continuous EMA backtest runner
│
├── data/
│   ├── bar.rs
│   ├── loader.rs
│   └── downloader.rs
│
├── strategy/
│   ├── always_buy.rs
│   ├── always_sell.rs
│   └── ema_switch.rs
│
└── backtest/
    └── backtest_ema_crossover.rs
```

---

## ⚡ Quick Start

Install Rust:

```bash
https://rustup.rs
```

Clone repo:

```bash
git clone <repo-url>
cd quantX
```

Run backtest:

```bash
cargo run
```

The engine will:

1. Download multi-year historical data
2. Parse CSV files in parallel
3. Merge candles into a single timeline
4. Run EMA trend-switch strategy
5. Apply fees + slippage
6. Print detailed performance summary

---

## 🧠 Why Rust for Quant Backtesting?

* Predictable execution
* Strong type safety
* Zero-cost abstractions
* Safe concurrency model
* Ideal for research → live trading pipelines

This project is intentionally structured like a **real quant stack**.

---

## 📌 Future Enhancements

* Multi-symbol portfolio backtesting
* Risk models & exposure limits
* Performance metrics (Sharpe / Sortino / DD)
* Parameter grid-search optimizer
* Live trading bridge (paper → real)

---
