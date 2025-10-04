#!/bin/bash

# Nginx 配置部署脚本 - 在服务器上运行

set -e

DOMAIN="cryptoprice.bithub.pro"
NGINX_CONF="nginx-cryptoprice.conf"
GATEWAY_DIR="/home/ubuntu/gateway"

echo "=== Nginx 配置部署脚本 ==="
echo "域名: $DOMAIN"
echo ""

# 检查 Nginx 是否已安装
if ! command -v nginx &> /dev/null; then
    echo "1. 安装 Nginx..."
    sudo apt update
    sudo apt install -y nginx
    echo "   Nginx 已安装"
else
    echo "1. Nginx 已安装，跳过"
fi

# 检查 certbot 是否已安装
if ! command -v certbot &> /dev/null; then
    echo ""
    echo "2. 安装 Certbot（Let's Encrypt）..."
    sudo apt install -y certbot python3-certbot-nginx
    echo "   Certbot 已安装"
else
    echo ""
    echo "2. Certbot 已安装，跳过"
fi

# 创建 certbot 验证目录
echo ""
echo "3. 创建证书验证目录..."
sudo mkdir -p /var/www/certbot
sudo chown -R www-data:www-data /var/www/certbot

# 复制临时 Nginx 配置（用于获取证书）
echo ""
echo "4. 配置临时 Nginx（用于获取 SSL 证书）..."
sudo tee /etc/nginx/sites-available/$DOMAIN > /dev/null <<EOF
server {
    listen 80;
    listen [::]:80;
    server_name $DOMAIN;

    location /.well-known/acme-challenge/ {
        root /var/www/certbot;
    }

    location / {
        return 200 'OK';
        add_header Content-Type text/plain;
    }
}
EOF

# 启用站点配置
sudo ln -sf /etc/nginx/sites-available/$DOMAIN /etc/nginx/sites-enabled/

# 测试 Nginx 配置
echo ""
echo "5. 测试 Nginx 配置..."
sudo nginx -t

# 重新加载 Nginx
echo ""
echo "6. 重新加载 Nginx..."
sudo systemctl reload nginx

# 获取 SSL 证书
echo ""
echo "7. 获取 SSL 证书..."
echo "   请确保域名 $DOMAIN 已经解析到此服务器的 IP 地址"
echo "   按回车继续，或按 Ctrl+C 取消..."
read

sudo certbot certonly \
    --webroot \
    --webroot-path=/var/www/certbot \
    -d $DOMAIN \
    --email admin@bithub.pro \
    --agree-tos \
    --no-eff-email \
    --force-renewal || {
        echo "证书获取失败，可能原因："
        echo "  1. 域名未正确解析到此服务器"
        echo "  2. 防火墙阻止了 80 端口"
        echo "  3. Nginx 配置有误"
        echo ""
        echo "请检查后重新运行脚本"
        exit 1
    }

# 部署完整的 Nginx 配置
echo ""
echo "8. 部署完整的 Nginx 配置..."
sudo cp $GATEWAY_DIR/$NGINX_CONF /etc/nginx/sites-available/$DOMAIN

# 测试配置
echo ""
echo "9. 测试最终配置..."
sudo nginx -t

# 重新加载 Nginx
echo ""
echo "10. 重新加载 Nginx..."
sudo systemctl reload nginx

# 配置证书自动续期
echo ""
echo "11. 配置证书自动续期..."
sudo systemctl enable certbot.timer
sudo systemctl start certbot.timer

# 显示服务状态
echo ""
echo "12. Nginx 服务状态："
sudo systemctl status nginx --no-pager

echo ""
echo "=== 部署完成 ==="
echo ""
echo "配置信息："
echo "  域名：        https://$DOMAIN"
echo "  健康检查：    https://$DOMAIN/health"
echo "  价格 API：    https://$DOMAIN/api/v1/price?symbol=BTCUSDT"
echo ""
echo "常用命令："
echo "  测试配置：    sudo nginx -t"
echo "  重载配置：    sudo systemctl reload nginx"
echo "  查看日志：    sudo tail -f /var/log/nginx/cryptoprice.access.log"
echo "  证书续期：    sudo certbot renew --dry-run"
echo ""
echo "注意事项："
echo "  1. 确保防火墙开放 80 和 443 端口"
echo "  2. 证书将在到期前自动续期"
echo "  3. 访问日志：/var/log/nginx/cryptoprice.access.log"
echo "  4. 错误日志：/var/log/nginx/cryptoprice.error.log"
