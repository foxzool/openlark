# Legacy Entrypoint Migration Notes

本文档记录 `0.16.0` 起开始标记为 deprecated 的 legacy entrypoints，以及每类入口的推荐迁移方向。

## 1. 范围

本轮 deprecation 只覆盖**根 crate `openlark` 中的历史兼容入口**：

1. 原始 crate-name passthrough re-export
2. 根 crate 顶层 `*Client` compatibility alias

以下内容**不在本轮 deprecated 范围内**：

- `openlark-client` crate 本身（它仍然是高级入口）
- 根 crate canonical 入口：`Client` / `ClientBuilder` / `prelude` / `auth` / `communication` / `docs` 等业务命名空间
- 各业务 crate 自己的权威入口

## 2. 类别一：原始 crate-name passthrough re-export

这些路径历史上为了兼容旧代码保留，但从 `0.16.0` 起不再建议继续直接使用。

| 已 deprecated 的入口 | 推荐替代路径 | 说明 |
| --- | --- | --- |
| `open_lark::openlark_client::*` | `open_lark::{Client, ClientBuilder, prelude}`，或直接依赖 `openlark-client` | 普通用户应回到根 crate canonical 入口；高级用户可显式依赖 `openlark-client` |
| `open_lark::openlark_core::*` / `open_lark::core::*` | 根 crate 公开的 `CoreConfig` / `CoreError` / `RequestOption` / `SDKResult`，或直接依赖 `openlark-core` | 避免继续通过隐藏 passthrough 心智访问 core crate |
| `open_lark::openlark_auth::*` | `open_lark::auth::*` | 根 crate 已提供 canonical 业务命名空间 |
| `open_lark::openlark_communication::*` | `open_lark::communication::*` | 同上 |
| `open_lark::openlark_docs::*` | `open_lark::docs::*` | 同上 |
| `open_lark::openlark_webhook::*` | `open_lark::webhook::*` | 同上 |

## 3. 类别二：根 crate 顶层 `*Client` compatibility alias

以下根 crate 顶层 alias 从 `0.16.0` 起标记为 deprecated：

- `AuthClient`
- `CommunicationClient`
- `DocsClient`
- `HrClient`
- `MeetingClient`
- `CardkitClient`
- `AiClient`
- `WorkflowClient`
- `PlatformClient`
- `ApplicationClient`
- `HelpdeskClient`
- `MailClient`
- `AnalyticsClient`
- `UserClient`
- `SecurityClient`

它们的迁移规则如下：

| 旧入口类别 | 运行时推荐入口 | 显式类型推荐入口 |
| --- | --- | --- |
| 根 crate 顶层 `*Client` alias | `Client` 实例字段（如 `client.docs`、`client.communication`、`client.workflow`） | `openlark_client::<XxxClient>`，或对应业务 crate 的顶层类型 |

示例：

### Before

```rust
use open_lark::DocsClient;
```

### After

```rust
use open_lark::Client;

let client = Client::builder()
    .app_id("app_id")
    .app_secret("app_secret")
    .build()?;

let docs = &client.docs;
```

如果你确实需要显式类型名，请改为：

```rust
use openlark_client::DocsClient;
```

## 4. 迁移判断原则

遇到 legacy entrypoint 时，按以下顺序选择替代路径：

1. **普通业务调用**：优先回到根 crate `Client` + 业务字段链
2. **只需要单一业务域**：直接依赖对应业务 crate
3. **明确需要统一客户端层高级能力**：直接依赖 `openlark-client`
4. **只需要 core 通用类型**：优先用根 crate 已公开的稳定通用类型；必要时直接依赖 `openlark-core`

## 5. 发布纪律

这些 deprecated legacy entrypoints 在 `0.15.x` 周期内仍会保留兼容职责，但：

- 新文档、README、示例不再推广它们
- 未来 minor 版本若要移除，必须先更新 migration guide、release notes 与 compatibility note
- 如果某类 alias 仍然有保留价值，应给出书面解释，而不是继续扩张使用面

更一般的 deprecated API 支持周期规则见：

- `docs/DEPRECATED_API_SUPPORT_POLICY.md`
