.PHONY: build run test clean dev docker-up docker-down migrate

# 默认目标
all: build

# 构建整个项目
build:
	cargo build --release

# 运行后端
dev-backend:
	cd backend && cargo run

# 运行前端
dev-frontend:
	cd frontend && trunk serve

# 运行测试
test:
	cargo test --workspace

# 清理构建文件
clean:
	cargo clean
	rm -rf frontend/dist

# 数据库迁移
migrate:
	cd backend && cargo sqlx migrate run

# Docker 操作
docker-up:
	docker-compose up -d

docker-down:
	docker-compose down

docker-build:
	docker-compose build

docker-logs:
	docker-compose logs -f

# 部署到生产
deploy: docker-build docker-up

# 基准测试
bench:
	cd backend && cargo bench
	cd wasm && wasm-pack build --target web

# 格式化代码
fmt:
	cargo fmt --all

# 代码检查
lint:
	cargo clippy --all-targets --all-features

# 安装依赖
install:
	cargo install trunk wasm-bindgen-cli sqlx-cli
	rustup target add wasm32-unknown-unknown
