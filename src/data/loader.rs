use crate::data::bar::Bar;
use csv::ReaderBuilder;
use std::{error::Error, fs::File, io::BufReader};
use chrono::{DateTime, NaiveDateTime, Utc};

pub struct CsvLoader {
    pub path: String,
}

impl CsvLoader {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    pub fn load(&self) -> Result<Vec<Bar>, Box<dyn Error + Send + Sync>> {
        let file = File::open(&self.path)?;
        // Binance files have no header row
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(BufReader::new(file));

        let mut bars = Vec::new();
        for result in rdr.records() {
            let record = result?;
            let ts_micro: i64 = record[0].parse::<i64>()?;
            let ts_sec = ts_micro / 1_000_000;
            let naive = NaiveDateTime::from_timestamp_opt(ts_sec, 0).ok_or("Invalid timestamp")?;
            let datetime = DateTime::<Utc>::from_utc(naive, Utc);
            let timestamp = datetime.to_rfc3339();

            let bar = Bar {
                timestamp,
                open: record[1].parse::<f64>()?,
                high: record[2].parse::<f64>()?,
                low: record[3].parse::<f64>()?,
                close: record[4].parse::<f64>()?,
                volume: record[5].parse::<f64>()?,
            };
            bars.push(bar);
        }
        Ok(bars)
    }
}
