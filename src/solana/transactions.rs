use solana_sdk::{
    signer::Signer,
    transaction::Transaction,
    pubkey::Pubkey,
    hash::Hash,
};

pub fn create_transaction(signer: &solana_sdk::signer::keypair::Keypair, to: &Pubkey, amount: u64, recent_blockhash: Hash) -> Transaction {
    let instruction = solana_sdk::system_instruction::transfer(
        &signer.pubkey(),
        to,
        amount,
    );

    Transaction::new_signed_with_payer(
        &[instruction],
        Some(&signer.pubkey()),
        &[signer],
        recent_blockhash,
    )
}

pub fn sign_transaction(transaction: &mut Transaction, signer: &solana_sdk::signer::keypair::Keypair) {
    transaction.sign(&[signer], transaction.message.recent_blockhash);
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::signer::keypair::Keypair;

    #[test]
    fn test_create_transaction() {
        let signer = Keypair::new();
        let to = Pubkey::new_unique();
        let recent_blockhash = Hash::new_from_array([0u8; 32]);
        let transaction = create_transaction(&signer, &to, 1000, recent_blockhash);
        assert_eq!(transaction.signatures.len(), 1);
        assert_eq!(transaction.message.account_keys.len(), 2); // Payer and recipient
    }

    #[test]
    fn test_sign_transaction() {
        let signer = Keypair::new();
        let to = Pubkey::new_unique();
        let recent_blockhash = Hash::new_from_array([0u8; 32]);
        let mut transaction = create_transaction(&signer, &to, 1000, recent_blockhash);
        assert_eq!(transaction.signatures[0].verify(&signer.pubkey(), transaction.message.data()), true);
    }
}