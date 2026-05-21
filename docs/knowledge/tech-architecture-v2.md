# OpenForge 技术架构方案 v2（定稿）

> 合并：产品经理v1 + 系统开发者v1
> 日期：2026-05-21
> 状态：定稿
> 对比评审：[review-pm-vs-dev.md](./review-pm-vs-dev.md)

---

## 一、整体架构 — 四层八模块

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
│  │ 项目/场景/   │  │ YAML→Runtime   │  │ 四柱模型             │ │
│  │ 资产/构建    │  │ 逻辑编译执行   │  │ 新功能=注册扩展     │ │
│  └──────┬───────┘  └───────┬────────┘  └──────────┬───────────┘ │
│         └──────────────────┴───────────────────────┘             │
│                            │ Asset Forge                         │
│                     ┌──────┴──────┐                              │
│                     │ 资产生成管线 │                              │
│                     │ AI生图/音效  │                              │
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
│          └───┬───────────┬────────────┬──────────┘              │
│              │           │            │                          │
│  ┌───────────┴──┐ ┌─────┴──────┐ ┌───┴──────────────┐         │
│  │forge-runtime-│ │forge-runtime│ │ forge-runtime-   │         │
│  │    light     │ │   -cocos   │ │   godot          │         │
│  │ Canvas/WebGL │ │ 编译嵌入    │ │ WebSocket云渲染  │         │
│  │ 2D+简单3D    │ │ 微信/抖音   │ │ 3D/物理/光线追踪 │         │
│  │ 浏览器直接跑 │ │ 小游戏     │ │ Phase 2          │         │
│  └──────────────┘ └────────────┘ └──────────────────┘         │
│                                       ┌──────────────────┐      │
│                                       │forge-runtime-    │      │
│                                       │  unreal (预留)   │      │
│                                       │ Pixel Streaming  │      │
│                                       │ Phase 3          │      │
│                                       └──────────────────┘      │
└──────────────────────────────────────────────────────────────────┘
                               │
┌──────────────────────────────┼──────────────────────────────────┐
│                    构建层 (Build Layer)                           │
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌────────────────────────┐│
│  │ Build Pipeline│  │ Package Mgr  │  │  Platform Exporters   ││
│  │ 一键构建     │  │ 扩展/资产包  │  │  Web/WeChat/Bytedance ││
│  └──────────────┘  └──────────────┘  │  Win/Mac/Linux/Mobile ││
│                                        └────────────────────────┘│
└──────────────────────────────────────────────────────────────────┘
```

### 设计哲学

| 优先级 | 原则 | 实现 |
|--------|------|------|
| 🥇 | **无限扩展性** | Extension Registry四柱模型，新功能=注册扩展，架构不改 |
| 🥈 | **AI-First** | 所有操作可通过REST API完成，Web Studio不含独占功能 |
| 🥉 | **引擎无关** | Runtime Abstraction统一接口，新引擎注册即用 |
| 4 | **从小到大** | Phase 1覆盖2D+小游戏，Phase 2/3渐进扩展到3A |
| 5 | **安全执行** | 逻辑引擎不支持图灵完备，循环/递归有上限，防服务器卡死 |

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
| 认证鉴权 | API Key + JWT，Agent用Key，人类用JWT |

**关键设计**：
- Forge Core本身不渲染、不执行逻辑、不生成资产——它只**编排**
- 所有操作通过Forge Bus发布事件，模块间松耦合
- AI Agent只和Forge Core交互，不需要了解下游实现

### 2.2 Script Forge — 游戏逻辑引擎

**一句话**：YAML描述的游戏逻辑→可运行代码

**逻辑描述采用ECS模式**（Entity-Component-Script），这是游戏行业标准：

```yaml
scene: level_1
entities:
  - id: player
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

  - id: coin_1
    components:
      - type: Transform
        position: [300, 150]
      - type: SpriteRenderer
        asset: "coin-spin"
      - type: Collider
        shape: circle
        radius: 12
        is_trigger: true
      - type: Tag
        name: "coin"
```

**编译管线**：
```
YAML → Parser → AST → 类型检查+约束验证 → Runtime Bytecode → 执行
                                                    ↓
                                          forge-runtime-light (Phase 1)
                                          Cocos TypeScript (Phase 1)
                                          GDScript (Phase 2)
```

**安全约束**（服务器端执行必须）：
- 不支持图灵完备——循环有上限（max 1000次/帧）
- 递归深度上限10层
- 单帧执行时间上限16ms（超过自动挂起）
- 禁止文件系统/网络访问（沙箱执行）

### 2.3 Asset Forge — 资产生成管线

**一句话**：AI生成资产→标准化入库→场景引用

```
AI生图请求 → image_generate → 标准化处理 → 资产库
AI生音效请求 → 音频生成 → 格式标准化 → 资产库 → 场景引用
```

**资产入库标准**：

| 类型 | 格式 | 元数据 |
|------|------|--------|
| 2D精灵 | PNG (atlas) | size, pivot, frames, fps |
| TileMap | JSON | grid_size, tiles[], layers[] |
| 3D模型 | glTF 2.0 | bbox, materials, animations[] |
| 音效 | OGG | duration, loop, volume |
| 音乐 | OGG | duration, bpm, loop |
| 字体 | TTF/OTF | family, weight |

### 2.4 Extension Registry — 扩展注册中心

**一句话**：新功能=注册扩展，架构本身永远不需要改

复用体系四柱模型（与OpenDAW/OpenLink同构），游戏化映射：

| 柱 | 功能 | OpenForge注册内容 | 示例 |
|----|------|-------------------|------|
| Action | 注册新动作 | 游戏逻辑动作 | move, collide, play_sound, change_scene, spawn_entity |
| Condition | 注册新条件 | 条件判断 | key_pressed, score_gt, time_elapsed, in_area, entity_collision |
| Hook | 注册新拦截器 | 生命周期钩子 | on_start, on_update, on_collide, on_destroy, build_start |
| Component | 注册新组件 | 渲染/物理/UI组件 | Sprite, Audio, Physics, UI, Particle, Camera |

**Rust接口定义**：

```rust
// Action: 游戏动作
#[async_trait]
pub trait ActionHandler: Send + Sync {
    fn name(&self) -> &str;
    async fn execute(&self, ctx: &mut GameContext, params: &Value) -> Result<()>;
}

// Condition: 条件判断
#[async_trait]
pub trait ConditionHandler: Send + Sync {
    fn name(&self) -> &str;
    async fn evaluate(&self, ctx: &GameContext, params: &Value) -> Result<bool>;
}

// Hook: 生命周期钩子
#[async_trait]
pub trait HookHandler: Send + Sync {
    fn name(&self) -> &str;
    fn phase(&self) -> HookPhase;   // OnInit, OnUpdate, OnRender, OnEvent, OnShutdown
    fn priority(&self) -> i32;
    async fn run(&self, ctx: &mut GameContext) -> Result<()>;
}

// Component: 渲染/游戏组件
#[async_trait]
pub trait ComponentHandler: Send + Sync {
    fn name(&self) -> &str;
    fn supported_runtimes(&self) -> Vec<RuntimeType>;
    async fn init(&self, ctx: &mut GameContext, config: &Value) -> Result<()>;
    async fn update(&self, ctx: &mut GameContext, delta: f64) -> Result<()>;
}
```

**YAML配置注册**：

```yaml
# openforge-extensions.yaml
extensions:
  runtimes:
    - id: light
      type: builtin
      capabilities: [2d, canvas, webgl, particles]

    - id: cocos
      type: external
      capabilities: [2d, wechat, bytedance]
      config:
        cocos_creator_path: /usr/bin/cocos

    - id: godot
      type: external
      capabilities: [3d, physics, ray-tracing]
      config:
        godot_path: /usr/bin/godot
        render_mode: headless
        stream_fps: 30

  components:
    - id: particle-emitter
      type: builtin
      supported_runtimes: [light, godot]

  actions:
    - id: spawn-entity
      handler: forge-runtime-light::actions::SpawnEntity

  hooks:
    - event: build_start
      handler: extensions.procedural-gen.on_build_start
      priority: 10
```

**共享crate规划**：
- `forge-registry` 未来提取为 `open-registry`，与OpenDAW/OpenLink共享
- `forge-core/bus.rs` 未来提取为 `open-bus`，与OpenDAW共享

### 2.5 Runtime Abstraction — 运行时统一接口

**一句话**：所有引擎通过同一套接口接入，新引擎只需实现Trait

```rust
pub trait GameRuntime: Send + Sync {
    fn name(&self) -> &str;
    fn runtime_type(&self) -> RuntimeType;
    fn capabilities(&self) -> RuntimeCapabilities;

    fn init(&mut self, config: RuntimeConfig) -> Result<()>;
    fn update(&mut self, delta: f64) -> Result<()>;
    fn render(&self) -> Result<()>;
    fn handle_event(&mut self, event: GameEvent) -> Result<()>;
    fn shutdown(&mut self) -> Result<()>;
}

struct RuntimeCapabilities {
    dim_2d: bool,
    dim_3d: bool,
    physics: bool,
    particles: bool,
    max_sprites: usize,
    target_fps: usize,
    platforms: Vec<String>,  // ["web", "wechat", "bytedance", "desktop"]
}

enum RuntimeType {
    Light,      // 自研Canvas/WebGL
    Cocos,      // Cocos Creator桥接
    Godot,      // Godot云渲染
    Unreal,     // Unreal云渲染（预留）
    Custom(String),
}
```

**引擎兼容性矩阵**：

| 引擎 | Phase | 模式 | 导出平台 | 状态 |
|------|-------|------|----------|------|
| forge-runtime-light | 1 | 直接实现Trait | Web (HTML5) | ✅ 开发中 |
| forge-runtime-cocos | 1 | 编译嵌入Cocos | 微信/抖音小游戏 | ✅ 开发中 |
| forge-runtime-godot | 2 | WebSocket云渲染 | Web | 📋 待开发 |
| forge-runtime-unreal | 3 | Pixel Streaming | 全平台 | 📋 预留 |

### 2.6 forge-runtime-light — 轻量Web运行时

**一句话**：2D游戏在浏览器直接跑，不依赖云渲染

**Phase 1渲染能力清单**：

| 能力 | 实现 | 性能目标 |
|------|------|---------|
| 精灵渲染 | Canvas 2D SpriteBatch | 1000+精灵@60fps |
| TileMap | 分块渲染+视口裁剪 | 100x100地图 |
| 粒子系统 | Canvas 2D | 500粒子@60fps |
| 基础动画 | Sprite帧动画 | 30fps逐帧 |
| 基础物理 | AABB碰撞检测 | 100+实体 |
| 音频 | Web Audio API | 8通道混音 |
| 简单UI | HTML Overlay | — |

**TypeScript端**：
```typescript
class LightRuntime implements GameRuntime {
  canvas: HTMLCanvasElement;
  ctx: CanvasRenderingContext2D | WebGL2RenderingContext;

  spriteBatch: SpriteBatch;
  particleSystem: Particles;
  tilemapRenderer: Tilemap;

  maxSprites: 1000;
  targetFPS: 60;
}
```

### 2.7 forge-runtime-cocos — Cocos小游戏桥接

**一句话**：AI生成逻辑→编译为Cocos TS→导出微信/抖音小游戏

```
Forge YAML → Script Forge编译 → Cocos TypeScript脚本 → 嵌入Cocos项目 → 导出小游戏
```

- AI生成YAML逻辑
- Script Forge编译为Cocos兼容的TypeScript
- 用户用Cocos Creator打开项目打包（或自动化打包）

### 2.8 Web Studio — Web协作前端

**一句话**：任何设备的游戏开发入口

| 区域 | 功能 | 桌面布局 | 手机布局 |
|------|------|---------|---------|
| 预览区 | 实时游戏画面 | 右侧主区域 | 全屏 |
| 场景树 | 场景节点层级 | 左侧面板 | 抽屉 |
| 编辑区 | YAML/属性编辑 | 底部面板 | 全屏编辑 |
| 控制台 | 日志/错误/输出 | 底部折叠 | 底部折叠 |
| 工具栏 | 构建/发布/设置 | 顶部 | 底部 |

**实时通信**：
- 场景修改 → WebSocket → Forge Core → 事件广播 → 所有客户端同步
- 游戏预览 → forge-runtime-light本地渲染 / Cloud Render流推送

---

## 三、Rust Workspace Crate拆分

```
open-forge/
├── Cargo.toml                    # workspace根
├── crates/
│   ├── forge-core/               # AI编排核心：项目/场景/资产/构建的管理与调度
│   │   └── src/
│   │       ├── project.rs        # 项目CRUD+版本+快照
│   │       ├── scene.rs          # 场景图管理
│   │       ├── asset.rs          # 资产引用管理（不含生成）
│   │       ├── build.rs          # 构建调度
│   │       └── bus.rs            # Forge Bus消息总线
│   │
│   ├── forge-api/                # RESTful + WebSocket接口层
│   │   └── src/
│   │       ├── rest.rs           # Axum REST API
│   │       ├── ws.rs             # WebSocket实时事件
│   │       ├── auth.rs           # API Key + JWT认证
│   │       └── dto.rs            # 请求/响应数据结构
│   │
│   ├── script-forge/             # 游戏逻辑引擎：YAML→AST→Runtime
│   │   └── src/
│   │       ├── parser.rs         # YAML→AST
│   │       ├── checker.rs        # 类型检查+约束验证（循环上限/递归深度/超时）
│   │       ├── compiler.rs       # AST→Runtime Bytecode
│   │       ├── vm.rs             # 字节码虚拟机（沙箱执行）
│   │       └── codegen_cocos.rs  # Cocos TypeScript代码生成（Phase 1）
│   │
│   ├── asset-forge/              # 资产生成管线
│   │   └── src/
│   │       ├── pipeline.rs       # 生成请求→标准化→入库
│   │       ├── store.rs          # 资产库管理
│   │       └── processor.rs      # 格式标准化/裁剪/打包
│   │
│   ├── forge-registry/           # Extension Registry（未来提取为open-registry）
│   │   └── src/
│   │       ├── registry.rs       # 注册中心核心
│   │       ├── action.rs         # Action API
│   │       ├── condition.rs      # Condition API
│   │       ├── hook.rs           # Hook API
│   │       └── component.rs      # Component API
│   │
│   ├── forge-runtime/            # Runtime Abstraction定义+共享类型
│   │   └── src/
│   │       ├── trait.rs          # GameRuntime trait定义
│   │       ├── scene_graph.rs    # 场景图数据结构
│   │       ├── context.rs        # GameContext
│   │       └── event.rs          # GameEvent枚举
│   │
│   ├── forge-runtime-light/      # Light Runtime（Canvas/WebGL 2D）
│   │   └── src/
│   │       ├── runtime.rs        # 实现GameRuntime trait
│   │       └── serializer.rs     # SceneGraph→JSON→前端渲染指令
│   │
│   ├── forge-runtime-cocos/      # Cocos小游戏桥接
│   │   └── src/
│   │       ├── bridge.rs         # Cocos项目生成
│   │       └── codegen.rs        # TS代码生成协调
│   │
│   ├── forge-runtime-godot/      # Godot云渲染桥接（Phase 2）
│   │   └── src/
│   │       ├── bridge.rs         # WebSocket→Godot Headless
│   │       └── stream.rs         # 画面流式传输
│   │
│   └── forge-build/              # 构建发布管线
│       └── src/
│           ├── pipeline.rs       # 构建流程编排
│           ├── web.rs            # Web包导出
│           ├── wechat.rs         # 微信小游戏导出
│           ├── bytedance.rs      # 抖音小游戏导出
│           └── platform.rs       # 桌面/移动端导出（Phase 2+）
│
├── web-studio/                   # TypeScript前端
│   ├── src/
│   │   ├── app/                  # 主应用框架
│   │   ├── preview/              # 游戏预览区
│   │   ├── editor/               # YAML/属性编辑器
│   │   ├── scene-tree/           # 场景树面板
│   │   └── runtime/              # forge-runtime-light (Canvas/WebGL)
│   ├── package.json
│   └── tsconfig.json
│
├── docs/
│   ├── PRD.md
│   ├── knowledge/
│   └── adr/
│
└── scripts/
    └── dev.sh                    # 开发启动脚本
```

**crate依赖关系**：
```
forge-api → forge-core → forge-registry
                    → script-forge
                    → asset-forge
                    → forge-runtime (abstraction)
                         → forge-runtime-light (Phase 1)
                         → forge-runtime-cocos (Phase 1)
                         → forge-runtime-godot (Phase 2)
                    → forge-build
```

---

## 四、API设计

### 4.1 认证

| 方式 | 适用场景 | 获取方式 |
|------|---------|---------|
| API Key | AI Agent | 创建项目时生成 |
| JWT | Web Studio人类用户 | 登录后获取 |

### 4.2 RESTful API

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

### 4.3 WebSocket事件

**Path**: `/ws/v1/events?project_id=:id&token=:jwt`

```json
// 客户端→服务器
{ "type": "scene.update",  "data": { "node_id": "...", "changes": {} } }
{ "type": "game.input",    "data": { "keys": [], "mouse": {} } }
{ "type": "build.start",   "data": { "project_id": "..." } }

// 服务器→客户端
{ "type": "scene.changed",  "data": { "scene_id": "...", "diff": {} } }
{ "type": "game.frame",     "data": { "frame_data": "..." } }
{ "type": "build.progress", "data": { "percent": 45, "step": "compiling" } }
{ "type": "build.completed","data": { "download_url": "..." } }
{ "type": "build.failed",   "data": { "error": "..." } }
{ "type": "asset.ready",    "data": { "asset_id": "...", "url": "..." } }
{ "type": "runtime.error",  "data": { "code": "...", "message": "..." } }
```

### 4.4 错误处理

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid project name",
    "details": { "field": "name", "reason": "must be 1-100 characters" }
  }
}
```

| 错误码 | HTTP状态 | 描述 |
|--------|----------|------|
| VALIDATION_ERROR | 400 | 参数校验失败 |
| UNAUTHORIZED | 401 | 认证失败 |
| FORBIDDEN | 403 | 权限不足 |
| NOT_FOUND | 404 | 资源不存在 |
| BUILD_FAILED | 500 | 构建失败 |
| RUNTIME_ERROR | 500 | 运行时错误 |

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
    tag: Option<String>,       // "player", "enemy", "coin" 等
    components: Vec<Component>,
    children: Vec<SceneNode>,
    transform: Transform,
    scripts: Vec<ScriptBinding>,
}

enum Component {
    SpriteRenderer { asset_id: String, flip_h: bool, flip_v: bool },
    Collider { shape: Shape, size: Vec2, is_trigger: bool },
    Rigidbody { mass: f64, gravity_scale: f64, velocity: Vec2 },
    AudioPlayer { asset_id: String, volume: f64, loop_enabled: bool },
    Camera { zoom: f64, follow_target: Option<String> },
    ParticleEmitter { config: ParticleConfig },
    Custom { type_id: String, data: serde_json::Value },
}

struct Transform {
    position: Vec2,
    rotation: f64,  // degrees
    scale: Vec2,
}

struct ScriptBinding {
    event: String,          // "on_collision_enter", "on_update"
    actions: Vec<Action>,   // 条件+动作列表
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
    runtime: String,            // "light" | "cocos" | "godot"
    background_color: String,   // "#000000"
    target_platforms: Vec<String>, // ["web", "wechat"]
    physics: PhysicsSettings,
}
```

---

## 六、技术选型

| 选型 | 选择 | 理由 |
|------|------|------|
| HTTP框架 | Axum 0.7 | 与OpenLink/OpenVault/OpenDAW一致 |
| 异步运行时 | Tokio | Rust标准选择 |
| 序列化 | serde + serde_json + serde_yaml | Rust标准 |
| WebSocket | axum::extract::ws | Axum内置 |
| 前端框架 | React + Vite | 生态成熟，组件丰富 |
| Canvas渲染 | Canvas 2D API (Phase 1) | 够用，Phase 2按需升WebGL |
| 数据存储 | SQLite (rusqlite) | 轻量、零配置、单文件 |
| 认证 | API Key + JWT | Agent用Key，人类用JWT |
| 构建工具 | just (justfile) | 与OpenDAW一致 |
| Rust版本 | 1.95.0 | 与OpenDAW一致 |
| 编译约束 | CARGO_BUILD_JOBS=2, CARGO_TARGET_DIR=/tmp/openforge-target | 内存3.8G限制 |

---

## 七、Phase 1 开发路线（5 Sprint / 9周）

### Sprint 1（2周）：骨架搭建
- [ ] Rust workspace初始化 + crate骨架
- [ ] forge-core：项目CRUD + 场景图数据结构
- [ ] forge-api：Axum骨架 + 健康检查 + 认证中间件
- [ ] forge-runtime：GameRuntime trait + 共享类型
- [ ] Web Studio：React+Vite初始化 + 基本布局

### Sprint 2（2周）：场景可编辑
- [ ] forge-core：场景图CRUD + Forge Bus
- [ ] forge-api：项目+场景REST API完整实现
- [ ] Web Studio：场景树展示 + YAML编辑器 + WebSocket实时同步

### Sprint 3（2周）：能跑游戏
- [ ] script-forge：YAML解析 + AST + 基础VM（移动/碰撞/计分）
- [ ] forge-runtime-light：SceneGraph→JSON序列化 + Canvas精灵渲染
- [ ] Web Studio：游戏预览区（精灵+粒子+TileMap+音频）

### Sprint 4（2周）：AI可创作
- [ ] asset-forge：AI生图→标准化入库
- [ ] script-forge：完整逻辑编译（条件/事件/组件交互）+ 安全约束
- [ ] forge-registry：四柱基础实现（Action/Condition/Hook/Component）
- [ ] forge-runtime-cocos：Cocos TS代码生成 + 项目导出
- [ ] **验收**：AI Agent通过REST API从零创建可玩2D游戏

### Sprint 5（1周）：构建发布+收尾
- [ ] forge-build：Web包一键导出 + 微信小游戏导出
- [ ] 全链路测试：AI描述→场景生成→逻辑绑定→预览→构建→可玩
- [ ] 性能验收：1000+精灵@60fps
- [ ] 文档收尾 + 部署脚本

---

## 八、ADR索引

| ADR | 标题 | 决策 | 日期 |
|-----|------|------|------|
| 001 | 分层架构vs自研引擎 | 分层+桥接现有引擎 | 2026-05-18 |
| 002 | Forge Core API风格 | RESTful + WebSocket | 2026-05-18 |
| 003 | 游戏逻辑描述语言 | YAML | 2026-05-18 |
| 004 | Extension Registry实现 | 四柱模型（Action/Condition/Hook/Component） | 2026-05-18 |
| 005 | Runtime Abstraction | Trait抽象 + 插件化运行时 | 2026-05-18 |
| 006 | 多引擎桥接方案 | 按Phase迭代（Light→Cocos→Godot→Unreal） | 2026-05-18 |
