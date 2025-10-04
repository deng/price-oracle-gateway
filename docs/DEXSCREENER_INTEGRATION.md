# DexScreener 集成文档

## 概述

DexScreener 是一个去中心化交易所 (DEX) 聚合器，提供跨多个区块链和 DEX 的代币价格数据。

本服务集成了 DexScreener API，支持查询去中心化交易所中的代币价格。

## API 接口

### 端点
```
GET /api/v1/price
```

### 查询参数

支持两种查询方式：

#### 方式 1: 使用 base 和 quote 参数
```bash
curl "http://localhost:13012/api/v1/price?base=ASTER&quote=USDT"
```

#### 方式 2: 使用 symbol 参数
```bash
# DexScreener 会自动解析常见的交易对格式
curl "http://localhost:13012/api/v1/price?symbol=ASTER/USDT"
curl "http://localhost:13012/api/v1/price?symbol=ASTERUSDT"
```

**注意**: DexScreener 需要使用 `/` 分隔符或能自动识别的后缀（USDT, USDC, BNB, ETH）

## 响应格式

### 成功响应
```json
{
  "symbol": "ASTER/USDT",
  "price": 1.6128,
  "exchange": "dexscreener:pancakeswap",
  "timestamp": "2025-10-01T02:24:15.123456Z"
}
```

### 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `symbol` | string | 交易对符号，格式：BASE/QUOTE |
| `price` | number | 价格（以 priceNative 为准） |
| `exchange` | string | 格式：`dexscreener:{dexId}`，如 `dexscreener:pancakeswap` |
| `timestamp` | string | ISO 8601 格式的时间戳 |

### 多数据源响应

当启用多个数据源时，会返回所有数据源的价格：

```json
{
  "prices": [
    {
      "symbol": "BTCUSDT",
      "price": 65432.10,
      "exchange": "binance",
      "timestamp": "2025-10-01T02:24:15.123Z"
    },
    {
      "symbol": "BTCUSDT",
      "price": 65430.50,
      "exchange": "bitget",
      "timestamp": "2025-10-01T02:24:15.456Z"
    }
  ]
}
```

## DexScreener 特性

### 1. 多交易对匹配

DexScreener 的搜索接口可能返回多个匹配的交易对。本服务会：
- 精确匹配 `baseToken.symbol` 和 `quoteToken.symbol`（不区分大小写）
- 返回第一个完全匹配的交易对
- 如果没有匹配项，返回错误

### 2. 价格字段

- 使用 `priceNative` 作为价格字段（原生计价货币价格）
- 也可以获取 `priceUsd`（美元价格），但当前实现使用 `priceNative`

### 3. DEX 信息

- `exchange` 字段包含 DEX 的 ID，如 `pancakeswap`、`uniswap` 等
- 格式：`dexscreener:{dexId}`

### 4. 支持的链

DexScreener 支持多条区块链，包括：
- BSC (Binance Smart Chain)
- Ethereum
- Polygon
- Avalanche
- Arbitrum
- Optimism
- Base
- 以及更多...

## 配置

### 环境变量配置

在 `.env` 文件中添加：

```bash
APP__EXCHANGES__DEXSCREENER__ENABLED=true
APP__EXCHANGES__DEXSCREENER__BASE_URL=https://api.dexscreener.com
```

### TOML 配置

在 `config/default.toml` 中：

```toml
[exchanges.dexscreener]
enabled = true
base_url = "https://api.dexscreener.com"
```

### 禁用 DexScreener

设置 `enabled = false` 即可：

```bash
APP__EXCHANGES__DEXSCREENER__ENABLED=false
```

## 使用示例

### 1. 查询 ASTER/USDT 价格

```bash
curl "http://localhost:13012/api/v1/price?base=ASTER&quote=USDT"
```

**响应**:
```json
{
  "symbol": "ASTER/USDT",
  "price": 1.6128,
  "exchange": "dexscreener:pancakeswap",
  "timestamp": "2025-10-01T02:24:15.123456Z"
}
```

### 2. 查询 WBTC/USDC 价格

```bash
curl "http://localhost:13012/api/v1/price?symbol=WBTC/USDC"
```

### 3. 查询支持的其他格式

```bash
# 自动解析 USDT 后缀
curl "http://localhost:13012/api/v1/price?symbol=ASTERUSDT"

# 使用斜杠分隔符
curl "http://localhost:13012/api/v1/price?symbol=ASTER/USDT"
```

### 4. 多数据源对比

同时查询 CEX 和 DEX 价格：

```bash
# 如果同一代币在 CEX 和 DEX 都有交易
# 例如：BTC 在 Binance、Bitget 和某些 DEX 上都有
curl "http://localhost:13012/api/v1/price?symbol=WBTC/USDT"
```

## 错误处理

### 1. 交易对不存在

**请求**:
```bash
curl "http://localhost:13012/api/v1/price?base=INVALID&quote=TOKEN"
```

**响应** (400 Bad Request):
```json
{
  "error": "No matching pair found for INVALID/TOKEN on DexScreener"
}
```

### 2. 符号格式错误

**请求**:
```bash
curl "http://localhost:13012/api/v1/price?symbol=RANDOMTOKEN"
```

**响应** (400 Bad Request):
```json
{
  "error": "Cannot parse symbol 'RANDOMTOKEN'. Use BASE/QUOTE format"
}
```

### 3. API 请求失败

如果 DexScreener API 不可用，会返回：
```json
{
  "error": "DexScreener API error 503: Service Temporarily Unavailable"
}
```

## 性能考虑

### 1. API 限流

DexScreener API 有速率限制，建议：
- 实现缓存机制（当前版本未实现）
- 避免频繁查询相同交易对
- 设置合理的超时时间（默认 10 秒）

### 2. 超时配置

在配置文件中调整超时时间：

```toml
[exchanges]
timeout_secs = 10  # 所有交易所客户端的超时时间
```

### 3. 响应时间

DexScreener 查询通常需要：
- 200-500ms（正常情况）
- 1-2秒（高负载时）

## 与其他数据源对比

| 特性 | DexScreener | Binance | Bitget |
|------|-------------|---------|--------|
| 数据源类型 | DEX 聚合 | CEX | CEX |
| 支持代币 | 长尾代币、新币 | 主流币 | 主流币 |
| 价格类型 | 去中心化 | 中心化 | 中心化 |
| 链信息 | 多链支持 | 无 | 无 |
| 流动性信息 | 有 | 无 | 无 |

## 高级功能（响应中包含的额外信息）

虽然当前实现只返回价格，但 DexScreener API 还提供：

- `chainId`: 区块链 ID（如 bsc, ethereum）
- `dexId`: DEX ID（如 pancakeswap, uniswap）
- `pairAddress`: 交易对合约地址
- `liquidity`: 流动性信息
- `volume`: 交易量数据
- `txns`: 交易笔数统计
- `priceChange`: 价格变化百分比

这些数据可以在后续版本中添加支持。

## 测试

### 单元测试

运行测试：
```bash
cargo test --lib dexscreener
```

### 集成测试

```bash
# 测试 ASTER/USDT 查询
curl -v "http://localhost:13012/api/v1/price?base=ASTER&quote=USDT"

# 测试错误处理
curl -v "http://localhost:13012/api/v1/price?base=INVALID&quote=TOKEN"

# 测试自动解析
curl -v "http://localhost:13012/api/v1/price?symbol=ASTERUSDT"
```

## 故障排除

### 问题 1: DexScreener 未初始化

**症状**: 日志中没有 "DexScreener client initialized"

**解决方案**:
1. 检查 `.env` 文件中的配置
2. 确认 `APP__EXCHANGES__DEXSCREENER__ENABLED=true`
3. 重启服务

### 问题 2: 无法找到交易对

**症状**: "No matching pair found for BASE/QUOTE on DexScreener"

**可能原因**:
1. 交易对不存在或未上市
2. 符号拼写错误
3. 该代币没有与指定 quote 货币的交易对

**解决方案**:
- 访问 [DexScreener.com](https://dexscreener.com) 确认交易对存在
- 检查正确的符号名称
- 尝试其他 quote 货币（如 USDT → USDC）

### 问题 3: 响应时间过长

**解决方案**:
1. 检查网络连接
2. 调整超时设置
3. 考虑实现缓存机制

## 未来改进

1. **价格缓存**: 减少 API 调用次数
2. **流动性筛选**: 只返回流动性充足的交易对
3. **多链支持**: 允许指定特定区块链
4. **扩展响应**: 包含流动性、交易量等信息
5. **WebSocket**: 实时价格推送
6. **批量查询**: 一次查询多个交易对

## 参考资料

- [DexScreener API 文档](https://docs.dexscreener.com/api/reference)
- [DexScreener 官网](https://dexscreener.com)
