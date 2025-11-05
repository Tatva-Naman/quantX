use chrono::{Datelike, Utc};
use std::{fs, io::Write, path::Path};
use reqwest::blocking::get;

use zip::ZipArchive;
use std::io::Cursor;


pub fn download_binance_csv(symbol: &str, interval: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Get current date (Binance data is stored per-day)
    let now = Utc::now() - chrono::Duration::days(1); // Use yesterday's date to ensure data availability
    let date_str = format!("{:04}-{:02}-{:02}", now.year(), now.month(), now.day());

    // https://data.binance.vision/data/spot/daily/klines/BTCUSDT/1h/BTCUSDT-1h-2025-11-04.zip
    let url = format!(
        "https://data.binance.vision/data/spot/monthly/klines/{symbol}/{interval}/{symbol}-{interval}-{date}.zip",
        symbol = symbol,
        interval = interval,
        date = date_str
    );

    println!("Fetching data from: {url}");

    let response = get(&url)?;
    if !response.status().is_success() {
        return Err(format!("Failed to fetch data. Status: {}", response.status()).into());
    }

    fs::create_dir_all("data/market_data")?;

    let zip_path = format!("data/market_data/{symbol}-{interval}-{date}.zip", symbol = symbol, interval = interval, date = date_str);
    let bytes = response.bytes()?; 

    let mut file = fs::File::create(&zip_path)?;
    file.write_all(&bytes)?; // Use the stored bytes

    println!("✅ Saved ZIP: {zip_path}");

    let bytes = &bytes;
    let reader = Cursor::new(bytes);
    let mut zip = ZipArchive::new(reader)?;
    let mut file = zip.by_index(0)?;
    let mut out_path = format!("data/market_data/{symbol}-{interval}-{date_str}.csv");
    let mut out_file = fs::File::create(&out_path)?;
    std::io::copy(&mut file, &mut out_file)?;
    println!("✅ Extracted CSV: {out_path}");

    Ok(out_path)
}
