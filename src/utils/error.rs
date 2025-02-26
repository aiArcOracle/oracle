use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArcOracleError {
    #[error("Data fetch error: {0}")]
    DataFetch(String),
    #[error("LLM processing error: {0}")]
    LlmProcessing(String),
    #[error("Solana error: {0}")]
    Solana(String),
    #[error("Vector store error: {0}")]
    VectorStore(String),
}
