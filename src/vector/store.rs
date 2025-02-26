use serde_json::Value;
use std::error::Error;

#[async_trait::async_trait]
pub trait VectorStoreTrait {
    async fn insert(&self, key: &str, data: &Value) -> Result<(), Box<dyn Error>>;
    async fn query(&self, query: &str) -> Result<Vec<Value>, Box<dyn Error>>;
}

pub async fn create_vector_store() -> Result<Box<dyn VectorStoreTrait>, Box<dyn Error>> {
    Ok(Box::new(lancedb::VectorStore::new().await?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector::lancedb;
    use tokio;

    #[tokio::test]
    async fn test_create_vector_store() {
        let store = create_vector_store().await.unwrap();
        let data = serde_json::json!({ "sol_usd_price": 68420.69 });
        store.insert("test_key", &data).await.unwrap();
        let results = store.query("test_key").await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], data);
    }
}