# 缓存标识功能说明

## 概述

价格响应中新增 `cached` 字段，用于标识数据是来自缓存还是实时 API 调用。

## 响应格式

### 单个价格响应

```json
{
  "success": true,
  "data": {
    "symbol": "SOLUSDT",
    "price": 150.23,
    "exchange": "binance",
    "timestamp": 1696118400000,
    "cached": true
  },
  "error": null
}
```

### 多个价格响应

```json
{
  "success": true,
  "data": [
    {
      "symbol": "BTCUSDT",
      "price": 65432.10,
      "exchange": "binance",
      "timestamp": 1696118400000,
      "cached": true
    },
    {
      "symbol": "BTCUSDT",
      "price": 65430.50,
      "exchange": "bitget",
      "timestamp": 1696118400000,
      "cached": false
    }
  ],
  "error": null
}
```

## cached 字段说明

| 值 | 说明 |
|---|---|
| `true` | 数据来自缓存（10秒内的历史数据） |
| `false` | 数据来自实时 API 调用（最新数据） |

## 使用场景

### 1. 前端显示缓存状态

```typescript
interface Price {
  symbol: string;
  price: number;
  exchange: string;
  timestamp: number;
  cached: boolean;
}

function renderPrice(price: Price) {
  return (
    <div>
      <span>{price.symbol}: ${price.price}</span>
      {price.cached && <Badge>缓存</Badge>}
    </div>
  );
}
```

### 2. 性能监控

```typescript
function trackCacheHitRate(prices: Price[]) {
  const cachedCount = prices.filter(p => p.cached).length;
  const hitRate = (cachedCount / prices.length) * 100;
  console.log(`缓存命中率: ${hitRate}%`);
}
```

### 3. 数据新鲜度判断

```typescript
function isDataFresh(price: Price): boolean {
  if (price.cached) {
    // 缓存数据可能有最多 10 秒延迟
    return Date.now() - price.timestamp < 10000;
  }
  return true; // 实时数据总是新鲜的
}
```

## 测试示例

### 测试 1: 首次查询（无缓存）

**请求**:
```bash
curl "http://localhost:13012/api/v1/price?base=SOL&quote=USDT"
```

**响应**:
```json
{
  "success": true,
  "data": [
    {
      "symbol": "SOLUSDT",
      "price": 150.23,
      "exchange": "binance",
      "timestamp": 1696118400000,
      "cached": false   ← 实时 API 数据
    },
    {
      "symbol": "SOLUSDT",
      "price": 150.25,
      "exchange": "bitget",
      "timestamp": 1696118400100,
      "cached": false   ← 实时 API 数据
    }
  ],
  "error": null
}
```

**响应时间**: ~2300ms（调用所有 API）

### 测试 2: 10秒内重复查询（缓存命中）

**请求**:
```bash
# 立即重复上一次查询
curl "http://localhost:13012/api/v1/price?base=SOL&quote=USDT"
```

**响应**:
```json
{
  "success": true,
  "data": [
    {
      "symbol": "SOLUSDT",
      "price": 150.23,
      "exchange": "binance",
      "timestamp": 1696118400000,
      "cached": true    ← 来自缓存
    },
    {
      "symbol": "SOLUSDT",
      "price": 150.25,
      "exchange": "bitget",
      "timestamp": 1696118400100,
      "cached": true    ← 来自缓存
    }
  ],
  "error": null
}
```

**响应时间**: ~130ms（从缓存读取，快 ~95%）

### 测试 3: 10秒后查询（缓存过期）

**请求**:
```bash
# 等待 11 秒
sleep 11
curl "http://localhost:13012/api/v1/price?base=SOL&quote=USDT"
```

**响应**:
```json
{
  "success": true,
  "data": [
    {
      "symbol": "SOLUSDT",
      "price": 150.45,
      "exchange": "binance",
      "timestamp": 1696118411000,
      "cached": false   ← 缓存过期，重新调用 API
    },
    {
      "symbol": "SOLUSDT",
      "price": 150.47,
      "exchange": "bitget",
      "timestamp": 1696118411100,
      "cached": false   ← 缓存过期，重新调用 API
    }
  ],
  "error": null
}
```

**响应时间**: ~2400ms（重新调用所有 API）

## 实际日志分析

根据实际运行日志：

```
# 首次查询（cached: false）
2025-10-01T02:34:04.112428Z  started processing request
2025-10-01T02:34:06.426836Z  finished processing request latency=2314 ms

# 第二次查询（cached: true）
2025-10-01T02:34:07.259365Z  started processing request
2025-10-01T02:34:07.471630Z  finished processing request latency=212 ms  ← 快 91%

# 第三次查询（cached: true）
2025-10-01T02:34:08.223905Z  started processing request
2025-10-01T02:34:08.360418Z  finished processing request latency=136 ms  ← 快 94%

# 持续查询（cached: true）
latency=132 ms, 133 ms, 136 ms, 131 ms, 125 ms, 135 ms, 138 ms, 139 ms...
平均响应时间: ~135ms

# 缓存过期后（cached: false）
2025-10-01T02:39:43.298300Z  started processing request
2025-10-01T02:39:45.776540Z  finished processing request latency=2478 ms  ← 重新调用 API
```

## 性能对比

| 场景 | cached | 响应时间 | 性能提升 |
|------|--------|---------|---------|
| 首次查询 | `false` | ~2300ms | - |
| 缓存命中 | `true` | ~130ms | **~94%** |
| 缓存过期 | `false` | ~2400ms | - |

## 前端实现建议

### 1. 视觉区分

```tsx
function PriceCard({ price }: { price: Price }) {
  return (
    <Card className={price.cached ? 'cached' : 'live'}>
      <div className="price-value">${price.price}</div>
      <div className="price-meta">
        {price.cached ? (
          <Badge variant="secondary">
            <ClockIcon /> 缓存
          </Badge>
        ) : (
          <Badge variant="success">
            <LiveIcon /> 实时
          </Badge>
        )}
      </div>
    </Card>
  );
}
```

### 2. 自动刷新策略

```typescript
function usePriceData(symbol: string) {
  const [data, setData] = useState<Price[]>([]);
  
  useEffect(() => {
    const fetchPrice = async () => {
      const response = await fetch(`/api/v1/price?symbol=${symbol}`);
      const result = await response.json();
      setData(result.data);
      
      // 如果数据来自缓存，缩短刷新间隔
      const hasCached = result.data.some((p: Price) => p.cached);
      const interval = hasCached ? 5000 : 15000; // 5秒 vs 15秒
      
      setTimeout(fetchPrice, interval);
    };
    
    fetchPrice();
  }, [symbol]);
  
  return data;
}
```

### 3. 警告用户

```typescript
function DataFreshnessWarning({ prices }: { prices: Price[] }) {
  const allCached = prices.every(p => p.cached);
  const oldestTimestamp = Math.min(...prices.map(p => p.timestamp));
  const age = Date.now() - oldestTimestamp;
  
  if (allCached && age > 8000) {
    return (
      <Alert variant="warning">
        价格数据可能有 {Math.round(age / 1000)} 秒延迟
      </Alert>
    );
  }
  
  return null;
}
```

## 技术实现

### 数据结构

```rust
// models/price.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    pub symbol: String,
    pub price: f64,
    pub exchange: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub timestamp: DateTime<Utc>,
    #[serde(default)]
    pub cached: bool,  // 新增字段
}
```

### 缓存逻辑

```rust
// cache.rs
pub async fn get(&self, exchange: &str, symbol: &str) -> Option<Price> {
    let key = Self::make_key(exchange, symbol);
    self.cache.get(&key).await.map(|arc| {
        let mut price = (*arc).clone();
        price.cached = true;  // 从缓存返回时标记为 true
        price
    })
}
```

### API 客户端

```rust
// services/binance.rs
Ok(Price {
    symbol: ticker.symbol,
    price: price_value,
    exchange: self.name().to_string(),
    timestamp: Utc::now(),
    cached: false,  // API 返回的数据标记为 false
})
```

## 总结

✅ **cached 字段已添加**：所有价格响应都包含缓存标识  
✅ **自动标记**：缓存返回 `true`，API 返回 `false`  
✅ **性能提升可见**：前端可以显示数据来源  
✅ **向后兼容**：使用 `#[serde(default)]`，旧客户端不受影响  

前端现在可以根据 `cached` 字段来：
- 显示数据来源（缓存/实时）
- 调整刷新策略
- 提示用户数据新鲜度
- 监控缓存命中率
