# 项目规范 / Copilot instructions

## 目的

为未来的 Copilot 会话提供仓库特定的操作要点：如何构建、运行、测试、和导航代码以便快速做出正确改动。

## 构建、测试与 lint 命令

- 构建（开发模式）
  - cargo build
- 运行（本地调试 / dev）
  - cargo run
- 全量测试
  - cargo test
- 运行单个测试（通过名称过滤）
  - cargo test <test_name_or_pattern>
  - 例如：cargo test parse_input
- 代码格式化 / 检查
  - cargo fmt # 格式化
  - cargo fmt -- --check # CI 中检查
- 静态检查（lint）
  - cargo clippy --all-targets --all-features -- -D warnings

备注：这是一个 Rust CLI 程序（见 Cargo.toml），使用 tokio、ratatui、crossterm 等库；优先使用 cargo 系列命令进行所有开发/验证工作。

## 高级架构概览（Big picture）

- 目标：一个“幻想世界”风格的交互式 shell（CLI）并内置虚拟文件系统、bit/容量机制、以及用于战斗/AI 的玩法层。
- 二叉结构：UI + 运行时
  - UI / TUI：使用 crossterm + ratatui，在 main.rs 中初始化并驱动事件循环与渲染。
  - Shell / 命令层（src/cmd.rs）：负责解析用户输入、参数和命令分发（cd、ls、cat 等）。
  - 虚拟文件系统（src/vfs.rs & src/vfs/\*）：内存文件系统实现、路径解析、权限与 bit（大小）追踪。
- 异步运行时：tokio 用于并发 IO / 事件处理（注意 async 偕同库版本）。
- 项目分阶段规划在 TODO.md：从基本 shell 到 VFS、bit 系统、战斗与 AI 脚本逐步实现；很多设计决策（例如 pipe、重定向）在 TODO 中列为 later-v2。

主要文件职责（快速映射）

- src/main.rs — 启动、终端初始化、主事件循环、依赖注入点
- src/cmd.rs — 命令解析与调度（核心命令集入口）
- src/vfs.rs（或 src/vfs/\*）— VFS 数据结构、路径与权限逻辑
- Cargo.toml — 依赖与编辑器/版本信息（edition = "2024"）

## 关键约定与代码库习惯

- 分阶段设计：在实现或大幅修改功能前，先参阅 TODO.md 中对应阶段与 DoD（Definition of Done）。Copilot 操作时应避免跨阶段混合修改。
- 错误与可观察性：使用 color-eyre 统一错误处理；保留错误上下文以便调试。
- UI 与逻辑分离：尽量在非 UI 模块（vfs、命令处理）中保持无副作用，以便单元测试。
- 依赖版本固定：Cargo.toml 中依赖多数已指定具体版本，新增依赖前请审慎评估并同步 Cargo.lock。
- 测试策略：首选单元/集成测试（cargo test）；测试名可被过滤以只执行单个测试（如上所述）。
- 异步注意事项：当在同步上下文调用 async 函数时，遵循项目现有 tokio 使用模式（在 main.rs 中观察 runtime 启动方式）。

## 搜索与导航优先级（如何让 Copilot/子会话高效工作）

- 查找实现与接口：优先打开 src/\*.rs（main.rs、cmd.rs、vfs.rs）并阅读 TODO.md 以理解设计意图。
- 快速查找符号：grep/glob 在本仓库已足够；对跨模块数据流的深入追踪，先阅读调用点而不是盲目修改接口。

## 现有文档与 AI 助手配置

- 已存在文件：README.md、TODO.md、cloudbaserc.json、manifest.json。TODO.md 含主要路线图，修改实现前请参考它。
- Copilot 指令文件已存在（本文件为增强版）。仓库中未发现 CLAUDE.md、AGENTS.md、.cursorrules、CONVENTIONS.md 或 .windsurfrules 等其它 AI-agent 配置文件（如存在，请在此处补充并合并要点）。

## 与 Copilot 会话的建议流程

- 先阅读 TODO.md 对应阶段与 main.rs/cmd.rs/vfs.rs 的实现片段。
- 如果要改动 API 或命令签名，先在对话中说明设计改变并征询批准（避免拆分改动跨多个文件未引导的重构）。
- 在实现后运行：cargo fmt && cargo clippy && cargo test
