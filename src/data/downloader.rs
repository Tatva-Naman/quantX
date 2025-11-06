use reqwest;
use zip::ZipArchive;
use std::io::Cursor;
use std::{fs, io::Write};
use chrono::{Utc};
use std::path::Path;

pub async fn download_and_extract_for_date(
    symbol: &str,
    interval: &str,
    date_str: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!(
        "https://data.binance.vision/data/spot/daily/klines/{}/{}/{}-{}-{}.zip",
        symbol, interval, symbol, interval, date_str
    );

    println!("Fetching data from: {}", url);

    let resp = reqwest::get(&url).await?;
    if !resp.status().is_success() {
        return Err(format!("No file at {}", url).into());
    }

    let bytes = resp.bytes().await?;

    // Ensure directory
    fs::create_dir_all("data/market_data")?;

    // Save zip
    let zip_path = format!("data/market_data/{}-{}-{}.zip", symbol, interval, date_str);
    fs::write(&zip_path, &bytes)?;

    // unzip using blocking thread (zip crate is sync)
    let bytes_clone = bytes.to_vec(); // local owned Vec<u8>
    let csv_path = {
        let symbol = symbol.to_string();
        let interval = interval.to_string();
        let date = date_str.to_string();

        tokio::task::spawn_blocking(move || -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            let reader = Cursor::new(bytes_clone);
            let mut zip = ZipArchive::new(reader)?;
            let mut file = zip.by_index(0)?;
            let csv_path = format!("data/market_data/{}-{}-{}.csv", symbol, interval, date);
            let mut out_file = std::fs::File::create(&csv_path)?;
            std::io::copy(&mut file, &mut out_file)?;
            Ok(csv_path)
        })
        .await??
    };

    println!("✅ Extracted CSV: {}", csv_path);
    Ok(csv_path)
}
