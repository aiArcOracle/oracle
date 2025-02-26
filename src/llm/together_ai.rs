use rig::{
    completion::{CompletionRequest, CompletionResponse, Provider, ProviderError, ProviderResult},
};
use reqwest::Client;
use std::error::Error;

pub struct TogetherAIProvider {
    api_key: String,
    base_url: String,
    client: Client,
}

impl TogetherAIProvider {
    pub fn new(api_key: String) -> Self {
        TogetherAIProvider {
            api_key,
            base_url: "https://api.together.xyz/v1/inference".to_string(),
            client: Client::new(),
        }
    }

    async fn request(&self, prompt: &str, stream: bool) -> Result<String, Box<dyn Error>> {
        let request_body = serde_json::json!({
            "model": "DeepSeek-R1",
            "messages": [{"role": "user", "content": prompt}],
            "stream": stream,
            "max_tokens": 200,
            "temperature": 0.7,
        });

        let response = self.client
            .post(&self.base_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body)
            .send()
            .await?;

        if stream {
            let mut full_response = String::new();
            let mut stream = response.bytes_stream();
            while let Some(chunk) = stream.try_next().await? {
                let chunk_str = String::from_utf8(chunk.to_vec())?;

                if chunk_str.starts_with("data: ") {
                    let data = chunk_str[6..].trim();
                    if data == "[DONE]" {
                        break;
                    }
                    let json_data: serde_json::Value = serde_json::from_str(data)?;
                    if let Some(delta) = json_data["choices"][0]["delta"]["content"].as_str() {
                        full_response.push_str(delta);
                    }
                }
            }
            Ok(full_response)
        } else {
            let response_text = response.text().await?;
            let response_json: serde_json::Value = serde_json::from_str(&response_text)?;
            let content = response_json["choices"][0]["message"]["content"]
                .as_str()
                .unwrap_or_default()
                .to_string();
            Ok(content)
        }
    }
}

impl Provider for TogetherAIProvider {
    fn name(&self) -> &str {
        "Together AI"
    }

    async fn complete(&self, request: CompletionRequest) -> ProviderResult<CompletionResponse> {
        let prompt = request.prompt;
        let stream = request.stream;

        let response_text = self.request(&prompt, stream).await.map_err(|e| ProviderError::Completion(e.to_string()))?;

        Ok(CompletionResponse {
            text: response_text,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_together_ai_completion() {
        let api_key = "your_together_ai_api_key".to_string(); // Replace with a valid key for testing
        let provider = TogetherAIProvider::new(api_key);
        let prompt = "Analyze Solana price at $68,420.69. Provide a concise insight.";
        let request = CompletionRequest {
            prompt: prompt.to_string(),
            stream: false,
            ..Default::default()
        };
        let response = provider.complete(request).await.unwrap();
        assert!(!response.text.is_empty());
    }
}