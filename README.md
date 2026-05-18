# Open Forge

AI-First 游戏创作平台——AI Agent 在服务器上写游戏，人类通过 Web 在任何设备上查看和参与开发。

## 架构

```
Web Studio (前端)
    ↓
Forge Core (AI 编排层) — RESTful + WebSocket
    ↓
Script Forge / Asset Forge / Extension Registry / Build Pipeline
    ↓
Runtime Abstraction (统一运行时接口)
    ↓
Light Runtime (自研 2D) / Cocos Bridge / Godot Bridge
```

## Crate 结构

| Crate | 职责 |
|-------|------|
| `forge-core` | AI 编排层，REST+WS API 入口 |
| `script-forge` | YAML/JSON → 可运行游戏逻辑 |
| `extension-registry` | 四柱扩展注册中心（Action/Condition/Hook/Component） |
| `runtime-abstraction` | 统一运行时接口（init/update/render/event/shutdown） |
| `runtime-light` | 自研 Canvas/WebGL 2D 运行时 |
| `asset-forge` | 资产管理管线 |
| `build-pipeline` | 构建发布管线 |

## 开发

```bash
# 编译
cargo build

# 测试
cargo test

# 启动
cargo run -p forge-core
```

## 环境要求

- Rust 1.95.0+
- 编译输出: `CARGO_TARGET_DIR=/tmp/openforge-target`
- 编译并发: `CARGO_BUILD_JOBS=2`

## License

MIT
