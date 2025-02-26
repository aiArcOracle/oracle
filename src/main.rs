use arcoracle_backend::{
    data::fetch_data_from_arcdotfun,
    llm::ArcOracleAgent,
    solana::store_data_on_solana,
    vector::VectorStore,
};
use rig::{
    completion::Prompt,
    providers::{openai, gemini},
    vector_stores::lancedb::LanceDB,
};
use solana_sdk::{pubkey::Pubkey, signer::keypair::Keypair};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let gemini_api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");

    let openai_client = openai::Client::new(openai_api_key);
    let gemini_client = gemini::Client::from_env();

    let agent = ArcOracleAgent::new(&openai_client, &gemini_client).await?;
    let vector_store = VectorStore::new().await?;

    let data_source = fetch_data_from_arcdotfun().await?;
    let solana_price = data_source.get("sol_usd_price").unwrap_or(&"68,420.69".to_string());

    vector_store.insert(&format!("Solana Price: {}", solana_price), &data_source).await?;

    let prompt = Prompt::new(&format!(
        "Analyze this Solana price data: {}. Provide a concise insight for DeFi users.",
        solana_price
    ));
    let response = agent.complete(prompt).await?;
    println!("ArcOracle Insight: {}", response.text);

    let keypair = Keypair::new();
    let pubkey = Pubkey::new(&keypair.pubkey().to_bytes());
    store_data_on_solana(&keypair, &data_source).await?;
    println!("Storing data on Solana at Pubkey: {}", pubkey);

    Ok(())
}
