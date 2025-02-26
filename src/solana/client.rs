use solana_client::rpc_client::RpcClient;

pub struct SolanaClient {
    rpc_client: RpcClient,
}

impl SolanaClient {
    pub fn new(url: &str) -> Self {
        SolanaClient {
            rpc_client: RpcClient::new(url.to_string()),
        }
    }

    pub async fn get_balance(&self, pubkey: &solana_sdk::pubkey::Pubkey) -> Result<u64, Box<dyn std::error::Error>> {
        Ok(self.rpc_client.get_balance(pubkey)?)
    }
}
