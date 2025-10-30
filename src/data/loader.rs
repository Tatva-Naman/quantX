use crate::data::bar::Bar;
use std::{error::Error, fs::File, io::BufReader};
use csv::ReaderBuilder;

pub struct CsvLoader {
    pub path: String,
}

impl CsvLoader {
    pub fn new(path: &str) -> Self {
        Self { path: path.to_string() }
    }

    pub fn load(&self) -> Result<Vec<Bar>, Box<dyn Error>> {
        let file = File::open(&self.path)?;
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(BufReader::new(file));

        let mut bars = Vec::new();
        for result in rdr.records() {
            let record = result?;
            let bar = Bar {
                timestamp: record[0].to_string(),
                open: record[1].parse::<f64>()?,
                high: record[2].parse::<f64>()?,
                low: record[3].parse::<f64>()?,
                close: record[4].parse::<f64>()?,
                volume: record[5].parse::<i64>()?,
            };
            bars.push(bar);
        }
        Ok(bars)
    }
}
