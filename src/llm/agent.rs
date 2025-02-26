use rig::{
    completion::{CompletionRequest, CompletionResponse, ProviderResult},
    providers::{Provider},
};
use std::error::Error;

pub struct ArcOracleAgent {
    provider: Box<dyn Provider>,
}

impl ArcOracleAgent {
    pub async fn new(together_api_key: String) -> Result<Self, Box<dyn Error>> {
        let provider = Box::new(super::together_ai::TogetherAIProvider::new(together_api_key));
        Ok(ArcOracleAgent { provider })
    }

    pub async fn complete(&self, request: CompletionRequest) -> ProviderResult<CompletionResponse> {
        self.provider.complete(request).await
    }
}
