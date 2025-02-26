use rig::vector_stores::lancedb::LanceDB;
use serde_json::Value;
use std::error::Error;
use crate::vector::store::VectorStoreTrait;

pub struct VectorStore {
    store: LanceDB,
}

impl VectorStore {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let store = LanceDB::new("arcoracle_data").await?;
        Ok(VectorStore { store })
    }
}

#[async_trait::async_trait]
impl VectorStoreTrait for VectorStore {
    async fn insert(&self, key: &str, data: &Value) -> Result<(), Box<dyn Error>> {
        self.store.insert(key, data).await
    }

    async fn query(&self, query: &str) -> Result<Vec<Value>, Box<dyn Error>> {
        self.store.query(query).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tokio;

    #[tokio::test]
    async fn test_vector_store() {
        let store = VectorStore::new().await.unwrap();
        let data = json!({ "sol_usd_price": 68420.69, "timestamp": "2025-02-25T07:39:00Z" });

        // Insert data
        store.insert("solana_price", &data).await.unwrap();
        let results = store.query("solana_price").await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], data);

        // Clean up (optional for testing)
        store.insert("solana_price", &json!(null)).await.unwrap(); // Simulate deletion
    }
}