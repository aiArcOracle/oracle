use rig::{
    completion::Prompt,
    providers::{openai, gemini},
};

pub struct ArcOracleAgent {
    openai_client: openai::Client,
    gemini_client: gemini::Client,
}

impl ArcOracleAgent {
    pub async fn new(openai_client: &openai::Client, gemini_client: &gemini::Client) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(ArcOracleAgent {
            openai_client: openai_client.clone(),
            gemini_client: gemini_client.clone(),
        })
    }

    pub async fn complete(&self, prompt: Prompt) -> Result<rig::completion::Response, Box<dyn std::error::Error>> {
        let agent = self.openai_client
            .agent(openai::completion::GPT_4_TURBO)
            .preamble("You are ArcOracle, an AI agent providing real-time data insights for Solana. Be concise and accurate.")
            .temperature(0.7)
            .build();
        agent.complete(prompt).await
    }
}
