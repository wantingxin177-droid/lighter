# 构建阶段
FROM rust:1.75-slim as builder

WORKDIR /app

# 安装依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# 复制workspace文件
COPY Cargo.toml Cargo.lock ./
COPY shared ./shared
COPY backend/Cargo.toml ./backend/
RUN mkdir -p backend/src && echo "fn main() {}" > backend/src/main.rs

# 缓存依赖
RUN cargo build --release -p kin-backend

# 复制源代码
COPY backend/src ./backend/src
COPY backend/migrations ./backend/migrations

# 重新构建
RUN touch backend/src/main.rs
RUN cargo build --release -p kin-backend

# 运行阶段
FROM debian:bookworm-slim

WORKDIR /app

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# 复制二进制文件
COPY --from=builder /app/target/release/kin-backend ./
COPY --from=builder /app/backend/migrations ./migrations

# 非root用户
RUN useradd -m -u 1000 appuser && chown -R appuser:appuser /app
USER appuser

EXPOSE 8080

CMD ["./kin-backend"]
