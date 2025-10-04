# 数据源迁移总结：CoinDesk → API Ninjas

## 迁移概述

已成功将价格数据源从 CoinDesk 迁移至 API Ninjas。

## 迁移原因

### CoinDesk 的限制
- ✗ 仅支持 BTC 价格
- ✗ 无法查询其他加密货币（ETH, LTC, XRP 等）
- ✓ 无需 API Key
- ✓ 免费无限制

### API Ninjas 的优势
- ✓ 支持多种主流加密货币
- ✓ 支持 BTC, ETH, LTC, XRP, ADA, DOT, LINK 等
- ✓ 简单易用的 REST API
- ✓ 免费额度：10,000 次/月
- ✗ 需要注册获取 API Key

## 迁移变更

### 1. 代码变更

#### 删除的文件
- ✗ `src/services/coindesk.rs`
- ✗ `COINDESK_INTEGRATION.md`

#### 新增的文件
- ✓ `src/services/apininjas.rs`
- ✓ `APININJAS_INTEGRATION.md`

#### 修改的文件
- `src/services/mod.rs` - 更新导入
- `src/config/mod.rs` - 添加 ApiNinjasConfig
- `src/main.rs` - 更新初始化逻辑
- `config/default.toml` - 更新配置
- `.env.example` - 更新环境变量示例
- `README.md` - 更新文档
- `API_TESTS.md` - 更新测试用例

### 2. 配置变更

#### 旧配置（CoinDesk）
```toml
[exchanges.coindesk]
enabled = true
base_url = "https://api.coindesk.com"
```

#### 新配置（API Ninjas）
```toml
[exchanges.apininjas]
enabled = true
base_url = "https://api.api-ninjas.com"
api_key = "YOUR_API_KEY_HERE"
```

### 3. API 端点变更

| 特性 | CoinDesk | API Ninjas |
|------|----------|-----------|
| 端点 | `/v1/bpi/currentprice/{currency}.json` | `/v1/cryptoprice?symbol={symbol}` |
| 认证 | 无需 | API Key (Header) |
| 格式 | 分离的 currency | 组合的 symbol |

## 使用差异

### CoinDesk（已移除）
```bash
# 只能查询 BTC
curl "http://localhost:13012/api/v1/price?symbol=BTCUSD&exchange=coindesk"
curl "http://localhost:13012/api/v1/price?symbol=BTCEUR&exchange=coindesk"
```

### API Ninjas（新）
```bash
# 支持多种币种
curl "http://localhost:13012/api/v1/price?symbol=BTCUSD&exchange=apininjas"
curl "http://localhost:13012/api/v1/price?symbol=ETHUSDT&exchange=apininjas"
curl "http://localhost:13012/api/v1/price?symbol=LTCUSD&exchange=apininjas"
```

## 功能对比

| 功能 | CoinDesk | API Ninjas |
|------|----------|-----------|
| 支持的币种 | 仅 BTC | BTC, ETH, LTC, XRP, ADA, DOT, LINK 等 |
| API Key | 不需要 | **需要** |
| 请求限制 | 无限制 | 10,000/月（免费） |
| 响应速度 | 快 | 快 |
| 数据准确性 | 高 | 高 |
| 报价货币 | USD, EUR, GBP | USD, USDT, EUR, GBP 等 |

## 迁移步骤（已完成）

- [x] 创建 API Ninjas 客户端
- [x] 更新服务模块导入
- [x] 更新配置结构
- [x] 更新主程序初始化逻辑
- [x] 更新所有配置文件
- [x] 更新文档和测试用例
- [x] 删除 CoinDesk 相关代码
- [x] 编译测试通过

## 开始使用

### 1. 获取 API Key
访问 https://api-ninjas.com 注册并获取免费 API Key

### 2. 配置 API Key

**方式 1：配置文件**
编辑 `config/default.toml`：
```toml
[exchanges.apininjas]
enabled = true
base_url = "https://api.api-ninjas.com"
api_key = "your_actual_api_key_here"
```

**方式 2：环境变量**
```bash
export APP__EXCHANGES__APININJAS__API_KEY="your_actual_api_key_here"
```

### 3. 启动服务
```bash
cargo run
```

应该看到：
```
INFO gateway: Binance client initialized
INFO gateway: API Ninjas client initialized
```

### 4. 测试
```bash
# 测试 API Ninjas
curl "http://localhost:13012/api/v1/price?symbol=BTCUSD&exchange=apininjas"

# 对比 Binance 和 API Ninjas
curl "http://localhost:13012/api/v1/price?symbol=BTCUSD"
```

## 注意事项

1. **API Key 必须配置**
   - 没有 API Key 服务将无法启动
   - 可以临时禁用：`enabled = false`

2. **请求限额**
   - 免费账号：10,000 次/月
   - 建议监控使用量
   - 考虑实现缓存机制

3. **币种支持**
   - API Ninjas 支持主流币种
   - 不如 Binance 全面
   - 对于不支持的币种，会返回错误

## 推荐配置

建议同时启用两个数据源：

```toml
[exchanges.binance]
enabled = true
base_url = "https://api.binance.com"

[exchanges.apininjas]
enabled = true
base_url = "https://api.api-ninjas.com"
api_key = "your_api_key"
```

这样可以：
- Binance：作为主要数据源（覆盖更多币种）
- API Ninjas：作为备用和验证数据源

## 回滚方案

如果需要回滚到 CoinDesk：
1. 从 git 历史恢复 `src/services/coindesk.rs`
2. 恢复配置结构
3. 重新编译

但不建议回滚，因为 API Ninjas 提供了更多功能。

## 后续优化建议

1. **添加缓存**
   - 减少 API 调用次数
   - 降低延迟
   - 节省 API 配额

2. **错误处理**
   - API 限额超出时的降级策略
   - 自动切换到 Binance

3. **监控**
   - API 使用量监控
   - 错误率监控
   - 响应时间监控

4. **价格聚合**
   - 计算多个数据源的平均价格
   - 价格差异报警

## 相关文档

- [APININJAS_INTEGRATION.md](./APININJAS_INTEGRATION.md) - API Ninjas 详细集成文档
- [API_TESTS.md](./API_TESTS.md) - API 测试用例
- [README.md](./README.md) - 项目主文档
