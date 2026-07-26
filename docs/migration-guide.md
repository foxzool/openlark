# OpenLark 迁移指南

本文档覆盖跨版本公开入口迁移。**当前 workspace 版本为 0.19.0**。下方按版本分节；**从 0.18 升级请先读 0.19 专节**。

完整 breaking 表与逐 API 迁移代码见根目录 [`CHANGELOG.md`](../CHANGELOG.md) 的
`## [0.19.0]` 节（GitHub Release 正文亦从此提取）。

---

# OpenLark 0.19 迁移指南

适用范围：从 `0.18.x` 迁移到 `0.19.x`

## 一句话结论

`0.19` 是 breaking 窗口：删除 registry 诊断半边、接通飞书错误码解码（`ApiError.raw_code`）、
收并 `WsClientError`、对齐 attendance 字段 schema，并清理一批零消费者死 trait/helper。
业务调用仍走 `client.<domain>`；多数只做 leaf builder 的代码主要受 attendance 字段与
错误处理路径影响。

## 1. Registry 删除（#471）

0.18 仍保留 `Client::registry()` 只读诊断；**0.19 整段删除**：

| 已删除 | 替代 |
|--------|------|
| `Client::registry()` / `ServiceRegistry` / `ServiceEntry` / `ServiceMetadata` / `RegistryError` | 删除。能力是否编译 → **Cargo feature** + `openlark-capability-unique` trybuild（编译期） |
| `LarkClient` / `ServiceTrait` / `ServiceLifecycle` / `LazyService` / `ClientErrorHandling` | 删除。业务继续 `client.<domain>` |
| `error::registry_error()` / `From<RegistryError>` | 删除 |

```rust
// before (0.18)
if client.registry().has_service("docs") { /* ... */ }

// after (0.19)
// 删除 registry 调用。用 Cargo feature 门控编译期路径：
#[cfg(feature = "docs")]
let _docs = &client.docs;
```

仅走 `client.<domain>` 的代码零影响。

## 2. `ApiError` / `raw_code` / 构造器（#544–#546，ADR-0004）

生产路径曾把飞书 9 位业务码 `as u16` 截断，导致 `ErrorCode` 恒为 `Unknown`。0.19 接通
`ErrorCode::from_code(raw_code)` 单路径。

| 变更 | 说明 |
|------|------|
| `ApiError.status: u16` → `raw_code: i32` | 字段语义为原始错误码（飞书 body `code` 或 HTTP 非 2xx 合成 status） |
| `api_error` / `CoreError::api*` / `api_err!` | 参数 `u16` → `i32`（勿再 `as u16`） |
| `ErrorBuilder::status(u16)` → `raw_code(i32)` | Builder 同步 |
| 删除 `ErrorCode::from_feishu_code` | 改 `ErrorCode::from_code`（未知 → `Unknown`，非 `None`） |
| 删除 `openlark_client::error::from_feishu_response` | 改 core/client `api_error` 或 `CoreError::Api` |
| retry 谓词 | `is_retryable` 改匹配 `ErrorCode` variant；延迟公式不变 |

```rust
// 读字段
if let CoreError::Api(api) = &err {
    let raw = api.raw_code; // i32 原样，如 99991663
    let kind = api.code;    // ErrorCode::TenantAccessTokenInvalid
    let _ = (raw, kind);
}

// 构造：勿 as u16
let _ = api_error(99991663, "/open-apis/...", "token invalid", None);
let _ = ErrorBuilder::new(BuilderKind::Api).raw_code(404).message("not found").build();
```

完整 before/after 表见 CHANGELOG `## [0.19.0]` 中 ADR-0004 条目。

## 3. `WsClientError`（ADR-0003）

端点发现 HTTP 收口到 core `Transport`；公开错误变体收敛：

| 变更 | 说明 |
|------|------|
| 删除 `ServerError{code,message}` / `ClientError{code,message}` | 端点发现独占、零外部消费者 |
| `RequestError` 负载 | `reqwest::Error` → `CoreError`（透传 `request_id`） |
| 保留 | `UnexpectedResponse` 与全部 WS 会话 variant |

```rust
// before
// match err {
//     WsClientError::ServerError { .. } | WsClientError::ClientError { .. } => {}
//     WsClientError::RequestError(reqwest_err) => {}
//     ...
// }

// after
match err {
    WsClientError::RequestError(core_err) => {
        let _ = core_err.request_id(); // 端点业务错误现可带 request_id
    }
    WsClientError::UnexpectedResponse(_) => {}
    // ConnectionClosed / WsError / HandlerPanicked / ... 不变
    _ => {}
}
```

## 4. Attendance 字段 / Builder 摘要（#526–#533）

一批 attendance API 与飞书官网 schema 对齐；本地 wiremock 曾抄自错误实现而全绿。
**只改字段与 Builder 签名，不重做业务语义。** 按族快速对照：

| API 族 | 要点 |
|--------|------|
| `user_daily_shift`（batch_create / batch_create_temp / query） | `shifts`→`user_daily_shifts`；`TempShift`→`UserTmpDailyShift`；query 日期 `check_date_from/to`(i32 yyyyMMdd)，`user_ids` 必填 |
| `user_task_remedy`（create / query） | create 用 `remedy_date`/`punch_no`/`work_type`/`remedy_time(string)`；query 改 `user_ids` + `check_time_from/to`；`RemedyRecord` 删除（响应透传 `Value`） |
| `leave_accrual_record/patch` | 必填 `leave_granting_record_id`/`employment_id`/`leave_type_id`/`reason`；`leave_id` 为 path |
| `user_approval/query` | `user_ids` + `check_date_from/to`；`UserApproval` 删除 |
| `user_stats_view/update` | 嵌套 `view { view_id, stats_type, user_id, items[...] }`；path `user_stats_view_id` |
| `approval_info/process` | `approval_id`/`approval_type`/`status`；响应嵌套 `approval_info` |
| `archive_rule/del_report` | 必填 `month`/`operator_id`/`archive_rule_id`；响应空对象 |
| `archive_rule/upload_report` | `archive_report_datas` + `ArchiveFieldData`；响应 `invalid_code`/`invalid_member_id` |

逐 API `::new` 签名与字段表见 CHANGELOG；各 leaf 的 rustdoc/`docPath` 与官网一致。

## 5. 已删除的死 trait / helper（速查）

| 符号 | 替代 |
|------|------|
| `AsyncApiClient` / `SyncApiClient`（#504） | 直接 `Transport::request_typed` / leaf builder |
| `Response::into_result`（#505） | `Response::decode(context)`（leaf 走 `request_typed` 不受影响） |
| `ensure_success`（#506） | 空成功类 API 走 `request_typed` + 响应类型的 `ApiResponseTrait` |
| `Transport::do_send` 公开性（#478） | `pub` → `pub(crate)`；外部勿调用 |
| `auth::app_ticket::apply_app_ticket`（ADR-0002） | 由 `Transport::request` 自动恢复；模块 `pub(crate)` |
| HR 7 个 config-holder facade（#474：`Hire`/`Attendance`/…） | `client.hr.config()` 直达；`client.hr.okr.v2()` 保留 |
| security 风险评估装置 / `SecurityErrorBuilder` / `map_feishu_security_error` | 删除；用 core 通用错误构造器 |
| HR 端点 unit variant → tuple path-param | 直接构造 enum 需传参；leaf builder 零影响 |

```bash
# 升级后快速 grep 死调用点
rg 'Client::registry|\.registry\(\)|FeatureLoader|ServiceRegistry' 
rg 'AsyncApiClient|SyncApiClient|into_result|ensure_success|from_feishu_code|from_feishu_response'
rg 'ApiError.*\.status|\.status\([0-9]+\)'   # ErrorBuilder / 读字段
rg 'WsClientError::(ServerError|ClientError)'
rg 'client\.hr\.(attendance|hire|corehr|payroll|performance|compensation|ehr)\b'
```

## 6. 非破坏但相关

- **HR 共享原语**（#473）：canonical 路径 `openlark_hr::common::shared_models::*`。
  `hire::hire::common_models` 对 7 个共享类型仍 `#[deprecated]` 再导出；可选清理见 #556。
  请立即改 import，勿再依赖 alias。
- **OpenSpec 退役**：纯 process，无 Rust 公开 API 影响。

## 7. 升级自检

- [ ] 无 `client.registry()` / `ServiceRegistry` / registry prelude trait
- [ ] 错误处理读 `raw_code` / `code`，构造传 `i32`，无 `as u16` / `from_feishu_*`
- [ ] `match WsClientError` 覆盖 `RequestError(CoreError)`，无 `ServerError`/`ClientError`
- [ ] attendance 调用按上表改字段与 `::new` 签名；相关集成测试/mock 同步
- [ ] 无 `into_result` / `ensure_success` / `AsyncApiClient` / HR facade 字段
- [ ] 阅读 CHANGELOG `## [0.19.0]` Breaking 全文（本专节为摘要）

---

# OpenLark 0.18 迁移指南

适用范围：从 `0.17.x` 或更早版本迁移到 `0.18.x`

## 一句话结论

`0.18` 在 WebSocket 会话收缩之外，完成了 **编译能力 catalog 统一**与 **registry metadata-only 诊断收缩**（#423 / #434–#437）：

- 全部业务域 Client 字段与 registry 元数据由 `capability` catalog 单源生成
- `Client::registry()` 只读诊断：listing / lookup / presence / 依赖图
- 删除无法兑现的 typed-instance、虚假 lifecycle 与 `FeatureLoader` 旁路初始化

## 1. registry / FeatureLoader 迁移

> ⚠️ **0.19 已移除整个 registry 半边**（见上方 **OpenLark 0.19** 专节）。下方「推荐诊断写法」
> 仅适用于 **0.18.x**。从 0.18 升级到 0.19 时删除所有 `client.registry()` 调用即可。

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
