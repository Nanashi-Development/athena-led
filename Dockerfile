FROM rust:slim

# 安装交叉编译工具和必要的构建工具
RUN apt-get update && apt-get install -y \
    gcc-aarch64-linux-gnu \
    g++-aarch64-linux-gnu \
    musl-tools \
    pkg-config \
    --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*

# 添加目标架构
RUN rustup target add aarch64-unknown-linux-musl

# 创建新的用户和工作目录
RUN useradd -m -u 1000 rust
USER rust
WORKDIR /home/rust/athena-led

# 设置交叉编译环境变量
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-linux-gnu-gcc \
    CC_aarch64_unknown_linux_musl=aarch64-linux-gnu-gcc \
    CXX_aarch64_unknown_linux_musl=aarch64-linux-gnu-g++

# 首先复制 Cargo.toml 和 Cargo.lock
COPY --chown=rust:rust Cargo.toml Cargo.lock ./

# 创建一个虚拟的 src 目录和主文件，以便缓存依赖项
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --target aarch64-unknown-linux-musl --release && \
    rm -rf src target/aarch64-unknown-linux-musl/release/deps/athena_led*

# 复制实际的源代码
COPY --chown=rust:rust src ./src/

# 构建项目
RUN cargo build --target aarch64-unknown-linux-musl --release && \
    mkdir -p /home/rust/release && \
    cp target/aarch64-unknown-linux-musl/release/athena-led /home/rust/release/

# 保持容器运行
CMD ["tail", "-f", "/dev/null"]
