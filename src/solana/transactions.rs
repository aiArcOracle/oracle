use solana_sdk::{signer::Signer, transaction::Transaction};

pub fn create_transaction(signer: &solana_sdk::signer::keypair::Keypair) -> Transaction {
    Transaction::new_signed_with_payer(
        &[],
        Some(&signer.pubkey()),
        &[signer],
        solana_sdk::hash::Hash::default(),
    )
}
