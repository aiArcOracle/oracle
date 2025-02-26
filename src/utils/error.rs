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
    #[error("Configuration error: {0}")]
    Config(String),
}

impl From<reqwest::Error> for ArcOracleError {
    fn from(error: reqwest::Error) -> Self {
        ArcOracleError::DataFetch(error.to_string())
    }
}

impl From<rig::providers::ProviderError> for ArcOracleError {
    fn from(error: rig::providers::ProviderError) -> Self {
        ArcOracleError::LlmProcessing(error.to_string())
    }
}

impl From<solana_sdk::transaction::TransactionError> for ArcOracleError {
    fn from(error: solana_sdk::transaction::TransactionError) -> Self {
        ArcOracleError::Solana(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let data_error = ArcOracleError::DataFetch("Failed to fetch data".to_string());
        assert_eq!(data_error.to_string(), "Data fetch error: Failed to fetch data");

        let llm_error = ArcOracleError::LlmProcessing("LLM failed".to_string());
        assert_eq!(llm_error.to_string(), "LLM processing error: LLM failed");
    }
}