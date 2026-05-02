#!/bin/bash

# 快速 Nginx 配置部署脚本（仅 HTTP，无 SSL）
# 在服务器上运行

set -e

DOMAIN="cryptoprice.bithub.pro"
NGINX_CONF="nginx-cryptoprice-http.conf"
GATEWAY_DIR="/home/ubuntu/gateway/price-oracle"

echo "=== Nginx 快速配置部署（仅 HTTP）==="
echo "域名: $DOMAIN"
echo ""

# 检查 Nginx 是否已安装
if ! command -v nginx &> /dev/null; then
    echo "1. 安装 Nginx..."
    sudo apt update
    sudo apt install -y nginx
    echo "   Nginx 已安装"
else
    echo "1. Nginx 已安装"
fi

# 部署 Nginx 配置
echo ""
echo "2. 部署 Nginx 配置..."
sudo cp $GATEWAY_DIR/$NGINX_CONF /etc/nginx/sites-available/$DOMAIN

# 启用站点配置
echo ""
echo "3. 启用站点..."
sudo ln -sf /etc/nginx/sites-available/$DOMAIN /etc/nginx/sites-enabled/

# 测试 Nginx 配置
echo ""
echo "4. 测试 Nginx 配置..."
sudo nginx -t

# 重新加载 Nginx
echo ""
echo "5. 重新加载 Nginx..."
sudo systemctl reload nginx

# 确保 Nginx 开机自启
sudo systemctl enable nginx

# 显示服务状态
echo ""
echo "6. Nginx 服务状态："
sudo systemctl status nginx --no-pager | head -20

echo ""
echo "=== 部署完成 ==="
echo ""
echo "配置信息："
echo "  域名：        http://$DOMAIN"
echo "  健康检查：    http://$DOMAIN/health"
echo "  价格 API：    http://$DOMAIN/api/v1/price?symbol=BTCUSDT"
echo ""
echo "测试命令："
echo "  curl http://$DOMAIN/health"
echo "  curl 'http://$DOMAIN/api/v1/price?symbol=BTCUSDT'"
echo ""
echo "常用命令："
echo "  测试配置：    sudo nginx -t"
echo "  重载配置：    sudo systemctl reload nginx"
echo "  查看日志：    sudo tail -f /var/log/nginx/cryptoprice.access.log"
echo ""
echo "注意事项："
echo "  1. 确保防火墙开放 80 端口：sudo ufw allow 80"
echo "  2. 确保域名已解析到此服务器 IP"
echo "  3. 如需 HTTPS，请使用 setup-nginx.sh 脚本"
