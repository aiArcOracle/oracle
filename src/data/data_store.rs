use serde_json::Value;
use std::error::Error;

pub trait DataStore {
    async fn store(&self, key: &str, data: &Value) -> Result<(), Box<dyn Error>>;
    async fn retrieve(&self, key: &str) -> Result<Value, Box<dyn Error>>;
    async fn delete(&self, key: &str) -> Result<(), Box<dyn Error>>;
}

pub struct InMemoryDataStore {
    store: std::collections::HashMap<String, Value>,
}

impl InMemoryDataStore {
    pub fn new() -> Self {
        InMemoryDataStore {
            store: std::collections::HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl DataStore for InMemoryDataStore {
    async fn store(&self, key: &str, data: &Value) -> Result<(), Box<dyn Error>> {
        let mut store = self.store.lock().await; // Assuming thread-safe, use Mutex for real implementation
        store.insert(key.to_string(), data.clone());
        Ok(())
    }

    async fn retrieve(&self, key: &str) -> Result<Value, Box<dyn Error>> {
        let store = self.store.lock().await;
        store.get(key)
            .cloned()
            .ok_or_else(|| format!("Key '{}' not found", key).into())
    }

    async fn delete(&self, key: &str) -> Result<(), Box<dyn Error>> {
        let mut store = self.store.lock().await;
        if store.remove(key).is_some() {
            Ok(())
        } else {
            Err(format!("Key '{}' not found", key).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_in_memory_data_store() {
        let store = InMemoryDataStore::new();
        let data = serde_json::json!({ "sol_usd_price": 68420.69 });

        // Store data
        store.store("solana_price", &data).await.unwrap();
        let retrieved = store.retrieve("solana_price").await.unwrap();
        assert_eq!(retrieved, data);

        // Delete data
        store.delete("solana_price").await.unwrap();
        let result = store.retrieve("solana_price").await;
        assert!(result.is_err());
    }
}