use serde_json::Value;

pub trait DataStore {
    async fn store(&self, data: &Value) -> Result<(), Box<dyn std::error::Error>>;
    async fn retrieve(&self, key: &str) -> Result<Value, Box<dyn std::error::Error>>;
}
