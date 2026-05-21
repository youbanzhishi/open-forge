# OpenForge 技术架构方案 v1

> 作者：产品经理（初稿，待系统开发者评审）
> 日期：2026-05-20
> 状态：初稿

---

## 一、整体架构 — 四层七模块

```
┌──────────────────────────────────────────────────────────────────┐
│                      接口层 (Interface Layer)                     │
│                                                                    │
│  ┌───────────┐  ┌───────────┐  ┌────────────┐  ┌─────────────┐  │
│  │ REST API  │  │ WebSocket │  │ Web Studio │  │  CLI Shell  │  │
│  │ /api/v1   │  │ /ws/*     │  │ (TS+Canvas)│  │ YAML/JSON   │  │
│  │ Agent首选 │  │ 实时事件流 │  │ 人类首选   │  │ 本地调试    │  │
│  └─────┬─────┘  └─────┬─────┘  └─────┬──────┘  └──────┬──────┘  │
│        └──────────────┴──────┬───────┴─────────────────┘         │
│                              │ Forge Bus (消息总线)               │
└──────────────────────────────┼──────────────────────────────────┘
                               │
┌──────────────────────────────┼──────────────────────────────────┐
│                    编排层 (Orchestration Layer)                    │
│                              │                                    │
│  ┌──────────────┐  ┌────────┴───────┐  ┌──────────────────────┐ │
│  │ Forge Core   │  │ Script Forge   │  │ Extension Registry   │ │
│  │ AI创作编排   │  │ 逻辑引擎       │  │ 扩展注册中心         │ │
│  │ 项目/场景/   │  │ YAML→Runtime   │  │ 新功能=注册扩展     │ │
│  │ 资产/构建    │  │ 逻辑编译执行   │  │ 架构永远不需要改     │ │
│  └──────┬───────┘  └───────┬────────┘  └──────────┬───────────┘ │
│         └──────────────────┴───────────────────────┘             │
│                            │ Asset Forge                         │
│                     ┌──────┴──────┐                              │
│                     │ 资产生成管线 │                              │
│                     │ AI生图/3D   │                              │
│                     │ 标准化入库   │                              │
│                     └─────────────┘                              │
└──────────────────────────────────────────────────────────────────┘
                               │
┌──────────────────────────────┼──────────────────────────────────┐
│                    运行时层 (Runtime Layer)                       │
│                              │                                    │
│          ┌───────────────────┴───────────────────┐              │
│          │         Runtime Abstraction            │              │
│          │  init/update/render/event/shutdown     │              │
│          └───────────┬───────────────┬───────────┘              │
│                      │               │                           │
│         ┌────────────┴──┐   ┌────────┴──────────┐              │
│         │ Light Runtime │   │ Cloud Render Bridge│              │
│         │ Canvas/WebGL  │   │ Godot Headless    │              │
│         │ 2D+简单3D     │   │ Unreal PIC        │              │
│         │ 浏览器直接跑  │   │ 云渲染流式推送    │              │
│         └───────────────┘   └───────────────────┘              │
└──────────────────────────────────────────────────────────────────┘
                               │
┌──────────────────────────────┼──────────────────────────────────┐
│                    构建层 (Build Layer)                           │
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌────────────────────────┐│
│  │ Build Pipeline│  │ Package Mgr  │  │  Platform Exporters   ││
│  │ 一键构建     │  │ 扩展/资产包  │  │  Web/Win/Mac/Linux    ││
│  └──────────────┘  └──────────────┘  └────────────────────────┘│
└──────────────────────────────────────────────────────────────────┘
```

---

## 二、模块职责

### 2.1 Forge Core — AI创作编排核心

**一句话**：AI Agent的游戏开发统一入口

| 职责 | 说明 |
|------|------|
| 项目管理 | 项目CRUD、版本管理、快照/回滚 |
| 场景编排 | 场景图CRUD、节点树管理、组件挂载 |
| 资产调度 | 资产生成请求→Asset Forge→入库→场景引用 |
| 逻辑调度 | 游戏逻辑描述→Script Forge→编译→绑定到场景 |
| 构建调度 | 构建请求→Build Pipeline→产物→发布 |
| 事件总线 | Forge Bus：模块间解耦通信 |

**关键设计**：
- Forge Core本身不渲染、不执行逻辑、不生成资产——它只**编排**
- 所有操作通过Forge Bus发布事件，模块间松耦合
- AI Agent只和Forge Core交互，不需要了解下游实现

### 2.2 Script Forge — 游戏逻辑引擎

**一句话**：YAML/JSON描述的游戏逻辑→可运行代码

```yaml
# 示例：一个简单角色的游戏逻辑
entity: player
components:
  - type: Transform
    position: [0, 0]
    speed: 200
  
  - type: SpriteRenderer
    asset: "player-idle"
    flip_horizontal: true
  
  - type: Collider
    shape: rect
    size: [32, 48]
  
  - type: InputController
    bindings:
      arrow_up: move_up
      arrow_down: move_down
      arrow_left: move_left
      arrow_right: move_right
      space: jump

scripts:
  on_move_up:
    - transform.velocity.y = -speed
  on_jump:
    - if: transform.grounded
      then: transform.velocity.y = -jump_force
  
  on_collision_enter:
    - if: other.tag == "coin"
      then: 
        - game.score += 10
        - other.destroy()
    - if: other.tag == "enemy"
      then:
        - game.lives -= 1
        - transform.position = game.checkpoint
```

**编译管线**：
```
YAML/JSON → AST → 类型检查 → Runtime Bytecode → 执行
                                      ↓
                            Light Runtime (Phase 1)
                            Godot GDScript (Phase 2)
```

**设计约束**：
- 逻辑描述语言必须是YAML/JSON（AI友好+人类可读）
- Phase 1编译为内部字节码，Light Runtime直接执行
- Phase 2可编译为GDScript，Godot直接运行
- 不支持图灵完备——循环/递归有上限，防止无限循环卡死服务器

### 2.3 Asset Forge — 资产生成管线

**一句话**：AI生成资产→标准化入库→场景引用

```
AI生图请求 → image_generate → 标准化处理 → 资产库
                                                  ↓
AI生3D请求 → 未来3D生图 → glTF标准化 → 资产库 → 场景引用
AI生音效请求 → 音频生成 → 格式标准化 → 资产库
```

**资产入库标准**：
| 类型 | 格式 | 元数据 |
|------|------|--------|
| 2D精灵 | PNG (atlas) | size, pivot, frames, fps |
| TileMap | JSON | grid_size, tiles[], layers[] |
| 3D模型 | glTF 2.0 | bbox, materials, animations[] |
| 音效 | OGG/MP3 | duration, loop, volume |
| 音乐 | OGG/MP3 | duration, bpm, loop |
| 字体 | TTF/OTF | family, weight |

### 2.4 Extension Registry — 扩展注册中心

**一句话**：新功能=注册扩展，架构本身永远不需要改

复用体系已有的四柱模型（参考 `共享知识/设计模式/extension-registry.md`）：

| 柱 | OpenForge映射 | 注册什么 |
|----|---------------|---------|
| Action API | Capability API | 注册新能力（资产生成器/逻辑节点/AI助手） |
| Condition API | Query API | 注册新查询（资产检索/场景分析/性能诊断） |
| Hook API | Hook API | 注册新拦截器（构建前校验/发布前审批/运行时监控） |
| Protocol API | Runtime API | 注册新运行时（Godot/Unreal/自定义引擎） |

**注册示例**：
```yaml
# openforge-extensions.yaml
extensions:
  runtimes:
    - id: light-runtime
      type: builtin
      capabilities: [2d, canvas, webgl, particles]
    
    - id: godot-headless
      type: external
      capabilities: [3d, physics, ray-tracing]
      config:
        godot_path: /usr/bin/godot
        render_mode: headless
        stream_fps: 30

  capabilities:
    - id: procedural-gen
      type: asset-generator
      input_schema: world-description.yaml
      output_type: scene-graph

  hooks:
    - event: build_start
      handler: capabilities.procedural-gen.on_build_start
      priority: 10
```

**关键约束**：
- 核心层零业务逻辑——只做通用调度
- 一切扩展通过Registry——注册即用，核心0改动
- Runtime Abstraction是关键抽象——所有运行时必须实现统一接口

### 2.5 Light Runtime — 轻量Web运行时

**一句话**：2D游戏在浏览器直接跑，不依赖云渲染

**Runtime Abstraction接口**：
```rust
trait GameRuntime: Send + Sync {
    fn init(&mut self, scene: &SceneGraph, assets: &AssetStore) -> Result<()>;
    fn update(&mut self, dt: f64, input: &InputState) -> Result<()>;
    fn render(&self, ctx: &RenderContext) -> Result<()>;
    fn handle_event(&mut self, event: GameEvent) -> Result<()>;
    fn shutdown(&mut self) -> Result<()>;
    
    fn capabilities(&self) -> RuntimeCapabilities;
}

struct RuntimeCapabilities {
    dim_2d: bool,
    dim_3d: bool,
    physics: bool,
    particles: bool,
    max_sprites: usize,
    max_fps: usize,
}
```

**Light Runtime实现**（TypeScript）：
```typescript
// 浏览器端运行时
class LightRuntime implements GameRuntime {
  canvas: HTMLCanvasElement;
  ctx: CanvasRenderingContext2D | WebGL2RenderingContext;
  
  // 渲染管线
  spriteBatch: SpriteBatch;     // 批量精灵渲染
  particleSystem: Particles;    // 粒子系统
  tilemapRenderer: Tilemap;     // TileMap渲染
  
  // 性能目标
  maxSprites: 1000;
  targetFPS: 60;
}
```

**Phase 1渲染能力清单**：
| 能力 | 实现 | 性能目标 |
|------|------|---------|
| 精灵渲染 | Canvas 2D SpriteBatch | 1000+精灵@60fps |
| TileMap | 分块渲染+视口裁剪 | 100x100地图 |
| 粒子系统 | Canvas 2D | 500粒子@60fps |
| 基础动画 | Sprite帧动画 | 30fps逐帧 |
| 简单UI | HTML Overlay | — |
| 音频 | Web Audio API | 8通道混音 |

### 2.6 Cloud Render Bridge — 云渲染桥接

**一句话**：3D游戏通过Godot/Unreal云渲染，Web端流式接收

Phase 1不实现，预留接口。Phase 2先桥接Godot headless。

```rust
struct CloudRenderBridge {
    runtime: Arc<dyn GameRuntime>,
    stream_config: StreamConfig,
}

struct StreamConfig {
    resolution: (u32, u32),  // 1920x1080
    fps: u32,                 // 30
    codec: StreamCodec,       // H264/WebRTC
    latency_target_ms: u32,   // 50
}
```

### 2.7 Web Studio — Web协作前端

**一句话**：任何设备的游戏开发入口

**功能分区**：
| 区域 | 功能 | 设备适配 |
|------|------|---------|
| 预览区 | 实时游戏画面 | 全屏(手机)/右侧(桌面) |
| 场景树 | 场景节点层级 | 抽屉(手机)/左侧(桌面) |
| 编辑区 | YAML/属性编辑 | 全屏编辑(手机)/底部(桌面) |
| 控制台 | 日志/错误/输出 | 折叠面板 |
| 工具栏 | 构建/发布/设置 | 底部(手机)/顶部(桌面) |

**实时通信**：
- 场景修改 → WebSocket → Forge Core → 事件广播 → 所有客户端同步
- 游戏预览 → Light Runtime本地渲染 / Cloud Render流推送

---

## 三、Rust Workspace Crate拆分

```
open-forge/
├── Cargo.toml              # workspace根
├── crates/
│   ├── forge-core/         # 核心编排：项目/场景/资产/构建的管理与调度
│   │   └── src/
│   │       ├── project.rs  # 项目CRUD+版本+快照
│   │       ├── scene.rs    # 场景图管理
│   │       ├── asset.rs    # 资产引用管理（不含生成）
│   │       ├── build.rs    # 构建调度
│   │       └── bus.rs      # Forge Bus消息总线
│   │
│   ├── forge-api/          # RESTful + WebSocket接口层
│   │   └── src/
│   │       ├── rest.rs     # Axum REST API
│   │       ├── ws.rs       # WebSocket实时事件
│   │       └── dto.rs      # 请求/响应数据结构
│   │
│   ├── script-forge/       # 游戏逻辑引擎：YAML/JSON→AST→Runtime
│   │   └── src/
│   │       ├── parser.rs   # YAML/JSON→AST
│   │       ├── checker.rs  # 类型检查+约束验证
│   │       ├── compiler.rs # AST→Runtime Bytecode
│   │       └── vm.rs       # 字节码虚拟机
│   │
│   ├── asset-forge/        # 资产生成管线
│   │   └── src/
│   │       ├── pipeline.rs # 生成请求→标准化→入库
│   │       ├── store.rs    # 资产库管理
│   │       └── processor.rs# 格式标准化/裁剪/打包
│   │
│   ├── forge-registry/     # Extension Registry（复用open-registry模式）
│   │   └── src/
│   │       ├── registry.rs # 注册中心核心
│   │       ├── capability.rs # Capability API
│   │       ├── query.rs    # Query API
│   │       ├── hook.rs     # Hook API
│   │       └── runtime_registry.rs # Runtime API
│   │
│   ├── forge-runtime/      # Runtime Abstraction + Light Runtime绑定
│   │   └── src/
│   │       ├── trait.rs    # GameRuntime trait定义
│   │       ├── scene_graph.rs # 场景图数据结构
│   │       └── light.rs   # Light Runtime的Rust端（场景图→JSON→前端）
│   │
│   └── forge-build/        # 构建发布管线
│       └── src/
│           ├── pipeline.rs # 构建流程编排
│           ├── web.rs      # Web包导出
│           └── platform.rs # 多平台导出（Phase 2+）
│
├── web-studio/             # TypeScript前端
│   ├── src/
│   │   ├── app/            # 主应用框架
│   │   ├── preview/        # 游戏预览区
│   │   ├── editor/         # YAML/属性编辑器
│   │   ├── scene-tree/     # 场景树面板
│   │   └── runtime/        # Light Runtime (Canvas/WebGL)
│   ├── package.json
│   └── tsconfig.json
│
├── docs/
│   ├── PRD.md
│   ├── knowledge/
│   └── adr/
│
└── scripts/
    └── dev.sh              # 开发启动脚本
```

**crate依赖关系**：
```
forge-api → forge-core → forge-registry
                    → script-forge
                    → asset-forge
                    → forge-runtime → forge-build
```

**与体系共享crate的规划**：
- `forge-registry` 未来可提取为 `open-registry`，与OpenDAW/OpenLink共享
- `forge-core/bus.rs` 未来可提取为 `open-bus`，与OpenDAW共享

---

## 四、API设计

### 4.1 RESTful API

**Base Path**: `/api/v1`

#### 项目管理
```
POST   /projects                    创建项目
GET    /projects                    列出项目
GET    /projects/:id                获取项目详情
PUT    /projects/:id                更新项目
DELETE /projects/:id                删除项目
POST   /projects/:id/snapshot       创建快照
POST   /projects/:id/rollback       回滚到快照
```

#### 场景管理
```
POST   /projects/:id/scenes         创建场景
GET    /projects/:id/scenes         列出场景
GET    /projects/:id/scenes/:sid    获取场景详情（含完整节点树）
PUT    /projects/:id/scenes/:sid    更新场景
DELETE /projects/:id/scenes/:sid    删除场景

POST   /projects/:id/scenes/:sid/nodes       添加节点
PUT    /projects/:id/scenes/:sid/nodes/:nid  更新节点
DELETE /projects/:id/scenes/:sid/nodes/:nid  删除节点
```

#### 资产管理
```
POST   /projects/:id/assets/generate   AI生成资产
GET    /projects/:id/assets            列出资产
GET    /projects/:id/assets/:aid       获取资产详情
PUT    /projects/:id/assets/:aid       更新资产元数据
DELETE /projects/:id/assets/:aid       删除资产
```

#### 逻辑管理
```
POST   /projects/:id/scripts          创建逻辑脚本
GET    /projects/:id/scripts          列出脚本
GET    /projects/:id/scripts/:sid     获取脚本详情
PUT    /projects/:id/scripts/:sid     更新脚本
DELETE /projects/:id/scripts/:sid     删除脚本
POST   /projects/:id/scripts/:sid/compile  编译脚本
```

#### 构建发布
```
POST   /projects/:id/build            触发构建
GET    /projects/:id/builds           列出构建记录
GET    /projects/:id/builds/:bid      获取构建状态
GET    /projects/:id/builds/:bid/download  下载构建产物
```

#### 扩展管理
```
GET    /extensions                    列出已注册扩展
POST   /extensions                    注册新扩展
DELETE /extensions/:id                注销扩展
GET    /extensions/:id/capabilities   获取扩展能力
```

### 4.2 WebSocket事件

**Path**: `/ws/v1`

```json
// 客户端→服务器
{ "type": "scene.update",  "data": { "node_id": "...", "changes": {} } }
{ "type": "game.input",    "data": { "keys": [], "mouse": {} } }
{ "type": "build.start",   "data": { "project_id": "..." } }

// 服务器→客户端
{ "type": "scene.changed",  "data": { "scene_id": "...", "diff": {} } }
{ "type": "game.frame",     "data": { "frame_data": "..." } }  // 云渲染时
{ "type": "build.progress", "data": { "percent": 45, "step": "compiling" } }
{ "type": "asset.ready",    "data": { "asset_id": "...", "url": "..." } }
{ "type": "error",          "data": { "code": "...", "message": "..." } }
```

---

## 五、数据模型

### 5.1 场景图（核心数据结构）

```rust
struct SceneGraph {
    id: String,
    name: String,
    root: SceneNode,
}

struct SceneNode {
    id: String,
    name: String,
    components: Vec<Component>,
    children: Vec<SceneNode>,
    transform: Transform,
}

enum Component {
    SpriteRenderer { asset_id: String, flip_h: bool, flip_v: bool },
    Collider { shape: Shape, size: Vec2, is_trigger: bool },
    Rigidbody { mass: f64, gravity_scale: f64, velocity: Vec2 },
    AudioPlayer { asset_id: String, volume: f64, loop: bool },
    Script { script_id: String, params: serde_json::Value },
    Camera { zoom: f64, follow_target: Option<String> },
    ParticleEmitter { config: ParticleConfig },
    Custom { type_id: String, data: serde_json::Value },
}

struct Transform {
    position: Vec2,
    rotation: f64,  // degrees
    scale: Vec2,
}
```

### 5.2 项目结构

```rust
struct Project {
    id: String,
    name: String,
    description: String,
    created_at: DateTime,
    updated_at: DateTime,
    
    scenes: Vec<String>,        // scene IDs
    assets: Vec<String>,        // asset IDs
    scripts: Vec<String>,       // script IDs
    
    settings: ProjectSettings,
    snapshots: Vec<Snapshot>,
}

struct ProjectSettings {
    resolution: (u32, u32),     // (1280, 720)
    target_fps: u32,            // 60
    runtime: String,            // "light-runtime"
    background_color: String,   // "#000000"
    physics: PhysicsSettings,
}
```

---

## 六、Phase 1开发路线

### Sprint 1（2周）：骨架搭建
- [ ] Rust workspace初始化+7个crate骨架
- [ ] forge-core：项目CRUD + 场景图数据结构
- [ ] forge-api：Axum骨架 + 健康检查
- [ ] Web Studio：React/Vite项目初始化 + 基本布局

### Sprint 2（2周）：场景可编辑
- [ ] forge-core：场景图CRUD + Forge Bus
- [ ] forge-api：项目+场景REST API完整实现
- [ ] Web Studio：场景树展示 + YAML编辑器 + WebSocket实时同步

### Sprint 3（2周）：能跑游戏
- [ ] script-forge：YAML解析 + AST + 基础VM（移动/碰撞/计分）
- [ ] forge-runtime：SceneGraph→JSON序列化 + Runtime Abstraction trait
- [ ] Web Studio：Light Runtime（Canvas精灵渲染 + 粒子 + TileMap）

### Sprint 4（2周）：AI可创作
- [ ] asset-forge：AI生图→标准化入库
- [ ] script-forge：完整逻辑编译（条件/事件/组件交互）
- [ ] forge-registry：Extension Registry基础实现
- [ ] AI Agent通过REST API从零创建可玩2D游戏

### Sprint 5（1周）：构建发布+验收
- [ ] forge-build：Web包一键导出
- [ ] 全链路测试：AI描述→场景生成→逻辑绑定→预览→构建→可玩
- [ ] 性能验收：100+精灵@60fps

---

## 七、技术选型清单

| 选型 | 选择 | 理由 |
|------|------|------|
| HTTP框架 | Axum 0.7 | 与OpenLink/OpenVault一致，团队经验复用 |
| 异步运行时 | Tokio | Rust标准选择 |
| 序列化 | serde + serde_json | Rust标准，与YAML互转 |
| YAML解析 | serde_yaml | 简单可靠 |
| WebSocket | axum::extract::ws | Axum内置 |
| 前端框架 | React + Vite | 生态成熟，组件丰富 |
| Canvas渲染 | Canvas 2D API | Phase 1够用，Phase 2按需升WebGL |
| 数据存储 | SQLite (rusqlite) | 轻量、零配置、单文件 |
| 构建工具 | just (justfile) | 与OpenDAW一致 |

---

## 八、关键设计决策（需ADR）

| ADR | 决策 | 初步倾向 | 待确认 |
|-----|------|---------|--------|
| ADR-002 | API风格 | RESTful+WebSocket | GraphQL对前端更友好但AI直接用REST更简单 |
| ADR-003 | 逻辑描述语言 | YAML（主）+ JSON（备） | 是否需要自定义DSL？ |
| ADR-004 | Extension Registry实现 | 复用四柱模型+YAML配置 | 是否需要动态加载.so/.dll？ |
| ADR-005 | 数据存储 | SQLite | 项目数据是否需要实时协作CRDT？ |
| ADR-006 | 前端框架 | React+Vite | Vue/Svelte的优劣？ |
