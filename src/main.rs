use arcoracle_backend::{
    data::{fetch_data_from_arcdotfun, SolanaData},
    llm::ArcOracleAgent,
    solana::store_data_on_solana,
    vector::VectorStore,
    utils::{load_config, init_logging},
};
use rig::{
    completion::{Prompt, CompletionRequest, CompletionResponse},
    providers::{Provider, ProviderError, ProviderResult},
};
use solana_sdk::{pubkey::Pubkey, signer::keypair::Keypair};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let together_api_key = env::var("TOGETHER_API_KEY").expect("TOGETHER_API_KEY must be set");

    let agent = ArcOracleAgent::new(together_api_key).await?;
    let vector_store = VectorStore::new().await?;

    let data_source = fetch_data_from_arcdotfun().await?;
    let solana_price = data_source.get("sol_usd_price").unwrap_or(&"68,420.69".to_string());

    vector_store.insert(&format!("Solana Price: {}", solana_price), &data_source).await?;

    let prompt = Prompt::new(&format!(
        "Analyze this Solana price data: {}. Provide a concise insight for DeFi users.",
        solana_price
    ));
    let request = CompletionRequest {
        prompt,
        stream: false,  // Non-streaming by default, can be changed to true for streaming
        ..Default::default()
    };
    let response = agent.complete(request).await?;
    println!("ArcOracle Insight: {}", response.text);

    let keypair = Keypair::new();
    let pubkey = Pubkey::new(&keypair.pubkey().to_bytes());
    store_data_on_solana(&keypair, &data_source).await?;
    println!("Storing data on Solana at Pubkey: {}", pubkey);

    Ok(())
}
