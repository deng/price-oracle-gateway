#!/bin/bash

# Price Oracle Gateway 启动脚本

# 设置日志级别
export RUST_LOG=${RUST_LOG:-info}

# 获取脚本所在目录
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# 加载环境变量
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
fi

# 启动服务
echo "Starting price-oracle-gateway service..."
./price-oracle-gateway
