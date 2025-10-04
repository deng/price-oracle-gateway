# API 测试示例

## 测试价格查询 API

### 方式 1：使用 symbol 参数

```bash
# 查询 BTC/USDT 价格（使用 symbol）
curl "http://localhost:13012/api/v1/price?symbol=BTCUSDT"

# 指定交易所
curl "http://localhost:13012/api/v1/price?symbol=BTCUSDT&exchange=binance"

# 查询 ETH/USDT 价格
curl "http://localhost:13012/api/v1/price?symbol=ETHUSDT"
```

### 方式 2：使用 base 和 quote 参数

```bash
# 查询 BTC/USDT 价格（使用 base 和 quote）
curl "http://localhost:13012/api/v1/price?base=BTC&quote=USDT"

# 指定交易所
curl "http://localhost:13012/api/v1/price?base=BTC&quote=USDT&exchange=binance"

# 查询 ETH/USDT 价格
curl "http://localhost:13012/api/v1/price?base=ETH&quote=USDT"

# 查询 BTC/EUR 价格
curl "http://localhost:13012/api/v1/price?base=BTC&quote=EUR"
```

### Bitget 交易所测试

```bash
# 从 Bitget 查询 BTC/USDT 价格
curl "http://localhost:13012/api/v1/price?symbol=BTCUSDT&exchange=bitget"

# 使用 base 和 quote 参数
curl "http://localhost:13012/api/v1/price?base=BTC&quote=USDT&exchange=bitget"

# 查询 ETH/USDT 价格
curl "http://localhost:13012/api/v1/price?base=ETH&quote=USDT&exchange=bitget"

# 查询 SOL/USDT 价格
curl "http://localhost:13012/api/v1/price?symbol=SOLUSDT&exchange=bitget"
```

### API Ninjas 数据源测试

```bash
# 从 API Ninjas 查询 BTC/USD 价格
curl "http://localhost:13012/api/v1/price?symbol=BTCUSD&exchange=apininjas"

# 使用 base 和 quote 参数
curl "http://localhost:13012/api/v1/price?base=BTC&quote=USD&exchange=apininjas"

# 查询 ETH/USD 价格
curl "http://localhost:13012/api/v1/price?base=ETH&quote=USD&exchange=apininjas"

# 查询 LTC/USD 价格
curl "http://localhost:13012/api/v1/price?base=LTC&quote=USD&exchange=apininjas"

# 从所有数据源查询 BTC/USD（会同时返回 Binance 和 API Ninjas 的价格）
curl "http://localhost:13012/api/v1/price?symbol=BTCUSD"
```

### 错误情况测试

```bash
# 未提供任何参数（应该返回错误）
curl "http://localhost:13012/api/v1/price"

# 只提供 base 没有 quote（应该返回错误）
curl "http://localhost:13012/api/v1/price?base=BTC"

# 只提供 quote 没有 base（应该返回错误）
curl "http://localhost:13012/api/v1/price?quote=USDT"
```

### 健康检查

```bash
# 健康检查
curl "http://localhost:13012/health"
```

## 预期响应示例

### 成功响应

```json
{
  "success": true,
  "data": {
    "symbol": "BTCUSDT",
    "price": 43250.50,
    "exchange": "binance",
    "timestamp": 1696161600000
  }
}
```

### 错误响应

```json
{
  "success": false,
  "error": "Either 'symbol' or both 'base' and 'quote' must be provided"
}
```
