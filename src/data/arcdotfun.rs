use serde_json::Value;
use std::error::Error;

pub async fn fetch_data_from_arcdotfun() -> Result<Value, Box<dyn Error>> {
    let data = serde_json::json!({
        "sol_usd_price": "68,420.69",
        "timestamp": "2025-02-25T07:39:00Z"
    });
    Ok(data)
}
