# Bitget 集成说明

## 概述

已成功集成 Bitget 作为新的加密货币价格数据源。Bitget 是全球领先的加密货币衍生品交易所之一，提供现货和合约交易。

## Bitget 特性

### 关于 Bitget
- 全球领先的加密货币交易所
- 成立于 2018 年
- 总部位于新加坡
- 支持现货、合约、跟单等多种交易方式
- 全球用户超过 2000 万

### 支持的交易对
- 数百种现货交易对
- 主流币种：BTC, ETH, SOL, XRP, ADA, DOT, MATIC 等
- 稳定币交易对：USDT, USDC
- 示例：`BTCUSDT`, `ETHUSDT`, `SOLUSDT`

### API 特点
- **免费、无需 API Key**（公开市场数据）
- API 端点：`https://api.bitget.com/api/v2/spot/market/tickers`
- 实时价格数据
- 高可用性和稳定性
- 符合 REST API 标准

## 实现细节

### 1. 客户端实现
文件：`src/services/bitget.rs`

主要功能：
- 实现了 `ExchangeClient` trait
- 自动格式化符号为 Bitget 格式（`BTCUSDT` -> `BTCUSDT_SPBL`）
- 解析 JSON 响应数据
- 错误处理和状态码检查
- 返回标准化的价格数据

### 2. 符号格式
Bitget 现货交易对使用特殊格式：
- 输入：`BTCUSDT`
- Bitget 格式：`BTCUSDT_SPBL`（_SPBL 表示 spot）
- 输出：自动移除 `_SPBL` 后缀

### 3. API 响应结构
```json
{
  "code": "00000",
  "msg": "success",
  "requestTime": 1696161600000,
  "data": {
    "symbol": "BTCUSDT_SPBL",
    "high24h": "43500.50",
    "low24h": "42800.30",
    "close": "43250.50",
    "quoteVol": "1234567890.00",
    "baseVol": "28500.50",
    "usdtVol": "1234567890.00",
    "ts": "1696161600000"
  }
}
```

## 配置示例

### config/default.toml
```toml
[exchanges.bitget]
enabled = true
base_url = "https://api.bitget.com"
```

### 环境变量
```bash
APP__EXCHANGES__BITGET__ENABLED=true
APP__EXCHANGES__BITGET__BASE_URL=https://api.bitget.com
```

## 使用示例

### 单独查询 Bitget
```bash
# 使用 symbol 参数
curl "http://localhost:13012/api/v1/price?symbol=BTCUSDT&exchange=bitget"

# 使用 base 和 quote 参数
curl "http://localhost:13012/api/v1/price?base=ETH&quote=USDT&exchange=bitget"

# 查询 SOL 价格
curl "http://localhost:13012/api/v1/price?symbol=SOLUSDT&exchange=bitget"
```

### 从所有数据源查询（价格对比）
```bash
# 不指定 exchange，会同时查询 Binance、Bitget 和 API Ninjas
curl "http://localhost:13012/api/v1/price?symbol=BTCUSDT"
```

响应示例：
```json
{
  "success": true,
  "data": [
    {
      "symbol": "BTCUSDT",
      "price": 43250.50,
      "exchange": "binance",
      "timestamp": 1696161600000
    },
    {
      "symbol": "BTCUSDT",
      "price": 43249.80,
      "exchange": "bitget",
      "timestamp": 1696161601000
    },
    {
      "symbol": "BTCUSD",
      "price": 43248.30,
      "exchange": "apininjas",
      "timestamp": 1696161602000
    }
  ]
}
```

## 支持的交易对示例

### 主流币种
- `BTCUSDT` - 比特币/USDT
- `ETHUSDT` - 以太坊/USDT
- `SOLUSDT` - Solana/USDT
- `XRPUSDT` - 瑞波币/USDT
- `ADAUSDT` - 艾达币/USDT
- `DOTUSDT` - 波卡/USDT
- `MATICUSDT` - Polygon/USDT
- `AVAXUSDT` - Avalanche/USDT

### 稳定币对
- `BTCUSDC` - 比特币/USDC
- `ETHUSDC` - 以太坊/USDC

## 与其他交易所的对比

| 特性 | Binance | Bitget | API Ninjas |
|------|---------|--------|-----------|
| 交易对数量 | 1000+ | 300+ | 50+ |
| API Key | 不需要 | 不需要 | **需要** |
| 请求限制 | 宽松 | 宽松 | 10k/月（免费） |
| 响应速度 | 快 | 快 | 中 |
| 数据质量 | 高 | 高 | 高 |
| 全球排名 | Top 1 | Top 10 | N/A |

## 错误处理

### API 错误
```json
{
  "success": false,
  "error": "Bitget API error: 40001 - Invalid symbol"
}
```

### 网络错误
```json
{
  "success": false,
  "error": "Bitget API error 500: Internal Server Error"
}
```

### 不支持的交易对
```json
{
  "success": false,
  "error": "Bitget API error: 40014 - Symbol not found"
}
```

## 限制和注意事项

1. **仅支持现货交易对**
   - 当前实现仅支持现货市场（_SPBL）
   - 不支持合约市场数据

2. **符号格式**
   - 需要完整的交易对符号（如 `BTCUSDT`）
   - 系统会自动添加 `_SPBL` 后缀

3. **请求频率**
   - 公开 API 有速率限制
   - 建议添加缓存机制

## 优势

1. **免费使用**
   - 公开市场数据无需 API Key
   - 无请求次数限制（合理使用）

2. **数据可靠**
   - 来自真实交易所
   - 实时价格数据

3. **全球化**
   - 支持多个地区
   - 多语言支持

4. **高性能**
   - 响应速度快
   - API 稳定可靠

## 推荐使用场景

### 最佳用途
- 主流币种价格查询
- 多交易所价格对比
- 套利机会发现
- 价格监控和报警

### 配合其他数据源
建议同时启用多个数据源：
- **Binance**：最全面的币种覆盖
- **Bitget**：可靠的备用数据源
- **API Ninjas**：第三方验证数据源

## 性能优化建议

1. **实现缓存**
   ```rust
   // 示例：使用 TTL 缓存
   // 缓存时间：10-30 秒
   ```

2. **请求合并**
   - 对于相同交易对的并发请求，合并为一次 API 调用

3. **错误重试**
   - 实现指数退避重试机制
   - 最多重试 3 次

4. **连接池**
   - 复用 HTTP 连接
   - 减少连接建立开销

## 测试

服务启动后可以看到：
```
INFO gateway: Binance client initialized
INFO gateway: Bitget client initialized
INFO gateway: API Ninjas client initialized
```

表示所有数据源都已成功初始化。

## 官方资源

- [Bitget 官网](https://www.bitget.com)
- [Bitget API 文档](https://www.bitget.com/api-doc/spot/market/Get-Tickers)
- [现货市场](https://www.bitget.com/spot)

## 技术支持

如遇到问题：
1. 检查网络连接
2. 验证 API 端点是否正确
3. 查看服务日志获取详细错误信息
4. 参考官方 API 文档

## 未来增强

可能的改进方向：
- 支持合约市场数据
- 添加 24 小时成交量信息
- 实现 WebSocket 实时数据推送
- 添加订单簿数据查询
