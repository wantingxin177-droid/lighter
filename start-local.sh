#!/bin/bash

set -e

echo "🚀 Kin High Performance - 本地启动脚本"
echo "========================================"

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 检查命令是否存在
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# 安装依赖
echo -e "${YELLOW}📦 检查依赖...${NC}"

if ! command_exists cargo; then
    echo -e "${RED}❌ Rust/Cargo 未安装${NC}"
    echo "请安装 Rust: https://rustup.rs/"
    exit 1
fi

if ! command_exists trunk; then
    echo -e "${YELLOW}📥 安装 trunk...${NC}"
    cargo install trunk
fi

if ! command_exists sqlx; then
    echo -e "${YELLOW}📥 安装 sqlx-cli...${NC}"
    cargo install sqlx-cli --no-default-features --features native-tls,postgres
fi

# 检查 WASM 目标
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo -e "${YELLOW}📥 添加 WASM 目标...${NC}"
    rustup target add wasm32-unknown-unknown
fi

echo -e "${GREEN}✅ 依赖检查完成${NC}"

# 启动数据库（使用 Docker，如果可用）
echo -e "${YELLOW}🐳 尝试启动数据库...${NC}"

if command_exists docker; then
    # 尝试启动 PostgreSQL
    if ! docker ps | grep -q "kin-postgres"; then
        echo "启动 PostgreSQL..."
        docker run -d --name kin-postgres --rm \
            -e POSTGRES_USER=postgres \
            -e POSTGRES_PASSWORD=postgres \
            -e POSTGRES_DB=kin \
            -p 5432:5432 \
            postgres:16-alpine 2>/dev/null || echo "⚠️  PostgreSQL 启动失败，请手动启动"
    fi
    
    # 尝试启动 Redis
    if ! docker ps | grep -q "kin-redis"; then
        echo "启动 Redis..."
        docker run -d --name kin-redis --rm \
            -p 6379:6379 \
            redis:7-alpine 2>/dev/null || echo "⚠️  Redis 启动失败，请手动启动"
    fi
    
    echo -e "${GREEN}✅ 数据库启动完成${NC}"
    sleep 3
else
    echo -e "${YELLOW}⚠️  Docker 不可用，请手动配置数据库${NC}"
fi

# 数据库迁移
echo -e "${YELLOW}🔄 执行数据库迁移...${NC}"
cd backend
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/kin
sqlx migrate run 2>/dev/null || echo "⚠️  迁移失败，数据库可能未启动"
cd ..

echo -e "${GREEN}✅ 准备工作完成！${NC}"
echo ""
echo -e "${YELLOW}📝 启动命令:${NC}"
echo ""
echo "1. 启动后端（终端1）:"
echo -e "   ${GREEN}cd backend && cargo run${NC}"
echo ""
echo "2. 启动前端（终端2）:"
echo -e "   ${GREEN}cd frontend && trunk serve${NC}"
echo ""
echo "3. 访问:"
echo -e "   ${GREEN}http://localhost:8080${NC}"
echo ""

# 询问是否自动启动
read -p "是否自动启动后端? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${GREEN}🚀 启动后端...${NC}"
    cd backend && cargo run
fi
