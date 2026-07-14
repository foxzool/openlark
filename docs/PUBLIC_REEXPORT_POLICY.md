# Canonical Public API Entrypoints

本文件用于落实 issue `#39`：冻结 `openlark`、`openlark-client` 与业务 crate 的公开入口边界，明确哪些路径是 canonical，哪些路径只是兼容层。

## 目标

- 为普通 SDK 用户冻结一套清晰、稳定的主入口。
- 明确根 crate、`openlark-client`、业务 crate、`prelude` 的职责边界。
- 保留历史兼容路径，但不再把它们继续扩张成新的“同义入口”。
- 为后续 deprecation 提供统一判断标准。

## Canonical 入口清单

### 1. 普通用户默认入口

普通用户默认从根 crate `openlark` 开始：

- 依赖入口：`openlark`
- 导入入口：`use open_lark::prelude::*;`
- 创建入口：`open_lark::Client` / `open_lark::ClientBuilder`
- 调用入口：`client.auth`、`client.communication`、`client.docs`、`client.hr`、`client.meeting`、`client.cardkit`

约束：

- 根 crate 的 canonical 语义是“先拿到 `Client`，再进入业务域字段链”。
- 根 crate 顶层暴露的 `AuthClient` / `DocsClient` / `HrClient` 等类型，主要用于类型引用和高级场景，不应替代 `Client` 成为 README/示例中的主起点。

### 2. 高级统一客户端入口

`openlark-client` 是高级入口，不是普通用户默认入口：

- 依赖入口：`openlark-client`
- 导入入口：`use openlark_client::prelude::*;`
- 创建入口：`openlark_client::Client` / `openlark_client::ClientBuilder`
- 调用入口：`client.auth`、`client.communication`、`client.docs`、`client.hr`、`client.meeting`、`client.cardkit`

适用场景：

- 明确需要直接依赖统一客户端层
- 需要使用 `ServiceRegistry` 等客户端层诊断能力
- 明确不想通过根 crate 聚合依赖

### 3. 单业务域最小依赖入口

当用户只需要单一业务域能力时，canonical 入口是直接依赖对应业务 crate：

- `openlark-docs` → `DocsClient`
- `openlark-communication` → `CommunicationClient`
- `openlark-hr` → `HrClient`
- `openlark-meeting` → `MeetingClient`
- `openlark-cardkit` → `CardkitClient`
- `openlark-auth` → `AuthService` / `AuthenService` / `OAuthService`

约束：

- 业务 crate 继续持有自己的 request / response / resource / version 类型。
- 根 crate 不上提业务 crate 的深层类型，只保留顶层命名空间和必要的高频类型。

## 非 Canonical 但保留的兼容入口

以下路径当前仍然保留，但视为 compatibility aliases，不再作为新增文档、示例、README 或新 API 设计的依据：

- `open_lark::openlark_client::*`
- `open_lark::openlark_core::*`
- `open_lark::openlark_auth::*`
- `open_lark::openlark_communication::*`
- `open_lark::openlark_docs::*`
- `open_lark::openlark_webhook::*`
- 直接从根 crate 顶层构造业务 meta client，例如把 `open_lark::DocsClient` 当作普通用户主入口

保留理由：

- 兼容已有代码和文档链接
- 避免在 `0.15.x` 中制造破坏性删除

收敛要求：

- 新文档不再推广这些路径
- 新模块不得继续添加同类型的 passthrough re-export
- 如果兼容路径与 canonical 入口产生歧义，优先隐藏文档面或加 deprecation，而不是继续扩张

## 分层职责边界

### 1. 根 crate `openlark`

负责暴露：

- `Client`、`ClientBuilder`
- `Config`、`Error`、`Result`、`SDKResult`
- `RequestOption`、`CoreConfig`、`CoreError` 等跨业务通用基础类型
- 按 feature 打开的业务命名空间别名：`open_lark::auth`、`open_lark::communication`、`open_lark::docs`、`open_lark::hr` 等
- 为兼容历史代码保留的顶层 meta client 类型：`AuthClient`、`CommunicationClient`、`DocsClient`、`HrClient`、`MeetingClient`、`CardkitClient`

不应新增：

- `ServiceRegistry` / trait 等客户端内部实现细节
- 业务 crate 的 resource / version / request / response 类型
- 仅服务调试或内部拼装的结构

### 2. `openlark-client`

负责暴露：

- 统一客户端层的核心类型
- `ServiceRegistry`（metadata-only 诊断）、客户端 trait 等高级能力
- 各业务 crate 的顶层 meta client 类型

`openlark-client::prelude` 可暴露：

- 创建客户端必需的核心类型
- 高频错误类型与常用扩展
- 顶层 meta client 类型

`openlark-client::prelude` 不应新增：

- 深层 resource / version / request / response 类型
- 仅少数高级场景才使用的内部辅助类型

### 3. 业务 crate

负责暴露：

- 自己的权威入口
- 自己的局部 `prelude`
- 自己的 request / response / models / resource / version 类型

根 crate 和 `openlark-client` 只允许引用它们的顶层入口，不继续吸收深层结构。

## `prelude` 准入规则

只有同时满足以下条件的类型才允许进入根 crate `prelude`：

1. 普通用户在“创建客户端并发起调用”时高频使用。
2. 语义稳定，不依赖某个业务域的内部组织方式。
3. 跨 feature 成立，或属于顶层 meta 入口。
4. 不会制造新的同义入口或命名歧义。

默认不进入根 crate `prelude`：

- request / response / model
- 版本层 `*V1Service` / `*V2Service`
- resource 层类型
- registry / traits / bootstrap / internal helper

## 新模块接入规则

新增模块接入 `openlark` 时，按以下顺序判断：

1. 是否已经有业务 crate 的权威入口。
2. 根 crate 是否只需要提供顶层命名空间别名或顶层 meta client。
3. 放入 `prelude` 是否会制造重复语义入口。
4. 若已有一条 canonical 路径，则新路径只能作为兼容别名，不能再被宣传为主入口。

## 迁移与 Deprecation 策略

`0.15.x` 策略：

- 不破坏性删除现有兼容入口。
- 隐藏明显冗余的文档可见面，避免继续误导新用户。
- README、示例、crate docs 统一只展示 canonical 入口。

后续 minor 版本策略：

- 若兼容入口与 canonical 入口持续重叠且没有保留价值，可先加 `#[deprecated]` 与迁移说明。
- deprecation 至少保留一个明确发布周期，并在 migration guide / changelog 中给出替代路径。
- 只有在迁移说明和 deprecation 已完成后，才允许考虑移除兼容入口。

当前已补齐的迁移说明见：

- `docs/legacy-entrypoint-migration-notes.md`

## 当前冻结结论

- 根 crate 的 canonical 入口冻结为：`Client` / `ClientBuilder` / `prelude` / `client.<domain>`
- `openlark-client` 的 canonical 入口冻结为：高级统一客户端层
- 业务 crate 的 canonical 入口冻结为：各自顶层 client / service
- 原始 crate-name passthrough re-export 仅保留兼容职责，不再视为主入口
