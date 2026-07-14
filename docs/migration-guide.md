# OpenLark 迁移指南

本文档覆盖跨版本公开入口迁移。**当前 workspace 版本为 0.18.0**；下方按版本分节。

---

# OpenLark 0.18 迁移指南

适用范围：从 `0.17.x` 或更早版本迁移到 `0.18.x`

## 一句话结论

`0.18` 在 WebSocket 会话收缩之外，完成了 **编译能力 catalog 统一**与 **registry metadata-only 诊断收缩**（#423 / #434–#437）：

- 全部业务域 Client 字段与 registry 元数据由 `capability` catalog 单源生成
- `Client::registry()` 只读诊断：listing / lookup / presence / 依赖图
- 删除无法兑现的 typed-instance、虚假 lifecycle 与 `FeatureLoader` 旁路初始化

## 1. registry / FeatureLoader 迁移

### 已删除（严重正确性例外，0.18 直接移除）

| 旧 API | 替代 |
|--------|------|
| `openlark_client::FeatureLoader` | 删除。能力在 `Client::builder()...build()` 时由 catalog 注册 |
| `ServiceStatus` | 删除。registry 不再表达 lifecycle 状态 |
| `ServiceRegistry::register_service` / `unregister_service`（公开） | 删除。构造期内部注册为 `pub(crate)` |
| `ServiceRegistry::get_service_typed` | 删除。无 runtime instance；业务走 `client.<domain>` |
| `ServiceRegistry::update_service_status` | 删除 |
| `ServiceEntry.instance` / 时间戳 | 删除 |
| `ServiceMetadata.status` | 删除 |
| `RegistryError::CircularDependency` / `MissingDependencies` / `InvalidFeatureFlag` | 删除。这些变体只对应已移除的运行时注册、依赖校验和 `FeatureLoader` 路径；删除直接构造与穷举匹配分支 |

### 推荐诊断写法

```rust
use openlark_client::prelude::*;

let client = Client::builder()
    .app_id("app")
    .app_secret("secret")
    .build()?;

// 是否编译了某业务能力（与 Cargo feature 一致）
if client.registry().has_service("docs") {
    // ...
}

// 稳定顺序：priority 升序，同 priority 按 name
for entry in client.registry().list_services() {
    println!(
        "{} prio={} deps={:?}",
        entry.metadata.name, entry.metadata.priority, entry.metadata.dependencies
    );
}

// 单条元数据
let entry = client.registry().get_service("auth")?;
assert!(entry.metadata.description.is_some());
```

### 业务调用（不变）

```rust
// 继续使用 meta 链，不经 registry 取实例
#[cfg(feature = "docs")]
let _docs = &client.docs;
```

## 2. WebSocket（0.18）

见 CHANGELOG Breaking 表与 `docs/PUBLIC_API_STABILITY_POLICY.md`；`ws_client` 仅保留
`LarkWsClient` / 事件 handler 相关公开类型。

## 3. 升级自检

- [ ] 代码中无 `FeatureLoader` / `ServiceStatus` / `get_service_typed`
- [ ] 诊断仅用 `has_service` / `list_services` / `get_service` / `get_dependency_graph`
- [ ] 业务路径使用 `client.<domain>`，不期望 registry 返回可调用实例
- [ ] 阅读 CHANGELOG Unreleased / 0.18 Breaking 段

---

# OpenLark 0.15 迁移指南

适用范围：从 `0.14.x` 或更早版本迁移到 `0.15.x`

## 一句话结论

`0.15` 的核心变化不是“多了多少 API”，而是将 SDK 的公开入口、feature 表达和文档路径统一到更稳定的模型：

- 普通用户优先依赖根 crate `openlark`
- `openlark-client` 保留，但不再是默认推荐入口
- feature 命名应表达业务能力，而不是内部实现分层

## 迁移优先级

建议按以下顺序迁移：

1. 先切依赖入口
2. 再切 feature 写法
3. 最后清理历史入口和兼容调用

## 1. 依赖入口迁移

### 推荐写法

```toml
[dependencies]
openlark = "0.15"
```

或按需启用业务 feature：

```toml
[dependencies]
openlark = { version = "0.15", default-features = false, features = ["auth", "communication"] }
```

### 何时继续使用 `openlark-client`

只有在以下场景才建议继续直接依赖 `openlark-client`：

- 你明确要复用高级客户端实现层
- 你需要直接操作客户端层能力，而不是以业务 feature 为中心接入
- 你正在维护内部封装，对下游屏蔽根 crate 的组合 feature

普通业务应用与 SDK 使用者，优先使用 `openlark`。

## 2. feature 模型迁移

### 迁移原则

从 `0.15` 开始，feature 应表达“我要什么能力”，而不是“我要哪个内部层”。

优先使用以下三类 feature：

- 业务 feature：`auth`、`communication`、`docs`、`security`、`hr`、`workflow`、`meeting`、`ai`、`cardkit`、`webhook`
- 技术 feature：`websocket`、`otel`
- 组合 feature：`essential`、`enterprise`、`full`

### 推荐组合

```toml
openlark = "0.15"
openlark = { version = "0.15", features = ["essential"] }
openlark = { version = "0.15", features = ["enterprise"] }
openlark = { version = "0.15", features = ["full"] }
```

## 3. 公开入口迁移

### 推荐入口

```rust
use open_lark::prelude::*;

let client = Client::builder()
    .app_id("your_app_id")
    .app_secret("your_app_secret")
    .build()?;
```

### 推荐访问方式

```rust
client.docs.list_folder_children_all("folder_token", None).await?;
client.docs.find_sheet_by_title("spreadsheet_token", "汇总表").await?;
client.communication;
```

## 4. legacy entrypoint 说明

`legacy_client` 不再作为 `0.15` 的公开迁移目标。

如果你的历史代码依赖旧入口，请按下面的方向调整：

- 旧的“先决定依赖 `openlark` 还是 `openlark-client`”心智，迁移为“默认先用 `openlark`”
- 旧的实现层 feature 心智，迁移为业务能力 feature 心智
- 旧的分散示例入口，迁移为根 crate 与根 examples 的统一入口

已加上 deprecated 标记的 legacy entrypoint 分类与替代路径见：

- `docs/legacy-entrypoint-migration-notes.md`

## 5. 哪些变化可能影响你

以下变化最可能影响升级：

- 公开文档示例从 `openlark-client` 迁移到 `openlark`
- 部分历史入口不再作为默认推荐路径
- feature 组合的建议写法发生变化

## 6. 升级自检

升级到 `0.15` 后，建议至少确认以下事项：

- 依赖入口是否已经统一到 `openlark`
- README 或内部接入文档是否还保留历史依赖示例
- feature 是否表达业务能力而不是内部实现层
- 公开示例是否仍能编译通过

## 7. 常见问题

### `openlark-client` 被移除了吗？

没有。它仍然存在，但定位从“普通用户默认入口”调整为“高级入口/底层实现层”。

### `0.15` 会立即删除所有历史兼容层吗？

不会。`0.15` 的目标是先统一公开入口和迁移路径，再逐步收敛历史兼容层。

### 我应该优先跟随哪个文档？

优先级建议如下：

1. 根 `README.md`
2. 本文档
3. 对应业务 crate 的 README

## 8. 后续约束

从 `0.15` 开始，任何公开入口或公开 feature 的变化，都应同时提供：

- changelog 说明
- release note 说明
- 必要时的迁移文档更新

兼容性说明模板见：

- `docs/api-compatibility-note-template.md`

重构型迁移文档模板见：

- `docs/api-refactor-migration-template.md`
