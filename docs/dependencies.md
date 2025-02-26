# DEPENDENCIES

ArcOracle Backend relies on the following Rust crates and external services to deliver its functionality:

## RUST CRATES

- **RIG**: 0.1.0
  - Framework for LLM integration and vector storage, powering ArcOracle’s AI capabilities.
- **REQWEST**: 0.11
  - HTTP client for API requests to Together AI and @arcdotfun.
- **TOKIO**: 1.0
  - Async runtime for non-blocking operations.
- **SOLANA-SDK**: 1.18
  - Solana blockchain SDK for on-chain interactions.
- **SOLANA-CLIENT**: 1.18
  - Solana RPC client for blockchain queries.
- **SERDE_JSON**: 1.0
  - JSON serialization/deserialization for data handling.
- **DOTENV**: 0.15
  - Environment variable management for API keys.
- **CHRONO**: 0.4
  - Timestamp handling for Solana data.
- **SERDE**: 1.0
  - Serialization/deserialization support for Rust structs.
- **ASYNC-TRAIT**: 0.1
  - Async trait implementation for data and vector storage.
- **LOG**: 0.4
  - Logging framework for debugging and monitoring.
- **ENV_LOGGER**: 0.10
  - Logging setup for ArcOracle Backend.
- **THISERROR**: 1.0
  - Custom error handling for robust error management.

## EXTERNAL SERVICES

- **TOGETHER AI**: Provides access to DeepSeek-R1 and other LLMs for data analysis (requires API key in `.env`).
- **@ARCDOTFUN**: Data source for real-time Solana prices and social sentiment (requires API integration).
- **SOLANA BLOCKCHAIN**: Mainnet, Testnet, and Devnet for on-chain data storage and transactions.

## VERSION COMPATIBILITY

Ensure all dependencies are compatible with Rust 2021 edition and Rig 0.1.0. Regularly update `Cargo.toml` to align with the latest releases and security patches.

## MANAGING DEPENDENCIES

- Install dependencies via `cargo build` after cloning the repository (if applicable).
- Update `.env` with `TOGETHER_API_KEY` for Together AI access.
- Use `cargo update` to refresh dependencies as needed.