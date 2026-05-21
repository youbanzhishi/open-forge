# PRD — OpenForge AI-First 游戏创作平台

> 产品经理：产品经理角色
> 创建时间：2026-05-18
> 状态：已立项

## 需求背景

### 为什么做这个

未来的游戏开发将由AI Agent主导——但现有游戏引擎（Unreal/Unity/Godot）都绑定桌面端，离开电脑就无法开发。主人需要的是一个**AI Agent能在服务器上自主写游戏、人类通过任何设备Web端查看/参与开发**的平台。

### 核心洞察

1. **引擎≠平台**：引擎解决渲染和物理，平台解决创作流程。我们的价值在平台，不在引擎
2. **AI-First**：传统引擎是为人设计的编辑器，AI操作是后加的；我们从底层为AI Agent设计创作接口
3. **无限扩展**：小游戏自建渲染，3A云渲染桥接——分层架构让规模不影响设计

### 在蓝图的位置

本项目的"无限扩展"原则与体系蓝图五大支柱完全对齐：
- 新渲染后端=注册扩展，架构不改
- 新AI能力=注册扩展，架构不改
- 新协作模式=注册扩展，架构不改

## 用户故事

**作为 游戏创作者**，我想 **用自然语言告诉AI我要做什么游戏**，以便 **不用写代码就能把想法变成可玩游戏**

**作为 游戏开发者**，我想 **在任何设备上通过Web查看和修改AI正在开发的游戏**，以便 **不在电脑前也不耽误进度**

**作为 AI Agent**，我想 **通过标准化API创建场景/编写逻辑/生成资产**，以便 **不需要GUI就能完成游戏开发全流程**

## 产品命名

**OpenForge** — 开源锻造炉，锻造游戏

Slogan：**AI writes, You guide, Any device plays**

## 功能范围

### 做什么

| 模块 | 功能 | 说明 |
|------|------|------|
| 🔨 Forge Core | AI创作编排层 | AI Agent的核心接口：场景描述→资产生成→逻辑编排→构建发布 |
| 🎨 Asset Forge | 资产生成管线 | AI生成2D/3D资产（图片/模型/音效/音乐），标准化入库 |
| 📜 Script Forge | 游戏逻辑引擎 | YAML/JSON描述游戏逻辑→编译为可运行脚本，AI直接写逻辑 |
| 🖥️ Web Studio | Web协作前端 | 任何设备访问，实时预览+场景编辑+逻辑审查+发布管理 |
| ⚡ Light Runtime | 轻量Web运行时 | 2D/简单3D游戏在浏览器直接运行，无需云渲染 |
| ☁️ Cloud Render Bridge | 云渲染桥接 | 3A级游戏通过Godot headless/Unreal PIC云渲染，Web端流式接收 |
| 🔌 Extension Registry | 扩展注册中心 | 新渲染后端/新AI能力/新协作模式=注册扩展，架构永不需要改 |
| 📦 Build Pipeline | 构建发布管线 | 一键构建多平台包（Web/Windows/macOS/Linux/移动端） |

### 不做什么（Phase 1）

| 不做什么 | 为什么 |
|----------|--------|
| 自研3D渲染引擎 | 工程量无底洞，现有引擎可桥接 |
| Unreal级物理引擎 | 物理引擎是独立子系统，先对接现有方案 |
| 实时多人协作编辑 | Phase 1只做单人+AI，多人协作后置 |
| 可视化节点编辑器 | Phase 1用YAML/JSON，GUI后置 |
| 移动端原生App | Web优先，PWA渐进增强 |

## 架构设计

### 分层架构（核心决策）

```
┌──────────────────────────────────────────────────────┐
│                  Web Studio (前端)                     │
│         场景预览 · 逻辑审查 · 资产管理 · 发布           │
├──────────────────────────────────────────────────────┤
│              Forge Core (AI编排层) ← 护城河             │
│   自然语言→游戏描述→资产调度→逻辑编译→场景组装→构建      │
├──────────┬───────────┬───────────────────────────────┤
│ Script   │ Asset     │ Extension Registry             │
│ Forge    │ Forge     │ 新后端/能力/模式 = 注册扩展     │
│ 逻辑引擎  │ 资产管线  │ 架构本身永远不需要改            │
├──────────┴───────────┴───────────────────────────────┤
│                   Runtime Abstraction                 │
│         统一接口：init/update/render/event/shutdown    │
├─────────────────┬────────────────────────────────────┤
│  Light Runtime  │  Cloud Render Bridge                │
│  Canvas/WebGL   │  Godot Headless / Unreal PIC        │
│  2D+简单3D      │  云渲染流式推送                       │
│  浏览器直接跑    │  Web端只接收流+交互                   │
└─────────────────┴────────────────────────────────────┘
```

### 关键设计决策

| 决策 | 选择 | 理由 |
|------|------|------|
| 游戏逻辑描述语言 | YAML/JSON | AI友好、人类可读、与VCMix经验复用 |
| 2D渲染 | Canvas/WebGL | 轻量、浏览器原生、零依赖 |
| 3D渲染 | 桥接Godot优先 | 开源、headless可服务端运行、GDScript AI可写 |
| 云渲染方案 | Godot→Unreal渐进 | Phase 2先Godot云渲染，Phase 3再桥接Unreal |
| 扩展机制 | Registry模式 | 与OpenDAW Extension Registry理念一致 |
| AI接口风格 | RESTful + WebSocket | RESTful做资源CRUD，WebSocket做实时事件流 |

### Extension Registry 设计（无限扩展核心）

```
注册一个新渲染后端：
{
  "type": "runtime",
  "name": "unreal-pixel-streaming",
  "capabilities": ["3d", "physics", "ray-tracing"],
  "interface": "RuntimeAbstraction",
  "config": { ... }
}

注册一个新AI能力：
{
  "type": "ai-capability",
  "name": "procedural-world-gen",
  "input": "world-description.yaml",
  "output": "scene-graph.json",
  "interface": "ForgeCore"
}

注册一个新协作模式：
{
  "type": "collaboration",
  "name": "multiplayer-edit",
  "protocol": "crdt",
  "interface": "WebStudio"
}
```

**规则：新功能=注册扩展，架构本身永远不需要改**

## 阶段规划

### Phase 1：AI-First 2D创作平台（MVP）

**目标**：AI能在服务器上写2D游戏，人通过Web查看和修改

| 功能 | 验收标准 |
|------|----------|
| Forge Core | AI Agent通过RESTful API创建/修改/删除游戏项目，YAML描述→可运行场景 |
| Script Forge | 支持基础游戏逻辑：角色移动/碰撞/计分/UI/场景切换 |
| Asset Forge | AI生图→自动入资产库→场景引用，支持sprite/精灵图/TileMap |
| Light Runtime | 2D Canvas渲染：精灵/TileMap/粒子/基础动画，60fps流畅 |
| Web Studio | 实时预览+场景树查看+YAML编辑+发布按钮，手机可操作 |
| Build Pipeline | 一键导出Web包，浏览器直接玩 |

**技术栈**：Rust(Forge Core/Script Forge) + TypeScript(Light Runtime/Web Studio) + Canvas API

**交付物**：`https://forge.你的域名` 可访问，AI从零创建一个可玩的2D游戏

### Phase 2：Godot桥接 + 中型3D

**目标**：AI能产出Godot项目，支持3D游戏

| 功能 | 验收标准 |
|------|----------|
| Godot Bridge | AI输出→Godot .tscn/.gd文件→headless渲染→Web预览 |
| 3D场景支持 | Godot云渲染基础3D场景，Web端30fps+流式预览 |
| Asset Forge 3D | AI生成3D模型(glTF)→自动入资产库→Godot场景引用 |
| Script Forge升级 | GDScript/C#输出，AI直接写Godot脚本 |

### Phase 3：3A能力 + 生态

**目标**：云渲染桥接Unreal，支持3A级游戏开发

| 功能 | 验收标准 |
|------|----------|
| Unreal Bridge | Unreal Pixel Streaming，4K@30fps+云渲染 |
| 高级物理/光照 | 通过桥接引擎的高级特性 |
| 多人协作 | CRDT实时协作编辑 |
| 扩展市场 | 第三方扩展注册/分发 |

## 验收标准（可量化）

| 编号 | 验收项 | 标准 | 优先级 |
|------|--------|------|--------|
| AC-001 | AI创建2D游戏 | 从自然语言输入到可玩游戏<5分钟（简单2D） | P0 |
| AC-002 | Web端预览延迟 | 场景修改→Web预览更新<2秒（2D） | P0 |
| AC-003 | 手机端可用 | Web Studio在手机浏览器可完整操作 | P1 |
| AC-004 | 2D渲染性能 | 100+精灵同时渲染≥60fps | P1 |
| AC-005 | 扩展注册 | 新渲染后端注册<10行配置，无需改核心代码 | P0 |
| AC-006 | AI API覆盖 | 游戏开发全流程API覆盖（创建/资产/逻辑/场景/构建/发布） | P0 |
| AC-007 | Godot云渲染 | 3D场景Web预览≥30fps | P2 |
| AC-008 | 构建发布 | 一键Web包导出，浏览器直接玩 | P1 |

## 跨角色影响

| 角色 | 需要做什么 | Phase |
|------|-----------|-------|
| 系统开发者 | Forge Core/Script Forge核心Rust开发 | 1 |
| 前端开发 | Web Studio前端 + Light Runtime | 1 |
| 游戏开发工程师 | Godot Bridge对接 + 游戏逻辑设计规范 | 2 |
| 游戏美术设计师 | Asset Forge资产标准 + AI生图集成 | 1 |
| 产品经理 | 需求细化/验收/蓝图维护 | 全程 |
| 本地运维 | 部署Forge服务+云渲染环境 | 1-3 |
| AI调教师 | AI Agent创作流程优化 | 1-3 |

## 关联项目

| 项目 | 关系 |
|------|------|
| OpenDAW | 架构理念复用（Extension Registry/YAML描述→渲染） |
| 游戏开发项目 | 上游：现有游戏开发经验输入 |
| OpenLink | 下游：Forge服务可能部署在OpenLink之上 |

## 优先级与排期

- **优先级**：P0（主人直接立项）
- **Phase 1 MVP目标**：可玩的AI-First 2D创作平台
- **理由**：AI+游戏是未来赛道，先发优势重要；与现有技术栈（Rust+YAML+Extension Registry）高度复用

## 关键风险

| 风险 | 概率 | 影响 | 缓解 |
|------|------|------|------|
| 云渲染延迟过高 | 中 | Phase 2/3受阻 | Phase 1不依赖云渲染，先验证2D |
| AI生成游戏逻辑质量不稳 | 高 | 用户体验差 | 先限定游戏类型模板，逐步开放 |
| Godot headless不够稳定 | 中 | Phase 2受阻 | 备选：Godot Web导出 |
| 3A云渲染成本过高 | 高 | Phase 3受限 | 按需启动渲染实例，闲置自动关 |
