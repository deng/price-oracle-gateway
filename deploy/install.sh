#!/bin/bash

# Gateway 部署脚本 - 在服务器上运行

set -e

GATEWAY_DIR="/home/ubuntu/gateway"
SERVICE_NAME="gateway"

echo "=== Gateway 部署脚本 ==="
echo ""

# 检查是否在正确的目录
if [ ! -f "$GATEWAY_DIR/gateway" ]; then
    echo "错误：找不到 gateway 二进制文件"
    echo "请确保在正确的目录中运行此脚本"
    exit 1
fi

# 停止现有服务（如果正在运行）
echo "1. 停止现有服务..."
if systemctl is-active --quiet $SERVICE_NAME 2>/dev/null; then
    sudo systemctl stop $SERVICE_NAME
    echo "   服务已停止"
else
    echo "   服务未运行"
fi

# 安装 systemd 服务
echo ""
echo "2. 安装 systemd 服务..."
sudo cp $GATEWAY_DIR/gateway.service /etc/systemd/system/
sudo systemctl daemon-reload
echo "   服务已安装"

# 启用服务（开机自启）
echo ""
echo "3. 启用服务自动启动..."
sudo systemctl enable $SERVICE_NAME
echo "   已设置开机自启"

# 启动服务
echo ""
echo "4. 启动服务..."
sudo systemctl start $SERVICE_NAME

# 等待服务启动
sleep 2

# 检查服务状态
echo ""
echo "5. 服务状态："
sudo systemctl status $SERVICE_NAME --no-pager

echo ""
echo "=== 部署完成 ==="
echo ""
echo "常用命令："
echo "  查看状态：  sudo systemctl status $SERVICE_NAME"
echo "  查看日志：  sudo journalctl -u $SERVICE_NAME -f"
echo "  重启服务：  sudo systemctl restart $SERVICE_NAME"
echo "  停止服务：  sudo systemctl stop $SERVICE_NAME"
echo ""
echo "API 端点："
echo "  健康检查：  curl http://localhost:13012/health"
echo "  价格查询：  curl http://localhost:13012/api/v1/price?symbol=BTCUSDT"
