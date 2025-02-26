# ARCHITECTURE

## OVERVIEW

ArcOracle’s backend architecture is a high-performance, modular system designed to deliver real-time, decentralized data insights to the Solana blockchain. It integrates the Rig 0.1.0 framework, Together AI’s DeepSeek-R1, and the @arcdotfun framework, ensuring scalability, security, and decentralization across our roadmap.

## COMPONENTS

### DATA LAYER
- **SOURCE**: `data/arcdotfun.rs` fetches real-time Solana prices, social sentiment, and future advanced data (e.g., weather, AI predictions) from @arcdotfun.
- **STRUCTURE**: `data/solana_data.rs` models Solana data, while `data/data_store.rs` provides a generic storage interface for scalability.
- **EVOLUTION**:
  - Q1 2025: Initial data for foundation and token launch.
  - Q2 2025: Prototype with real @arcdotfun data.
  - Q3 2025: Expanded data for MVP.
  - Q4 2025: Scaled data for Mainnet with DeFi partnerships.
  - Q1 2026: Advanced data types for decentralization.

### LLM LAYER
- **PROCESSING**: `llm/agent.rs` and `llm/together_ai.rs` leverage Together AI’s DeepSeek-R1 for data analysis, generating DeFi insights.
- **SCALABILITY**: Supports multi-node LLM processing in Q1 2026 for decentralized operations.
- **EVOLUTION**:
  - Q1 2025: Basic insights for foundation.
  - Q2 2025: Testing on Solana Devnet.
  - Q3 2025: Security audits for MVP reliability.
  - Q4 2025: Optimized for Mainnet performance.
  - Q1 2026: Multi-node, low-latency processing with governance.

### SOLANA LAYER
- **INTEGRATION**: `solana/storage.rs`, `solana/transactions.rs`, and `solana/client.rs` manage on-chain data storage, transactions, and RPC interactions on Solana.
- **PHASES**:
  - Q1 2025: Foundation for token launch and data storage.
  - Q2 2025: Prototype on Solana Devnet.
  - Q3 2025: MVP on Solana Testnet with beta testing.
  - Q4 2025: Mainnet deployment with DeFi ecosystem integration.
  - Q1 2026: Decentralized multi-node architecture with governance and cross-chain compatibility.

### VECTOR STORAGE LAYER
- **STORAGE**: `vector/lancedb.rs` and `vector/store.rs` implement LanceDB for efficient data retrieval, planned for full integration in Q2 2025.
- **FUTURE**: Scales for multi-node and cross-chain data in Q1 2026, supporting advanced queries.

### UTILITIES LAYER
- **SUPPORT**: `utils/config.rs`, `utils/logging.rs`, and `utils/error.rs` provide configuration, logging, and error management.
- **SCALABILITY**: Supports all phases, ensuring robust operation and debugging across the roadmap.

## DATA FLOW

1. **DATA ACQUISITION**: `data/arcdotfun.rs` retrieves data from @arcdotfun.
2. **LLM ANALYSIS**: `llm/agent.rs` processes data with Together AI, storing insights in `vector/lancedb.rs`.
3. **SOLANA INTEGRATION**: `solana/storage.rs` stores processed data on Solana, managed by utilities.
4. **DECENTRALIZATION**: Q1 2026 introduces multi-node architecture, cross-chain compatibility (e.g., Ethereum), governance, and advanced data types.

## SCALABILITY & FUTURE EXPANSION

- **Q2 2025**: Prototype real @arcdotfun data and Solana Devnet testing.
- **Q3 2025**: MVP on Solana Testnet with security audits and vector storage optimization.
- **Q4 2025**: Mainnet launch, DeFi partnerships, and ecosystem integration.
- **Q1 2026**: Decentralized multi-node network, cross-chain compatibility (e.g., Ethereum), governance, and advanced data types.