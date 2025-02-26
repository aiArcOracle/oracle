# ArcOracle Backend

Welcome to the ArcOracle Backend, the core data processing and integration layer for the ArcOracle project. Built with Rust and powered by the Rig 0.1.0 framework, this backend leverages the Together AI API (defaulting to DeepSeek-R1) to provide real-time data insights for the Solana blockchain, using the @arcdotfun framework for data sourcing. This module aligns with ArcOracle’s Q1 2025 roadmap phase (Foundation & Token Launch), laying the foundation for decentralized oracle functionality.

## Overview

ArcOracle Backend is designed to:
- Fetch and process real-time data (e.g., Solana price feeds, X sentiment) via the @arcdotfun framework.
- Analyze data using large language models (LLM) through Together AI’s API, defaulting to DeepSeek-R1 for concise, accurate insights.
- Store and manage data using vector storage (e.g., LanceDB, planned for future integration).
- Interact with the Solana blockchain for on-chain data storage and transactions.
- Support modular expansion for prototype development, MVP launch, and decentralization as outlined in the ArcOracle roadmap.

## Project Structure

The backend is organized into modular Rust crates for maintainability and scalability:

```
backend/
├── Cargo.toml                    # Rust project configuration and dependencies
├── src/                          # Source code for Rust backend
│   ├── main.rs                   # Entry point for the Rust application
│   ├── data/                     # Data-related modules (e.g., fetching, storage)
│   │   ├── arcdotfun.rs          # Module for interacting with @arcdotfun data source
│   │   ├── solana_data.rs        # Module for Solana-specific data handling
│   │   └── data_store.rs         # General data storage logic
│   ├── llm/                      # LLM-related modules (using Rig and Together AI)
│   │   ├── agent.rs              # LLM agent configuration and logic
│   │   └── together_ai.rs        # Together AI provider implementation
│   ├── solana/                   # Solana blockchain integration
│   │   ├── storage.rs            # Solana on-chain data storage
│   │   ├── transactions.rs       # Solana transaction handling
│   │   └── client.rs             # Solana RPC client interaction
│   ├── vector/                   # Vector storage modules (placeholder for future)
│   │   ├── lancedb.rs            # LanceDB vector storage implementation (optional)
│   │   └── store.rs              # Vector store interface
│   ├── utils/                    # Utility functions
│   │   ├── config.rs             # Configuration management (e.g., API keys)
│   │   ├── logging.rs            # Logging utilities
│   │   └── error.rs              # Custom error handling
│   └── lib.rs                    # Library entry point for the backend
├── .env                          # Environment variables (API keys, etc.)
├── .gitignore                    # Git ignore file for backend
└── README.md                     # This documentation
```

## Setup

### Prerequisites
- **Rust**: Ensure Rust and Cargo are installed (`rustc --version` and `cargo --version`).
- **Git**: Required for version control (`git --version`).
- **Together AI API Key**: Obtain a key from [Together AI](https://www.together.ai).

### Installation
1. Clone or create the project using the provided `setup_arcoracle.sh` script:
   ```bash
   chmod +x setup_arcoracle.sh
   ./setup_arcoracle.sh
   ```
   - The script will prompt for a project path (defaults to `./arcoracle`).
   - It sets up the backend structure, initializes dependencies, and generates files.

2. Navigate to the backend directory:
   ```bash
   cd arcoracle/backend
   ```

3. Install dependencies:
   ```bash
   cargo build
   ```

4. Configure environment variables:
   - Update `.env` with your Together AI API key:
     ```env
     TOGETHER_API_KEY=your_together_ai_api_key
     ```

### Running the Backend
1. Ensure `.env` is configured with your API key.
2. Run the backend:
   ```bash
   cargo run
   ```
   - This will fetch Solana price data, process it with DeepSeek-R1 via Together AI, and simulate storage on Solana, printing insights and transaction details.

## Usage

The backend currently simulates ArcOracle’s core functionality:
- Fetches mock Solana price data (via `data/arcdotfun.rs`).
- Uses Together AI’s DeepSeek-R1 model to analyze data and generate DeFi insights (via `llm/agent.rs` and `llm/together_ai.rs`).
- Simulates Solana on-chain storage (via `solana/storage.rs`).
- Stores data in a vector store (via `vector/lancedb.rs`, placeholder for future integration).

### Example Output
Running `cargo run` might produce:
```
ArcOracle Insight: Solana price at $68,420.69 indicates strong market interest for DeFi applications.
Storing data on Solana at Pubkey: 5V5F7wP6X5JDwZ9qZQT5y8A9wTu6i8bY8Yv7nL6kL2m
```

## Dependencies
- `rig`: 0.1.0 (for LLM integration and vector storage)
- `reqwest`: 0.11 (for HTTP requests to Together AI)
- `tokio`: 1.0 (for async operations)
- `solana-sdk`: 1.18 (for Solana blockchain interaction)
- `serde_json`: 1.0 (for JSON handling)
- `dotenv`: 0.15 (for environment variable management)

## Architecture

### Data Flow
1. **Data Fetching**: `data/arcdotfun.rs` retrieves real-time data (e.g., Solana prices) from the @arcdotfun framework (simulated here).
2. **LLM Analysis**: `llm/agent.rs` and `llm/together_ai.rs` use Together AI’s DeepSeek-R1 to analyze data and generate insights.
3. **Vector Storage**: `vector/lancedb.rs` stores data in a vector database (placeholder for LanceDB integration).
4. **Solana Integration**: `solana/storage.rs` and related modules simulate on-chain data storage and transactions.

### Modularity
The backend is structured for scalability, with separate modules for each functionality, allowing easy expansion for:
- Prototype development (Q2 2025).
- MVP and Testnet launch (Q3 2025).
- Mainnet deployment and decentralization (Q4 2025 - Q1 2026).

## Roadmap Alignment
This backend implementation supports ArcOracle’s Q1 2025 phase (Foundation & Token Launch) by:
- Establishing a foundational framework for data processing and LLM integration.
- Simulating token and data interactions on Solana.
- Laying the groundwork for community engagement (e.g., via Twitter announcements).

Future phases will expand this framework:
- **Q2 2025**: Develop a prototype with real @arcdotfun data and Solana Devnet testing.
- **Q3 2025**: Launch an MVP on Solana Testnet with full vector storage and security audits.
- **Q4 2025**: Deploy to Solana Mainnet, integrate DeFi partnerships.
- **Q1 2026**: Transition to a decentralized multi-node network.

## Contributing
Contributions are welcome! Please follow these steps:
1. Fork the repository.
2. Create a branch for your feature or bug fix.
3. Submit a pull request with detailed changes and tests.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details (to be added).

## Contact
- Twitter: [@ArcOracle](https://x.com/aiArcOracle)

Unveil the Cosmos of Data with ArcOracle—built on Solana and @arcdotfun!
