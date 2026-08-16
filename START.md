# 🚀 快速启动指南

## 方式一：Docker Compose（推荐）

### 安装 Docker Compose
```bash
# 下载 Docker Compose v2
mkdir -p ~/.docker/cli-plugins/
curl -SL https://github.com/docker/compose/releases/download/v2.23.0/docker-compose-linux-x86_64 -o ~/.docker/cli-plugins/docker-compose
chmod +x ~/.docker/cli-plugins/docker-compose

# 验证
docker compose version
```

### 启动项目
```bash
cd /home/yyypc/桌面/Kin-high-performance
docker-compose up -d
```

访问: http://localhost

---

## 方式二：本地运行（开发模式）

### 1. 安装依赖工具

```bash
# 安装 trunk (WASM构建工具)
cargo install trunk

# 安装 sqlx-cli (数据库迁移)
cargo install sqlx-cli --no-default-features --features native-tls,postgres

# 添加 WASM 目标
rustup target add wasm32-unknown-unknown
```

### 2. 启动数据库

```bash
# 使用 Docker 启动 PostgreSQL 和 Redis
docker run -d --name kin-postgres \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=kin \
  -p 5432:5432 \
  postgres:16-alpine

docker run -d --name kin-redis \
  -p 6379:6379 \
  redis:7-alpine
```

### 3. 数据库迁移

```bash
cd backend
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/kin
cargo sqlx migrate run
```

### 4. 启动后端

```bash
cd backend
cargo run
```

后端启动在: http://localhost:8080

### 5. 启动前端（新终端）

```bash
cd frontend
trunk serve
```

前端启动在: http://localhost:8080

---

## 方式三：一键脚本

创建 `start.sh`:

```bash
#!/bin/bash

# 启动数据库
docker run -d --name kin-postgres --rm \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=kin \
  -p 5432:5432 \
  postgres:16-alpine 2>/dev/null || echo "PostgreSQL already running"

docker run -d --name kin-redis --rm \
  -p 6379:6379 \
  redis:7-alpine 2>/dev/null || echo "Redis already running"

# 等待数据库启动
sleep 3

# 数据库迁移
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/kin
cd backend && cargo sqlx migrate run 2>/dev/null || true

# 启动后端（后台）
cargo run &
BACKEND_PID=$!

# 启动前端
cd ../frontend
trunk serve

# 清理
kill $BACKEND_PID 2>/dev/null
docker stop kin-postgres kin-redis 2>/dev/null
```

运行:
```bash
chmod +x start.sh
./start.sh
```

---

## 验证运行

### 测试后端 API
```bash
# 健康检查
curl http://localhost:8080/api/v1/health

# 获取最新区块
curl http://localhost:8080/api/v1/blocks/latest

# 获取最近交易
curl http://localhost:8080/api/v1/txs/recent?limit=10
```

### 测试 WebSocket
```bash
# 使用 wscat 测试
npm install -g wscat
wscat -c ws://localhost:8080/api/v1/ws
```

---

## 常见问题

### 1. 端口被占用
```bash
# 查找占用 8080 的进程
lsof -i :8080
# 或
netstat -tulpn | grep 8080

# 停止进程
kill -9 <PID>
```

### 2. 数据库连接失败
```bash
# 检查 PostgreSQL 是否运行
docker ps | grep postgres

# 查看日志
docker logs kin-postgres
```

### 3. WASM 编译失败
```bash
# 重新安装 wasm 目标
rustup target add wasm32-unknown-unknown --force

# 清理缓存
cargo clean
cd frontend && trunk clean
```

### 4. trunk 命令找不到
```bash
# 确保 cargo bin 目录在 PATH 中
export PATH="$HOME/.cargo/bin:$PATH"

# 重新安装
cargo install trunk
```

---

## 性能测试

```bash
# 安装 wrk
sudo apt-get install wrk  # Ubuntu/Debian
brew install wrk          # macOS

# 压力测试
wrk -t4 -c100 -d30s http://localhost:8080/api/v1/health
```

---

## 停止服务

```bash
# 如果使用 Docker Compose
docker-compose down

# 如果使用本地运行
# 1. 按 Ctrl+C 停止前后端
# 2. 停止数据库
docker stop kin-postgres kin-redis
```
