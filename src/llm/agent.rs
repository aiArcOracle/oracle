use rig::{
    completion::{CompletionRequest, CompletionResponse, ProviderResult},
    providers::{Provider},
};
use std::error::Error;
use crate::llm::together_ai::TogetherAIProvider;

pub struct ArcOracleAgent {
    provider: Box<dyn Provider>,
}

impl ArcOracleAgent {
    pub async fn new(together_api_key: String) -> Result<Self, Box<dyn Error>> {
        let provider = Box::new(TogetherAIProvider::new(together_api_key));
        Ok(ArcOracleAgent { provider })
    }

    pub async fn complete(&self, request: CompletionRequest) -> ProviderResult<CompletionResponse> {
        self.provider.complete(request).await
    }

    pub async fn analyze_solana_data(&self, data: &str) -> Result<String, Box<dyn Error>> {
        let prompt = format!(
            "Analyze this Solana data: {}. Provide a concise insight for DeFi users, focusing on price trends and market sentiment.",
            data
        );
        let request = CompletionRequest {
            prompt,
            stream: false,  // Non-streaming by default; can be set to true for real-time
            ..Default::default()
        };
        let response = self.complete(request).await?;
        Ok(response.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_agent_completion() {
        let api_key = "your_together_ai_api_key".to_string(); // Replace with a valid key for testing
        let agent = ArcOracleAgent::new(api_key).await.unwrap();
        let prompt = "Analyze Solana price at $68,420.69. Provide a concise insight for DeFi users.";
        let request = CompletionRequest {
            prompt: prompt.to_string(),
            stream: false,
            ..Default::default()
        };
        let response = agent.complete(request).await.unwrap();
        assert!(!response.text.is_empty());
    }
}