

1. Token Discovery & Analysis:
   - Monitor Jupiter's `/v6/new-tokens` endpoint
     - Filter for pump.fun tokens
     - Track `knownMarkets` array length
     - Cache results to handle the 6mb payload efficiently
   - Analyze potential targets:
     - Targeting MCap <$500k
     - Check initial liquidity metrics
     - Monitor price momentum
2. Trading Strategy:
```rust
struct Strategy {
    // Entry criteria
    max_mcap: u64, // 500k USD
    min_liquidity: u64, // Minimum required liquidity
    
    // Exit points
    take_profit: f64, // 5-10x initial MCap
    moon_bag_pct: f64, // 20-30% position to keep.
    stop_loss: f64, // Initial stop loss %.
    
    // Risk Management
    position_size: f64, // Position size per trade
    max_concurrent: u8, // Max number of active trades
    daily_limit: f64, // Maximum daily risk amount
}
```

3. Trade Execution:
   - Use the Jupiter quote API for best routes
   - Implement fast execution with retries
   - Track transaction status and confirmations
   - Handle partial exits and position scaling.
4. CLI Interface:
Starting with:
```shell
# Core Commands
smt scan                  # Start scanning for new tokens
smt buy <TOKEN> <AMOUNT>  # Execute buy with amount in SOL
smt sell <TOKEN> <AMOUNT> # Execute sell with token amount
smt status               # Show active positions & PnL

# Configuration
smt config set mcap 500000  # Set maximum market cap
smt config set profit 5.0   # Set take profit multiple
```

