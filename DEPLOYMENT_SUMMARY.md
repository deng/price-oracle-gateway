# Gateway 部署摘要

## 部署信息
- **编译日期**: 2025年10月1日
- **目标平台**: Linux x86_64 (Ubuntu)
- **服务器地址**: ubuntu@43.156.92.191
- **部署目录**: /home/ubuntu/gateway
- **服务端口**: 13012

## 部署状态
✅ 二进制文件已成功编译（使用 cross 工具）
✅ 所有配置文件已上传
✅ systemd 服务已安装并启用
✅ 服务正在运行
✅ API 测试通过

## 服务端点
- **健康检查**: http://43.156.92.191:13012/health
- **价格查询**: http://43.156.92.191:13012/api/v1/price?symbol=BTCUSDT

## 管理命令
```bash
# SSH 登录
ssh -i /Users/dengzhizhong/.ssh/id_rsa ubuntu@43.156.92.191

# 服务管理
sudo systemctl status gateway    # 查看状态
sudo systemctl restart gateway   # 重启服务
sudo systemctl stop gateway      # 停止服务
sudo systemctl start gateway     # 启动服务

# 查看日志
sudo journalctl -u gateway -f    # 实时日志
sudo journalctl -u gateway -n 100  # 最近100条日志

# 查看端口占用
sudo lsof -i :13012
sudo netstat -tulpn | grep 13012
```

## 测试命令
```bash
# 本地测试（在服务器上）
curl http://localhost:13012/health
curl 'http://localhost:13012/api/v1/price?symbol=BTCUSDT'

# 远程测试（从你的电脑）
curl http://43.156.92.191:13012/health
curl 'http://43.156.92.191:13012/api/v1/price?symbol=BTCUSDT'
```

## 已测试的 API 响应
```json
{
  "status": "healthy",
  "timestamp": "2025-10-01T03:32:59.031925379Z",
  "version": "0.1.0"
}

{
  "success": true,
  "data": [
    {
      "symbol": "BTCUSDT",
      "price": 114463.14,
      "exchange": "binance",
      "timestamp": 1759289587108,
      "cached": false
    },
    {
      "symbol": "BTCUSDT",
      "price": 114464.98,
      "exchange": "bitget",
      "timestamp": 1759289587257,
      "cached": false
    }
  ]
}
```

## 部署的文件
```
/home/ubuntu/gateway/
├── gateway              # 二进制可执行文件（15MB）
├── config/              # 配置文件目录
│   └── default.toml
├── .env                 # 环境变量配置
├── .env.example         # 环境变量示例
├── gateway.service      # systemd 服务配置
├── start.sh             # 启动脚本
├── install.sh           # 安装脚本
└── README.md           # 部署文档
```

## 注意事项
1. 服务已配置为开机自启
2. 服务会在失败后自动重启（RestartSec=5s）
3. 环境变量从 .env 文件加载
4. 日志级别默认为 INFO（可通过 RUST_LOG 环境变量调整）
5. **请确保服务器防火墙允许 13012 端口访问**

## 防火墙配置（如需要）
```bash
# 如果使用 ufw
sudo ufw allow 13012/tcp
sudo ufw reload

# 如果使用 iptables
sudo iptables -A INPUT -p tcp --dport 13012 -j ACCEPT
sudo iptables-save
```

## 更新部署
如需更新服务，可以运行以下命令：
```bash
# 在本地编译新版本
cross build --release --target x86_64-unknown-linux-gnu

# 停止远程服务
ssh -i /Users/dengzhizhong/.ssh/id_rsa ubuntu@43.156.92.191 "sudo systemctl stop gateway"

# 上传新的二进制文件
scp -i /Users/dengzhizhong/.ssh/id_rsa target/x86_64-unknown-linux-gnu/release/gateway ubuntu@43.156.92.191:/home/ubuntu/gateway/

# 重启服务
ssh -i /Users/dengzhizhong/.ssh/id_rsa ubuntu@43.156.92.191 "sudo systemctl start gateway"
```

## 问题排查
如果服务无法启动，检查：
1. 端口是否被占用：`sudo lsof -i :13012`
2. 二进制文件是否有执行权限：`ls -l /home/ubuntu/gateway/gateway`
3. 环境变量是否正确配置：`cat /home/ubuntu/gateway/.env`
4. 服务日志：`sudo journalctl -u gateway -n 50`

## 编译说明
本次编译使用了以下特性：
- 使用 `cross` 工具进行交叉编译
- 启用了 `native-tls-vendored` 特性以静态链接 OpenSSL
- 编译模式：release（优化）
- 目标平台：x86_64-unknown-linux-gnu
