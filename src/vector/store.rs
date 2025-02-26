use serde_json::Value;

pub trait VectorStoreTrait {
    async fn insert(&self, key: &str, data: &Value) -> Result<(), Box<dyn std::error::Error>>;
    async fn query(&self, query: &str) -> Result<Vec<Value>, Box<dyn std::error::Error>>;
}
