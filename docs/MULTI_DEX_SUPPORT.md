# Multi-DEX Support for DexScreener

## Overview
The gateway service now supports detecting and selecting from multiple DEXs (Decentralized Exchanges) when querying prices through DexScreener.

## How It Works

### 1. Detection
When a trading pair exists on multiple DEXs, the system:
- Searches DexScreener API for all matching trading pairs
- Filters pairs by base and quote token symbols
- Collects all matching DEXs

### 2. Selection Strategy
The system selects the **best DEX based on liquidity**:
- Compares USD liquidity across all found DEXs
- Returns the trading pair with the **highest USD liquidity**
- This ensures better price accuracy and lower slippage

### 3. Logging
When multiple DEXs are found, the system logs all available DEXs:
```
Found 18 DEXs for WETH/USDT: etherex, quickswap, ambient, merchantmoe, lynex, ...
```

## Example Queries

### WETH/USDT
```bash
curl "http://localhost:13012/api/v1/price?base=WETH&quote=USDT"
```

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "symbol": "WETHUSDT",
      "price": 4158.3519,
      "exchange": "dexscreener:etherex",
      "timestamp": 1759287906096,
      "cached": false
    }
  ]
}
```

**Log Output:**
```
Found 18 DEXs for WETH/USDT: etherex, quickswap, ambient, merchantmoe, lynex, 
syncswap, ocelex, wagmi, raydium, kyo-finance, vvsfinance, ...
```

### ASTER/USDT
```bash
curl "http://localhost:13012/api/v1/price?base=ASTER&quote=USDT"
```

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "symbol": "ASTERUSDT",
      "price": 1.6173,
      "exchange": "dexscreener:pancakeswap",
      "timestamp": 1759287921289,
      "cached": false
    }
  ]
}
```

**Log Output:**
```
Found 12 DEXs for ASTER/USDT: pancakeswap, uniswap, omni-exchange, ...
```

## Technical Implementation

### Data Structure
```rust
#[derive(Debug, Deserialize)]
struct DexPair {
    dex_id: String,
    base_token: Token,
    quote_token: Token,
    price_native: String,
    liquidity: Option<Liquidity>,
}

#[derive(Debug, Deserialize)]
struct Liquidity {
    usd: Option<f64>,
}
```

### Selection Algorithm
```rust
// Collect all matching pairs
let matching_pairs: Vec<&DexPair> = pairs
    .iter()
    .filter(|pair| {
        pair.base_token.symbol.to_uppercase() == base_upper
            && pair.quote_token.symbol.to_uppercase() == quote_upper
    })
    .collect();

// Log if multiple DEXs found
if matching_pairs.len() > 1 {
    tracing::info!(
        "Found {} DEXs for {}/{}: {}",
        matching_pairs.len(),
        base,
        quote,
        dex_names.join(", ")
    );
}

// Return highest liquidity pair
matching_pairs.into_iter()
    .max_by(|a, b| {
        let a_liquidity = a.liquidity.as_ref().and_then(|l| l.usd).unwrap_or(0.0);
        let b_liquidity = b.liquidity.as_ref().and_then(|l| l.usd).unwrap_or(0.0);
        a_liquidity.partial_cmp(&b_liquidity).unwrap_or(std::cmp::Ordering::Equal)
    })
```

## Benefits

1. **Better Price Discovery**: Selects the most liquid market for accurate pricing
2. **Transparency**: Logs all available DEXs for monitoring and debugging
3. **Automatic Selection**: No need to manually specify which DEX to use
4. **Lower Slippage**: Higher liquidity typically means better execution prices

## Caching Behavior
- Multi-DEX queries are cached using the unified "dexscreener" key
- The specific DEX name (e.g., "dexscreener:pancakeswap") is preserved in responses
- Cache TTL is 10 seconds for all price queries
- See [CACHE_IMPLEMENTATION.md](./CACHE_IMPLEMENTATION.md) for details

## Related Documentation
- [DEXSCREENER_INTEGRATION.md](./DEXSCREENER_INTEGRATION.md) - DexScreener integration details
- [CACHE_IMPLEMENTATION.md](./CACHE_IMPLEMENTATION.md) - Caching mechanism
- [CACHED_FLAG.md](./CACHED_FLAG.md) - Cache indicator in responses
