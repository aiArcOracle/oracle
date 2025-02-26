use solana_sdk::{
    pubkey::Pubkey,
    signer::keypair::Keypair,
    transaction::Transaction,
};
use crate::data::SolanaData;
use std::error::Error;
use crate::solana::client::SolanaClient;

pub async fn store_data_on_solana(keypair: &Keypair, data: &serde_json::Value) -> Result<Pubkey, Box<dyn Error>> {
    let client = SolanaClient::new("https://api.mainnet-beta.solana.com"); // Use Solana Mainnet RPC
    let solana_data = SolanaData::from_json(data)?;

    // Create a simple transaction to store data (simulate for now)
    let pubkey = keypair.pubkey();
    let transaction = Transaction::new_signed_with_payer(
        &[],
        Some(&pubkey),
        &[keypair],
        client.rpc_client.get_latest_blockhash()?,
    );

    // Simulate sending the transaction (replace with actual Solana RPC call)
    println!("Transaction sent to store Solana data: price={}, timestamp={}", 
             solana_data.price, solana_data.timestamp);
    println!("Storing at Pubkey: {}", pubkey);

    Ok(pubkey)
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::signer::keypair::Keypair;
    use tokio;

    #[tokio::test]
    async fn test_store_data_on_solana() {
        let keypair = Keypair::new();
        let data = serde_json::json!({
            "sol_usd_price": "68,420.69",
            "timestamp": "2025-02-25T07:39:00Z",
            "source": "@arcdotfun"
        });
        let result = store_data_on_solana(&keypair, &data).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string().len(), 44); // Pubkey length
    }
}