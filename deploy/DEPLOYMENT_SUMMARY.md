# Gateway 部署摘要

## 部署信息

**部署时间**: 2025年10月1日  
**服务器**: ubuntu@43.156.92.191  
**部署目录**: /home/ubuntu/gateway  
**域名**: cryptoprice.bithub.pro

## 已部署的组件

### 1. Gateway 服务
- **二进制文件**: gateway (Linux x86_64, 15MB)
- **监听端口**: 13012
- **状态**: 运行中（systemd 管理）
- **自动启动**: 已启用

### 2. 配置文件
- `config/default.toml` - 主配置文件
- `.env` - 环境变量配置
- `.env.example` - 环境变量示例

### 3. Nginx 配置（待部署）
- `nginx-cryptoprice.conf` - HTTPS 配置（推荐）
- `nginx-cryptoprice-http.conf` - HTTP 配置（快速部署）

## 服务管理命令

### Gateway 服务
```bash
# 查看状态
sudo systemctl status gateway

# 查看日志
sudo journalctl -u gateway -f

# 重启服务
sudo systemctl restart gateway

# 停止服务
sudo systemctl stop gateway

# 启动服务
sudo systemctl start gateway
```

### Nginx 配置（需要手动部署）
```bash
# 方式一：仅 HTTP（快速）
cd /home/ubuntu/gateway
bash setup-nginx-http.sh

# 方式二：HTTPS + SSL（推荐）
cd /home/ubuntu/gateway
bash setup-nginx.sh
```

## API 端点

### 直接访问（本地）
```bash
# 健康检查
curl http://localhost:13012/health

# 价格查询
curl 'http://localhost:13012/api/v1/price?symbol=BTCUSDT'
```

### 通过域名访问（部署 Nginx 后）
```bash
# 健康检查
curl http://cryptoprice.bithub.pro/health

# 价格查询
curl 'http://cryptoprice.bithub.pro/api/v1/price?symbol=BTCUSDT'

# HTTPS（如果配置了 SSL）
curl https://cryptoprice.bithub.pro/health
```

## 部署清单

- [x] 编译 Linux x86_64 二进制文件
- [x] 准备配置文件和环境变量
- [x] 上传文件到服务器
- [x] 配置 systemd 服务
- [x] 启动并验证服务
- [x] 创建 Nginx 配置文件
- [ ] 部署 Nginx（待执行）
- [ ] 配置 SSL 证书（可选）

## 下一步操作

### 1. 配置域名解析
确保 `cryptoprice.bithub.pro` 解析到服务器 IP: `43.156.92.191`

### 2. 配置防火墙
```bash
# 允许 HTTP
sudo ufw allow 80

# 允许 HTTPS
sudo ufw allow 443

# 查看防火墙状态
sudo ufw status
```

### 3. 部署 Nginx
根据需求选择：
- **快速部署（HTTP）**: `bash setup-nginx-http.sh`
- **生产部署（HTTPS）**: `bash setup-nginx.sh`

### 4. 测试服务
```bash
# 本地测试
curl http://localhost:13012/health

# 域名测试（部署 Nginx 后）
curl http://cryptoprice.bithub.pro/health
```

## 故障排查

### Gateway 服务无法启动
```bash
# 查看详细日志
sudo journalctl -u gateway -n 50

# 检查端口占用
sudo lsof -i :13012

# 手动运行测试
cd /home/ubuntu/gateway
./gateway
```

### Nginx 配置问题
```bash
# 测试配置
sudo nginx -t

# 查看 Nginx 日志
sudo tail -f /var/log/nginx/error.log
sudo tail -f /var/log/nginx/cryptoprice.error.log

# 重新加载配置
sudo systemctl reload nginx
```

### SSL 证书问题
```bash
# 检查证书状态
sudo certbot certificates

# 手动续期测试
sudo certbot renew --dry-run

# 强制续期
sudo certbot renew --force-renewal
```

## 文件列表

```
/home/ubuntu/gateway/
├── gateway                          # 主程序
├── config/
│   └── default.toml                 # 配置文件
├── .env                             # 环境变量（包含 API keys）
├── .env.example                     # 环境变量示例
├── start.sh                         # 启动脚本
├── install.sh                       # 服务安装脚本
├── gateway.service                  # systemd 服务文件
├── nginx-cryptoprice.conf           # Nginx HTTPS 配置
├── nginx-cryptoprice-http.conf      # Nginx HTTP 配置
├── setup-nginx.sh                   # Nginx HTTPS 部署脚本
├── setup-nginx-http.sh              # Nginx HTTP 部署脚本
└── README.md                        # 说明文档
```

## 维护建议

1. **定期查看日志**: 每天检查服务日志，及时发现问题
2. **监控服务状态**: 配置监控告警（如 Prometheus + Grafana）
3. **备份配置**: 定期备份 .env 和配置文件
4. **更新证书**: SSL 证书会自动续期，但建议每月检查一次
5. **安全更新**: 定期更新系统和 Nginx：`sudo apt update && sudo apt upgrade`

## 联系信息

如有问题，请检查：
1. Gateway 服务日志：`sudo journalctl -u gateway -f`
2. Nginx 日志：`/var/log/nginx/cryptoprice.error.log`
3. 系统日志：`sudo journalctl -xe`
