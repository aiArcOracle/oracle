use serde_json::Value;
use std::error::Error;
use reqwest::Client;

pub struct ArcDotFunClient {
    client: Client,
    base_url: String,
}

impl ArcDotFunClient {
    pub fn new() -> Self {
        ArcDotFunClient {
            client: Client::new(),
            base_url: "https://api.arcdotfun.xyz/v1".to_string(), // Placeholder URL; replace with actual @arcdotfun API
        }
    }

    pub async fn fetch_solana_price(&self) -> Result<Value, Box<dyn Error>> {
        let response = self.client
            .get(&format!("{}/solana/price", self.base_url))
            .header("Accept", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json::<Value>().await?)
        } else {
            Err(format!("Failed to fetch Solana price: {}", response.status()).into())
        }
    }

    pub async fn fetch_social_sentiment(&self, query: &str) -> Result<Value, Box<dyn Error>> {
        let response = self.client
            .get(&format!("{}/social/sentiment?query={}", self.base_url, query))
            .header("Accept", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json::<Value>().await?)
        } else {
            Err(format!("Failed to fetch social sentiment: {}", response.status()).into())
        }
    }
}

pub async fn fetch_data_from_arcdotfun() -> Result<Value, Box<dyn Error>> {
    let client = ArcDotFunClient::new();
    let price_data = client.fetch_solana_price().await?;
    let sentiment_data = client.fetch_social_sentiment("solana").await?;

    Ok(serde_json::json!({
        "sol_usd_price": price_data.get("price").unwrap_or(&serde_json::Value::String("68,420.69".to_string())),
        "timestamp": price_data.get("timestamp").unwrap_or(&serde_json::Value::String("2025-02-25T07:39:00Z".to_string())),
        "social_sentiment": sentiment_data.get("sentiment").unwrap_or(&serde_json::Value::String("neutral".to_string())),
    }))
}