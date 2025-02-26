use rig::vector_stores::lancedb::LanceDB;

pub struct VectorStore {
    store: LanceDB,
}

impl VectorStore {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let store = LanceDB::new("arcoracle_data").await?;
        Ok(VectorStore { store })
    }

    pub async fn insert(&self, key: &str, data: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        self.store.insert(key, data).await
    }

    pub async fn query(&self, query: &str) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        self.store.query(query).await
    }
}
