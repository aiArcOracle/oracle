pub struct SolanaData {
    price: String,
    timestamp: String,
}

impl SolanaData {
    pub fn new(price: String, timestamp: String) -> Self {
        SolanaData { price, timestamp }
    }
}
