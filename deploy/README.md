# Gateway 部署说明

## 文件说明
- `gateway`: 编译好的二进制文件（Linux x86_64）
- `config/`: 配置文件目录
- `.env`: 环境变量配置文件（需要根据实际情况修改）
- `.env.example`: 环境变量示例文件
- `start.sh`: 启动脚本

## 部署步骤

### 1. 上传文件到服务器
```bash
scp -r deploy ubuntu@43.156.92.191:/opt/gateway
```

### 2. SSH 登录服务器
```bash
ssh ubuntu@43.156.92.191
```

### 3. 设置权限
```bash
cd /opt/gateway
chmod +x gateway start.sh
```

### 4. 配置环境变量
编辑 `.env` 文件，配置必要的环境变量（API keys 等）

### 5. 运行服务
```bash
# 直接运行
./start.sh

# 或使用 nohup 在后台运行
nohup ./start.sh > gateway.log 2>&1 &

# 或使用 systemd（推荐）
```

## 使用 systemd 管理服务（推荐）

创建 systemd service 文件：
```bash
sudo nano /etc/systemd/system/gateway.service
```

内容：
```ini
[Unit]
Description=Gateway Service
After=network.target

[Service]
Type=simple
User=ubuntu
WorkingDirectory=/opt/gateway
ExecStart=/opt/gateway/gateway
Restart=always
RestartSec=5
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
```

启用并启动服务：
```bash
sudo systemctl daemon-reload
sudo systemctl enable gateway
sudo systemctl start gateway
sudo systemctl status gateway
```

查看日志：
```bash
sudo journalctl -u gateway -f
```

## 端口说明
默认端口：13012（可在配置文件中修改）

确保服务器防火墙允许该端口访问。

## Nginx 反向代理配置

### 方式一：仅 HTTP（快速部署）
```bash
# 在服务器上运行
cd /home/ubuntu/gateway
bash setup-nginx-http.sh
```

配置文件：`nginx-cryptoprice-http.conf`
访问地址：`http://cryptoprice.bithub.pro`

### 方式二：HTTPS + SSL 证书（推荐生产环境）
```bash
# 在服务器上运行
cd /home/ubuntu/gateway
bash setup-nginx.sh
```

配置文件：`nginx-cryptoprice.conf`
访问地址：`https://cryptoprice.bithub.pro`

注意：
1. 运行前确保域名 `cryptoprice.bithub.pro` 已解析到服务器 IP
2. 需要开放防火墙端口：
   - HTTP: `sudo ufw allow 80`
   - HTTPS: `sudo ufw allow 443`

### API 测试
```bash
# 健康检查
curl http://cryptoprice.bithub.pro/health

# 价格查询
curl 'http://cryptoprice.bithub.pro/api/v1/price?symbol=BTCUSDT'
```
