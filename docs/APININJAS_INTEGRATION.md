# API Ninjas 集成说明

## 概述

已成功集成 API Ninjas 作为新的加密货币价格数据源，替代了 CoinDesk。API Ninjas 提供了更广泛的加密货币价格支持。

## API Ninjas 特性

### 支持的加密货币
- 主流加密货币：BTC, ETH, LTC, XRP, BCH, ADA, DOT, LINK 等
- 多种法币报价：USD, USDT, EUR, GBP 等
- 示例：`BTCUSD`, `ETHUSDT`, `LTCUSD`

### API 特点
- **需要 API Key**（免费注册获取）
- API 端点：`https://api.api-ninjas.com/v1/cryptoprice`
- 实时价格数据
- 简单易用的 REST API

### 与 CoinDesk 的对比

| 特性 | API Ninjas | CoinDesk |
|------|-----------|----------|
| 支持币种 | 多种主流币 | 仅 BTC |
| API Key | 需要 | 不需要 |
| 免费额度 | 10,000 请求/月 | 无限制 |
| 数据质量 | 高 | 高 |
| 易用性 | 简单 | 简单 |

## 获取 API Key

1. 访问 [API Ninjas](https://api-ninjas.com)
2. 注册免费账号
3. 在控制台获取 API Key
4. 免费账号提供 10,000 次/月的请求额度

## 实现细节

### 1. 客户端实现
文件：`src/services/apininjas.rs`

主要功能：
- 实现了 `ExchangeClient` trait
- 自动解析交易对符号（如 `BTCUSD` -> `BTC` + `USD`）
- 支持多种报价货币（USD, USDT, EUR, GBP 等）
- HTTP Header 中包含 API Key 认证
- 完善的错误处理

### 2. 配置集成
- 添加了 `apininjas` 配置项，包含独立的配置结构 `ApiNinjasConfig`
- 支持配置 API Key
- 可启用/禁用 API Ninjas

### 3. 符号解析
自动识别常见的报价货币后缀：
- USDT, USDC, USD, EUR, GBP, JPY
- BTC, ETH (作为报价货币)

## 配置示例

### config/default.toml
```toml
[exchanges.apininjas]
enabled = true
base_url = "https://api.api-ninjas.com"
api_key = "YOUR_API_KEY_HERE"  # 从 api-ninjas.com 获取
```

### 环境变量
```bash
APP__EXCHANGES__APININJAS__ENABLED=true
APP__EXCHANGES__APININJAS__BASE_URL=https://api.api-ninjas.com
APP__EXCHANGES__APININJAS__API_KEY=your_actual_api_key
```

## 使用示例

### 单独查询 API Ninjas
```bash
# 使用 symbol 参数
curl "http://localhost:13012/api/v1/price?symbol=BTCUSD&exchange=apininjas"

# 使用 base 和 quote 参数
curl "http://localhost:13012/api/v1/price?base=ETH&quote=USD&exchange=apininjas"
```

### 从所有数据源查询（价格对比）
```bash
# 不指定 exchange，会同时查询 Binance 和 API Ninjas
curl "http://localhost:13012/api/v1/price?symbol=BTCUSD"
```

响应示例：
```json
{
  "success": true,
  "data": [
    {
      "symbol": "BTCUSD",
      "price": 43250.50,
      "exchange": "binance",
      "timestamp": 1696161600000
    },
    {
      "symbol": "BTCUSD",
      "price": 43248.30,
      "exchange": "apininjas",
      "timestamp": 1696161601000
    }
  ]
}
```

## 支持的交易对示例

### 主流币种
- `BTCUSD`, `BTCUSDT` - 比特币
- `ETHUSD`, `ETHUSDT` - 以太坊
- `LTCUSD` - 莱特币
- `XRPUSD` - 瑞波币
- `BCHUSD` - 比特币现金
- `ADAUSD` - 艾达币
- `DOTUSD` - 波卡
- `LINKUSD` - Chainlink

### 法币报价
- USD - 美元
- USDT - Tether
- EUR - 欧元
- GBP - 英镑

## 错误处理

### API Key 无效
```json
{
  "success": false,
  "error": "API Ninjas API error 401: Unauthorized"
}
```

### 不支持的交易对
```json
{
  "success": false,
  "error": "Unable to parse symbol 'INVALIDPAIR'"
}
```

### API 限额超出
```json
{
  "success": false,
  "error": "API Ninjas API error 429: Too Many Requests"
}
```

## 限制和注意事项

1. **需要 API Key**
   - 必须从 api-ninjas.com 获取
   - 免费账号有请求限制（10,000/月）

2. **支持的币种**
   - 仅支持主流加密货币
   - 不如 Binance 支持的币种多

3. **请求频率**
   - 免费账号有速率限制
   - 建议添加缓存机制优化使用

## 与 Binance 的配合使用

建议同时启用两个数据源：
- **Binance**：作为主要数据源，覆盖更多币种
- **API Ninjas**：作为备用数据源，用于价格验证和对比

这样可以：
1. 提高数据可靠性
2. 对比不同数据源的价格
3. 在某个数据源不可用时有备选方案

## 性能优化建议

1. **缓存机制**
   - 实现价格缓存，减少 API 调用
   - 设置合理的缓存过期时间（如 10-30 秒）

2. **请求合并**
   - 对于相同交易对的并发请求，合并为一次 API 调用

3. **监控**
   - 监控 API 使用量
   - 在接近限额时发出警告

## 测试

服务启动后可以看到：
```
INFO gateway: Binance client initialized
INFO gateway: API Ninjas client initialized
```

表示两个数据源都已成功初始化。

## 官方文档

- [API Ninjas 官网](https://api-ninjas.com)
- [Crypto Price API 文档](https://api-ninjas.com/api/cryptoprice)
- [注册获取 API Key](https://api-ninjas.com/register)
