#[derive(Debug, Clone)]
pub struct Bar {
    pub timestamp: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bar_creation() {
        let bar = Bar {
            timestamp: "2025-10-24T00:15:00Z".to_string(),
            open: 1.0,
            high: 2.0,
            low: 0.5,
            close: 1.5,
            volume: 140.00,
        };
        assert_eq!(bar.close, 1.5);
    }
}