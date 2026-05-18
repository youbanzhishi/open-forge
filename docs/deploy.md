# OpenForge 部署指南

## 环境要求

- Rust 1.95.0+
- 内存: 512MB+
- 磁盘: 1GB+
- 端口: 3000（可配置）

## 编译

```bash
export CARGO_TARGET_DIR=/tmp/openforge-target
export CARGO_BUILD_JOBS=2
cargo build --release -p forge-core
```

## 启动

```bash
PORT=3000 ./target/release/forge-core
```

## Docker（Phase 2）

```dockerfile
FROM rust:1.95-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release -p forge-core

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/forge-core /usr/local/bin/
EXPOSE 3000
CMD ["forge-core"]
```

## 健康检查

```bash
curl http://localhost:3000/api/v1/projects
```

## 配置

| 环境变量 | 默认值 | 说明 |
|---------|--------|------|
| PORT | 3000 | 监听端口 |
| RUST_LOG | info | 日志级别 |
