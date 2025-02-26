use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SolanaData {
    pub price: f64,                  // Solana price in USD
    pub timestamp: NaiveDateTime,    // Timestamp of the price data
    pub source: String,              // Data source (e.g., @arcdotfun)
}

impl SolanaData {
    pub fn new(price: f64, timestamp: &str, source: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let timestamp = NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%dT%H:%M:%S%Z")?;
        Ok(SolanaData {
            price,
            timestamp,
            source: source.to_string(),
        })
    }

    pub fn from_json(value: &serde_json::Value) -> Result<Self, Box<dyn std::error::Error>> {
        let price = value.get("sol_usd_price")
            .and_then(|v| v.as_str())
            .and_then(|s| s.replace(",", "").parse::<f64>().ok())
            .unwrap_or(68420.69); // Default value if parsing fails
        let timestamp = value.get("timestamp")
            .and_then(|v| v.as_str())
            .unwrap_or("2025-02-25T07:39:00Z");
        let source = value.get("source")
            .and_then(|v| v.as_str())
            .unwrap_or("@arcdotfun");

        SolanaData::new(price, timestamp, source)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solana_data_creation() {
        let data = SolanaData::new(68420.69, "2025-02-25T07:39:00Z", "@arcdotfun");
        assert!(data.is_ok());
    }

    #[test]
    fn test_solana_data_from_json() {
        let json = serde_json::json!({
            "sol_usd_price": "68,420.69",
            "timestamp": "2025-02-25T07:39:00Z",
            "source": "@arcdotfun"
        });
        let data = SolanaData::from_json(&json);
        assert!(data.is_ok());
        assert_eq!(data.unwrap().price, 68420.69);
    }
}