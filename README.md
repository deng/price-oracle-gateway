# Price Oracle Gateway

一个使用 Rust 和 Axum 构建的高性能加密货币价格查询网关服务。

## 功能特性

- 🚀 高性能异步架构（基于 Tokio 和 Axum）
- 📊 支持多个交易所/数据源价格查询
  - Binance（支持数千种加密货币交易对）
  - Bitget（全球领先的加密货币交易所）
  - API Ninjas（支持主流加密货币价格）
  - DexScreener（DEX 聚合器，支持多链 DEX 价格查询）
- 🔄 智能多 DEX 支持
  - 自动检测交易对在多个 DEX 的存在
  - 基于流动性自动选择最优 DEX
  - 透明日志记录所有可用 DEX
- ⚡ 智能缓存机制
  - 10 秒 TTL 价格缓存
  - 缓存状态透明标识
  - 高性能内存缓存（moka）
- ⚙️ 灵活的配置管理（支持配置文件和环境变量）
- 📝 结构化日志记录
- 🏥 健康检查端点
- 🌐 CORS 支持

## 快速开始

### 前置要求

- Rust 1.70 或更高版本
- Cargo

### 安装

1. 克隆项目：
```bash
git clone <repository-url>
cd gateway
```

2. 创建配置文件：
```bash
mkdir -p config
cat > config/default.toml << EOF
[server]
host = "0.0.0.0"
port = 3000

[exchanges]
timeout_secs = 10

[exchanges.binance]
enabled = true
base_url = "https://api.binance.com"

[exchanges.bitget]
enabled = true
base_url = "https://api.bitget.com"

[exchanges.apininjas]
enabled = true
base_url = "https://api.api-ninjas.com"
api_key = "YOUR_API_KEY_HERE"
EOF
```

3. （可选）创建 `.env` 文件：
```bash
cat > .env << EOF
# 服务器配置
APP__SERVER__HOST=0.0.0.0
APP__SERVER__PORT=3000

# 日志级别
RUST_LOG=gateway=debug,tower_http=debug
EOF
```

### 运行

```bash
# 开发模式
cargo run

# 发布模式
cargo run --release
```

服务将在 `http://0.0.0.0:3000` 启动。

## API 文档

### 健康检查

检查服务运行状态。

**请求：**
```
GET /health
```

**响应：**
```json
{
  "status": "healthy",
  "timestamp": "2025-10-01T10:00:00.000Z",
  "version": "0.1.0"
}
```

### 获取价格

查询指定交易对的价格。支持两种查询方式：

#### 方式 1：使用交易对符号

**请求：**
```
GET /api/v1/price?symbol=BTCUSDT&exchange=binance
```

**参数：**
- `symbol` (必需): 交易对符号，例如 `BTCUSDT`
- `exchange` (可选): 交易所名称，目前仅支持 `binance`

#### 方式 2：使用基础货币和报价货币

**请求：**
```
GET /api/v1/price?base=BTC&quote=USDT&exchange=binance
```

**参数：**
- `base` (必需): 基础货币符号，例如 `BTC`
- `quote` (必需): 报价货币符号，例如 `USDT`
- `exchange` (可选): 交易所/数据源名称，支持 `binance`、`bitget` 或 `apininjas`

**注意：** 必须提供 `symbol` 或同时提供 `base` 和 `quote`。如果两者都提供，将优先使用 `base` 和 `quote`。

**响应示例：**
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

**错误响应示例：**
```json
{
  "success": false,
  "error": "Either 'symbol' or both 'base' and 'quote' must be provided"
}
```



## 配置说明

配置可以通过以下方式提供（优先级从高到低）：

1. **环境变量** - 使用 `APP__` 前缀，双下划线分隔层级
   - 例如: `APP__SERVER__PORT=8080`
2. **配置文件** - `config/default.toml`

### 配置项

| 配置项 | 类型 | 默认值 | 说明 |
|--------|------|--------|------|
| `server.host` | string | "0.0.0.0" | 服务监听地址 |
| `server.port` | number | 3000 | 服务监听端口 |
| `exchanges.timeout_secs` | number | 10 | 交易所 API 请求超时时间（秒） |
| `exchanges.binance.enabled` | boolean | true | 是否启用 Binance |
| `exchanges.binance.base_url` | string | - | Binance API 基础 URL |
| `exchanges.bitget.enabled` | boolean | true | 是否启用 Bitget |
| `exchanges.bitget.base_url` | string | - | Bitget API 基础 URL |
| `exchanges.apininjas.enabled` | boolean | true | 是否启用 API Ninjas |
| `exchanges.apininjas.base_url` | string | - | API Ninjas API 基础 URL |
| `exchanges.apininjas.api_key` | string | - | API Ninjas API Key |

## 项目结构

```
gateway/
├── src/
│   ├── config/         # 配置管理
│   ├── handlers/       # HTTP 请求处理器
│   ├── models/         # 数据模型
│   ├── services/       # 交易所客户端服务
│   ├── error.rs        # 错误处理
│   └── main.rs         # 应用入口
├── config/             # 配置文件目录
├── Cargo.toml          # 项目依赖
└── README.md           # 项目文档
```

## 开发

### 数据源支持说明

#### Binance
- 支持数千种加密货币交易对
- 使用交易对符号格式：`BTCUSDT`, `ETHUSDT` 等
- API 端点：`https://api.binance.com/api/v3/ticker/price`
- 无需 API Key

#### Bitget
- 全球领先的加密货币衍生品交易所
- 支持现货交易对（spot）
- 使用交易对符号格式：`BTCUSDT`, `ETHUSDT` 等
- API 端点：`https://api.bitget.com/api/v2/spot/market/tickers`
- 无需 API Key

#### API Ninjas
- 支持主流加密货币价格查询
- 支持的交易对：BTC, ETH, LTC, XRP 等主流币种
- 使用格式：`BTCUSD`, `ETHUSDT` 等
- API 端点：`https://api.api-ninjas.com/v1/cryptoprice`
- **需要 API Key**（从 https://api-ninjas.com 获取）

#### DexScreener
- DEX 聚合器，支持多链去中心化交易所价格
- **多 DEX 支持**：自动检测交易对在多个 DEX 的存在
- **智能选择**：基于 USD 流动性自动选择最优 DEX
- **透明日志**：记录所有找到的 DEX
- 支持的链：Ethereum, BSC, Polygon, Arbitrum 等多条链
- API 端点：`https://api.dexscreener.com/latest/dex/search`
- 无需 API Key
- 详细文档：[docs/MULTI_DEX_SUPPORT.md](docs/MULTI_DEX_SUPPORT.md)

### 获取 API Ninjas API Key

1. 访问 https://api-ninjas.com
2. 注册账号
3. 在控制台获取 API Key
4. 将 API Key 配置到 `config/default.toml` 或环境变量中

### 添加新的数据源

1. 在 `src/services/` 中创建新的交易所客户端
2. 实现 `ExchangeClient` trait
3. 在 `src/config/mod.rs` 中添加配置结构
4. 在 `src/main.rs` 中初始化客户端

### 运行测试

```bash
cargo test
```

### 代码检查

```bash
cargo clippy
```

### 代码格式化

```bash
cargo fmt
```

## 详细文档

- [API_TESTS.md](docs/API_TESTS.md) - API 测试示例
- [APININJAS_INTEGRATION.md](docs/APININJAS_INTEGRATION.md) - API Ninjas 集成文档
- [BITGET_INTEGRATION.md](docs/BITGET_INTEGRATION.md) - Bitget 集成文档
- [DEXSCREENER_INTEGRATION.md](docs/DEXSCREENER_INTEGRATION.md) - DexScreener 集成文档
- [MULTI_DEX_SUPPORT.md](docs/MULTI_DEX_SUPPORT.md) - 🆕 多 DEX 支持文档
- [CACHE_IMPLEMENTATION.md](docs/CACHE_IMPLEMENTATION.md) - 缓存实现文档
- [CACHED_FLAG.md](docs/CACHED_FLAG.md) - 缓存标识文档
- [MIGRATION_COINDESK_TO_APININJAS.md](docs/MIGRATION_COINDESK_TO_APININJAS.md) - 迁移文档

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！
