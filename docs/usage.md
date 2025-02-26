# USAGE

## RUNNING THE BACKEND

The ArcOracle Backend powers core oracle functionality, processing Solana data and generating insights. Follow these steps (assuming you have the backend set up):

1. Ensure `.env` is configured with your `TOGETHER_API_KEY`.
2. Navigate to the backend directory:
   ```bash
   cd path/to/arcoracle/backend
   ```
3. Run the backend:
   ```bash
   cargo run
   ```

### EXAMPLE OUTPUT

```
ArcOracle Insight: Solana price at $68,420.69 indicates strong market interest for DeFi applications, with positive sentiment.
Storing data on Solana at Pubkey: 5V5F7wP6X5JDwZ9qZQT5y8A9wTu6i8bY8Yv7nL6kL2m
```

## KEY OPERATIONS

- **DATA FETCHING**: Retrieves Solana prices and social sentiment from @arcdotfun.
- **LLM ANALYSIS**: Uses Together AI’s DeepSeek-R1 to analyze data, providing actionable DeFi insights.
- **SOLANA STORAGE**: Simulates on-chain data storage for token and data operations.
- **VECTOR STORAGE**: Stores data in LanceDB (future integration) for efficient retrieval.

## EXTENDING USAGE

- Update `src/data/arcdotfun.rs` to integrate real @arcdotfun API endpoints.
- Enhance `src/llm/agent.rs` for custom LLM prompts or additional Together AI models.
- Expand `src/solana/storage.rs` for full Solana transaction support in future phases (e.g., Q2 2025 onward).