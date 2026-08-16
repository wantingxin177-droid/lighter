# 构建阶段
FROM rust:1.75-slim as builder

WORKDIR /app

# 安装依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    clang \
    llvm \
    && rm -rf /var/lib/apt/lists/*

# 安装 trunk
RUN cargo install trunk wasm-bindgen-cli

# 复制workspace文件
COPY Cargo.toml Cargo.lock ./
COPY shared ./shared
COPY frontend/Cargo.toml ./frontend/
RUN mkdir -p frontend/src && echo "fn main() {}" > frontend/src/main.rs

# 复制源代码
COPY frontend/src ./frontend/src
COPY frontend/style ./frontend/style
COPY frontend/index.html ./frontend/

# 构建WASM
WORKDIR /app/frontend
RUN rustup target add wasm32-unknown-unknown
RUN trunk build --release

# 运行阶段 - Nginx
FROM nginx:alpine

# 复制构建结果
COPY --from=builder /app/frontend/dist /usr/share/nginx/html

# 复制nginx配置
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
