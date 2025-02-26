# ArcOracle Backend

The ArcOracle Backend is the robust, high-performance core of the ArcOracle decentralized oracle solution, engineered to deliver reliable, real-time data insights to the Solana blockchain. Built with Rust and powered by the Rig 0.1.0 framework, this backend integrates the Together AI API—defaulting to DeepSeek-R1—to process and analyze data, leveraging the @arcdotfun framework for data sourcing. This implementation aligns with ArcOracle’s Q1 2025 roadmap phase (Foundation & Token Launch), establishing a foundational framework for future scalability and decentralization.

## Project Overview

ArcOracle Backend is designed to empower the Solana ecosystem with precise, timely data for DeFi, Web3, and beyond. Key features include:

- **Real-Time Data Acquisition**: Fetch and process data (e.g., Solana price feeds, social sentiment) from the @arcdotfun framework.
- **Advanced AI Insights**: Utilize Together AI’s DeepSeek-R1 model for sophisticated data analysis, generating actionable insights for DeFi users.
- **Solana Integration**: Seamlessly store and manage data on the Solana blockchain, ensuring decentralization and immutability.
- **Vector Storage**: Support for vector-based data storage (e.g., LanceDB, to be fully integrated in future phases) for efficient retrieval and analysis.
- **Modular Architecture**: Built for scalability, enabling expansion from prototype development to full decentralization as per the ArcOracle roadmap.

## Project Structure

The backend is meticulously organized into modular Rust crates, adhering to best practices for maintainability and extensibility:

```
backend/
├── Cargo.toml                    # Rust project configuration and dependencies
├── src/                          # Source code for Rust backend
│   ├── main.rs                   # Entry point for the Rust application
│   ├── data/                     # Data-related modules (e.g., fetching, storage)
│   │   ├── arcdotfun.rs          # Interaction with @arcdotfun data source
│   │   ├── solana_data.rs        # Solana-specific data handling
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

## Getting Started

### Prerequisites
- **Rust & Cargo**: Install Rust via `rustup` (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y`).
- **Git**: Ensure Git is installed (`git --version`).
- **Together AI API Key**: Obtain a key from [Together AI](https://www.together.ai).

### Installation
1. Clone the ArcOracle repository or run the provided setup script:
   ```bash
   chmod +x setup_arcoracle.sh
   ./setup_arcoracle.sh
   ```
   - The script will prompt for a project path (defaults to `./arcoracle`) and set up the backend structure, initialize dependencies, and generate files.

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
   - This will fetch mock Solana price data, process it with DeepSeek-R1 via Together AI, generate DeFi insights, and simulate storage on Solana, outputting results to the console.

### Example Output
```
ArcOracle Insight: Solana price at $68,420.69 indicates strong market interest for DeFi applications, with positive sentiment.
Storing data on Solana at Pubkey: 5V5F7wP6X5JDwZ9qZQT5y8A9wTu6i8bY8Yv7nL6kL2m
```

## Dependencies
- `rig`: 0.1.0 (for LLM integration and vector storage)
- `reqwest`: 0.11 (for HTTP requests to Together AI)
- `tokio`: 1.0 (for async operations)
- `solana-sdk`: 1.18 (for Solana blockchain interaction)
- `solana-client`: 1.18 (for Solana RPC)
- `serde_json`: 1.0 (for JSON handling)
- `dotenv`: 0.15 (for environment variable management)
- `chrono`: 0.4 (for timestamp handling)
- `serde`: 1.0 (for serialization/deserialization)
- `async-trait`: 0.1 (for async traits)
- `log`: 0.4 (for logging)
- `env_logger`: 0.10 (for logging setup)
- `thiserror`: 1.0 (for custom error handling)

## Architecture

### Data Flow
1. **Data Acquisition**: `data/arcdotfun.rs` fetches real-time data (e.g., Solana prices, social sentiment) from the @arcdotfun framework (simulated for now).
2. **LLM Processing**: `llm/agent.rs` and `llm/together_ai.rs` leverage Together AI’s DeepSeek-R1 to analyze data, generating concise insights for DeFi users.
3. **Vector Storage**: `vector/lancedb.rs` stores data in a vector database (placeholder for LanceDB integration in future phases).
4. **Solana Integration**: `solana/storage.rs`, `solana/transactions.rs`, and `solana/client.rs` manage on-chain data storage and transactions on Solana.

### Modularity & Scalability
The backend’s modular design facilitates expansion across ArcOracle’s roadmap:
- **Q2 2025 (Prototype Development)**: Integrate real @arcdotfun data and test on Solana Devnet.
- **Q3 2025 (MVP & Testnet Launch)**: Deploy an MVP on Solana Testnet with full vector storage and security audits.
- **Q4 2025 (Mainnet Launch & Expansion)**: Launch on Solana Mainnet, partner with DeFi projects.
- **Q1 2026 (Decentralization & Growth)**: Transition to a decentralized multi-node network, expanding data types and governance.

## Contributing

We welcome contributions to enhance ArcOracle Backend! To contribute:

1. Fork the repository at [GitHub](https://github.com/aiArcOracle/oracle).
2. Create a branch for your feature or bug fix (`git checkout -b feature/your-feature`).
3. Implement changes, adhering to Rust best practices and this README.
4. Write tests and documentation as needed.
5. Submit a pull request with detailed descriptions and references to issues.

Please ensure your code follows the Rust coding guidelines and includes appropriate tests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details (to be added).

## Community & Support

- **GitHub**: [https://github.com/aiArcOracle/oracle](https://github.com/aiArcOracle/oracle)
- **Twitter**: [https://x.com/aiArcOracle](https://x.com/aiArcOracle)
- **Discord**: Join our community at [ArcOracle Discord](https://discord.gg/arcoracle) for discussions and support.
- **Email**: Contact us at `contact@arcoracle.io` for inquiries.

Unveil the Cosmos of Data with ArcOracle—built on Solana and @arcdotfun, driving the future of Web3 oracles!
