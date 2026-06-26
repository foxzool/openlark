## Context

OpenLark v0.16.1 是飞书开放平台的 Rust SDK，采用三层 workspace 架构（core → client → domain），包含 18 个 crates、~335K 行代码、1,560+ API。

架构审核发现以下结构性问题需要解决：

1. **ServiceRegistry 是伪注册**：`bootstrap.rs` 注册的 `ServiceMetadata` 仅含描述性字段，不持有服务实例。所有服务在 `Client::with_config()` 中通过 14 个 `#[cfg(feature)]` 块硬编码初始化。
2. **双重 Config**：`openlark_client::Config`（app_id, base_url 等扁平字段）与 `openlark_core::config::Config`（`Arc<ConfigInner>` 含 TokenProvider）共存，用户易混淆。
3. **根 crate re-export 臃肿**：约 300 行 deprecated 别名代码，`prelude` 用 `#[allow(deprecated)]` 导出旧类型。
4. **业务 crate 命名不一致**：`AuthClient`（自定义 struct）vs `DocsClient`（pub use）vs `WorkflowClient`（type alias）vs `SecurityClient`（`Arc<SecurityServices>`）。

约束条件：
- 不得破坏 `client.xxx` 链式字段访问的公开 API
- 所有业务 crate 需保持独立发布能力
- MSRV = 1.88

## Goals / Non-Goals

**Goals:**
- 通过宏收敛 ServiceRegistry / Client 的 feature-gated 样板，保留轻量元信息 registry 用于诊断
- 统一 Config 体系，减少一层间接
- 清理 v0.15 之前遗留的 deprecated re-export
- 统一业务 crate 的 Client 类型导出规范
- 修复 `openlark-core` 的 `testing` feature 默认启用问题
- 记录 HTTP 中间件管道和 `ApiRequest<R>` 类型约束为后续独立设计项
- 添加 `docs.rs` 构建配置

**Non-Goals:**
- 不改变 `client.xxx` 链式字段的公开使用方式
- 不新增外部依赖超过 2 个（`tower` 是候选）
- 不重写业务 crate 的内部实现
- 不改变版本号策略
- 不在本 change 中改造 Transport middleware 或 `ApiRequest<R>` 类型模型

## Decisions

### Decision 1: ServiceRegistry → 宏驱动的 feature-gated 初始化

**选择**: 使用声明宏（`macro_rules!`）自动生成 `Client` 的 feature-gated 字段和初始化代码，同时将 Registry 简化为轻量级元信息查询工具。

**理由**:
- Rust 的 trait object 服务容器在 18 个 crate 场景下引入不必要的动态分派和 `Any` downcast 复杂度
- 宏方案保持零成本抽象，编译时生成所有代码
- Registry 保留为纯元信息注册（用于 `registry.list_services()` 等诊断功能）

**替代方案**:
- **`Box<dyn Any>` 容器**: 需要大量 downcast，运行时开销
- **完全移除 Registry**: 失去诊断能力
- **Plugin 架构**: 过度设计，不符合 Rust 惯例

### Decision 2: Config 统一为 CoreConfig + ClientBuilder

**选择**: 将 `openlark_client::Config` 重构为 `openlark_core::config::Config` 的构建器包装，对外仅暴露 `Client::builder()` 和 `CoreConfig`。

**理由**:
- 用户 99% 通过 `Client::builder()` 创建客户端，无需直接接触两种 Config
- `openlark_core::config::Config` 的 `Arc<ConfigInner>` 已是零拷贝，无需再包装一层

**迁移路径**: `openlark_client::Config` 标记 `#[deprecated]`，`Client::builder()` 内部直接构建 `CoreConfig`。

### Decision 3: 根 crate deprecated 清理策略

**选择**: 在 v0.17 中移除所有 v0.15 标记的 deprecated 别名，保留 v0.16 新增的。

**理由**:
- 已标记 `deprecated(since = "0.15.0")` 超过一个次版本周期
- `prelude` 中的 `#[allow(deprecated)]` 让新用户也会看到旧 API
- 清理后 `src/lib.rs` 从 ~540 行减至 ~200 行

### Decision 4: HTTP 中间层与 `ApiRequest<R>` 类型复核 — 延迟到后续 change

**选择**: 在 design 中记录 tower-style 中间件和 `ApiRequest<R>` 类型约束复核，但作为独立 change 延迟实施。

**理由**:
- Transport 重构影响所有业务 crate 的请求路径
- 需要充分的性能基准测试
- 当前 Transport 的 tracing + validation 已覆盖核心场景
- `ApiRequest<R>` 的 request/response 关联关系属于 core API 类型模型，需要独立评估迁移风险

### Decision 5: 业务 crate Client 类型命名规范

**选择**: 制定统一规范 —— 所有业务 crate 导出 `XxxClient` 类型；允许 `struct` 或兼容性 `type alias`，但不允许把 `Arc<XxxServices>` 这类包装直接暴露为 public API。

**理由**:
- 一致性降低学习成本
- 已存在的 `type alias` 可作为兼容过渡，v1.0 前如需改为 wrapper struct 再单独评估 breaking change
- Arc 包装应由内部处理，不暴露给消费者

## Risks / Trade-offs

- **[破坏性变更]** 移除 deprecated 别名会影响未更新的用户 → 在 CHANGELOG 中提供详细迁移指南，major 版本前完成
- **[宏复杂度]** 宏驱动的初始化代码可能难以调试 → 保持宏简单，生成可读代码，添加文档注释
- **[Config 合并]** 部分内部代码依赖 `openlark_client::Config` 的扁平字段访问 → 逐步迁移，先 deprecated 再移除
- **[tower 引入]** 新增 `tower` 依赖增加编译时间 → 延迟到 v0.18，先评估必要性

## Migration Plan

1. **v0.17.0**: Config deprecated、根 crate清理、命名统一、testing feature 修复、`async_fn_in_trait` 移除、`docs.rs` metadata
2. **v0.17.x**: ServiceRegistry 元信息注册宏化，Client 字段/初始化宏化，减少新增业务域时的手写注册和初始化样板
3. **v0.18.0+**: tower 中间件层（可选）与 `ApiRequest<R>` 类型约束复核
4. **v1.0.0**: 移除所有 deprecated API，锁定公开 API

回滚策略：每个变更独立提交，可通过 git revert 单独回滚。
