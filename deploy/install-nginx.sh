#!/bin/bash

# Nginx 配置安装脚本
# 在服务器上运行

set -e

DOMAIN="cryptoprice.bithub.pro"
CONFIG_FILE="nginx-cryptoprice.conf"
NGINX_AVAILABLE="/etc/nginx/sites-available/$DOMAIN"
NGINX_ENABLED="/etc/nginx/sites-enabled/$DOMAIN"

echo "=== Nginx 配置安装脚本 ==="
echo ""

# 检查 Nginx 是否已安装
if ! command -v nginx &> /dev/null; then
    echo "Nginx 未安装，正在安装..."
    sudo apt update
    sudo apt install -y nginx
    echo "Nginx 安装完成"
else
    echo "✓ Nginx 已安装"
fi

# 备份现有配置（如果存在）
if [ -f "$NGINX_AVAILABLE" ]; then
    echo ""
    echo "备份现有配置..."
    sudo cp "$NGINX_AVAILABLE" "${NGINX_AVAILABLE}.backup.$(date +%Y%m%d_%H%M%S)"
fi

# 复制配置文件
echo ""
echo "安装配置文件..."
sudo cp /home/ubuntu/gateway/price-oracle/$CONFIG_FILE "$NGINX_AVAILABLE"
echo "✓ 配置文件已复制到 $NGINX_AVAILABLE"

# 创建软链接
echo ""
echo "启用站点..."
sudo ln -sf "$NGINX_AVAILABLE" "$NGINX_ENABLED"
echo "✓ 站点已启用"

# 测试配置
echo ""
echo "测试 Nginx 配置..."
if sudo nginx -t; then
    echo "✓ 配置测试通过"
else
    echo "✗ 配置测试失败，请检查配置文件"
    exit 1
fi

# 重新加载 Nginx
echo ""
echo "重新加载 Nginx..."
sudo systemctl reload nginx
echo "✓ Nginx 已重新加载"

# 检查 Nginx 状态
echo ""
echo "Nginx 状态："
sudo systemctl status nginx --no-pager | head -n 10

echo ""
echo "=== 安装完成 ==="
echo ""
echo "域名配置："
echo "  域名: $DOMAIN"
echo "  后端: http://127.0.0.1:13012"
echo ""
echo "测试命令："
echo "  curl -s http://$DOMAIN/health | jq ."
echo "  curl -s 'http://$DOMAIN/api/v1/price?symbol=BTCUSDT' | jq ."
echo ""
echo "注意："
echo "  1. 确保域名 DNS 已正确解析到服务器 IP"
echo "  2. 确保服务器防火墙允许 80 端口"
echo "  3. Gateway 服务必须正在运行"
