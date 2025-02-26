use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::error::Error;

pub struct SolanaClient {
    rpc_client: RpcClient,
}

impl SolanaClient {
    pub fn new(url: &str) -> Self {
        SolanaClient {
            rpc_client: RpcClient::new(url.to_string()),
        }
    }

    pub async fn get_balance(&self, pubkey: &Pubkey) -> Result<u64, Box<dyn Error>> {
        Ok(self.rpc_client.get_balance(pubkey)?)
    }

    pub async fn get_recent_blockhash(&self) -> Result<solana_sdk::hash::Hash, Box<dyn Error>> {
        Ok(self.rpc_client.get_latest_blockhash()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::pubkey::Pubkey;
    use tokio;

    #[tokio::test]
    async fn test_solana_client_balance() {
        let client = SolanaClient::new("https://api.mainnet-beta.solana.com");
        let pubkey = Pubkey::new_unique(); // Use a real pubkey for testing if needed
        let balance = client.get_balance(&pubkey).await;
        assert!(balance.is_ok()); // May return 0 if pubkey is new
    }

    #[tokio::test]
    async fn test_solana_client_blockhash() {
        let client = SolanaClient::new("https://api.mainnet-beta.solana.com");
        let blockhash = client.get_recent_blockhash().await;
        assert!(blockhash.is_ok());
    }
}