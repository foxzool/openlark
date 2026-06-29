## Context

issue #271 要求统一请求类型命名。openlark-auth 的请求类型用 `Builder` 后缀（模式 A），与 docs/communication 的 `Request`（模式 B）、helpdesk 的 `XxxRequest`+`XxxRequestBuilder`（模式 C）不一致。本 pilot 仅统一 auth 的 13 个请求类型 Builder，并确立 `#[deprecated]` type-alias 迁移机制供其余 crate 复用。

约束：auth 是默认 feature（`AGENTS.md` 记默认 features=auth），改动影响所有启用 auth 的用户；v0.18 已是 breaking 清理窗口（前序多个 deprecated change 已归档）。

## Goals / Non-Goals

**Goals:**
- auth 的 13 个请求类型统一为 `XxxRequest`。
- 确立 `#[deprecated]` type-alias 软迁移机制：旧名保留为 alias，调用方可编译 + warning，v1.0 移除。
- 验证此模式可复用到其余 crate（platform/ai/hr/cardkit）。

**Non-Goals:**
- 不动其他 crate（后续 change）。
- 不动 core 真·builder、helpdesk RequestBuilder、auth 的 `AuthorizationUrlBuilder`。
- 不在 v0.18 硬移除旧名（v1.0 才移除 alias）。

## Decisions

### 决策 1：方向 Builder → RequestBuilder（build 阶段从「→ Request」修订）
**选择**：auth 请求 builder 统一 `XxxRequestBuilder`（body 模型保持 `XxxRequest`）。
**理由**：open/design 原定「→ Request」，但 build 阶段实证发现 **5/13 目标 `XxxRequest` 名与 `crate::models::auth`/`models::authen` 已存在的请求体 body 模型 `XxxRequest` 撞名**（E0255 duplicate）：AppAccessTokenInternal/AppTicketResend/RefreshUserAccessTokenV1/TenantAccessTokenInternal/UserAccessTokenV1 的文件 import 了同名 body。用户确认改为「→ RequestBuilder」——body `XxxRequest` + builder `XxxRequestBuilder` 分离，零撞名，且对齐 helpdesk（47 类型已是此模式）。`TenantAccessTokenInternalRequestBuilder` 已是目标形式（不动）。
**备选**：A 混合（8 安全→Request、5 撞名→RequestBuilder）——拒绝，同 API 家族命名不一致；D 改 body 模型→XxxRequestBody 保「→Request」——scope 更大。

### 决策 2：用 `#[deprecated]` type alias 软迁移（非直接硬重命名）
**选择**：`pub struct XxxRequest { ... }` + `#[deprecated(note = "renamed to XxxRequest, will be removed in v1.0")] pub type XxxBuilder = XxxRequest;`
**理由**：auth 是默认 feature，直接硬重命名会让所有现有调用方编译失败。type alias 实现源码兼容（旧名仍编译 + warning），让用户渐进迁移，v1.0 再移除 alias。这是 Rust 生态标准重命名迁移模式。
**备选**：直接硬重命名（无 alias）——拒绝，v0.18 软 breaking 更友好；`#[deprecated]` trait/impl 包装——拒绝，type alias 零成本且够用。

### 决策 3：`XxxRequestBuilder`（已含 Request）的规范化
**选择**：`TenantAccessTokenInternalRequestBuilder` → `TenantAccessTokenInternalRequest`（去尾部 `Builder`，已有 `Request`，避免双 `Request`）。
**理由**：重命名规则不是机械后缀替换，而是"请求类型的规范名 = 主体 + Request"。对已有 `Request` 的，只去 `Builder`。

### 决策 4：prelude / re-export 同步
**选择**：auth 的 `prelude`/re-export 模块同时导出新名 `XxxRequest`；旧名 alias 也导出（带 `#[deprecated]`）。
**理由**：保证 `use openlark_auth::prelude::*` 的用户既能用新名，旧名也不破坏。

## Risks / Trade-offs

- **[type alias 与 generic/where 约束]** → auth 请求类型多为非泛型 concrete struct，type alias 直接等价，无 generic 展开问题。若个别类型有 trait impl，type alias 自动继承（alias 是透明别名）。build 阶段实证。
- **[pub use re-export 链]** → alias 在 re-export 链中需保持 `#[deprecated]` 传播；Rust 的 `#[deprecated]` 在 type alias 上对调用方生效。design/build 验证 warning 确实触发。
- **[文档/示例双名混乱]** → docs/examples 统一用新名；旧名 alias 仅作兼容层，文档注明 v1.0 移除。
- **[pilot 模式可复用性]** → 其余 crate（尤其 platform 98 个）结构同 auth，本 pilot 验证的模式可直接套用；但 platform 量大需单独 change + 可能分批。

## Migration Plan

- v0.18（本 change）：struct 重命名为 `XxxRequest`，旧名 `XxxBuilder` 作 `#[deprecated]` type alias。调用方编译通过 + warning。
- v1.0：移除 `#[deprecated]` alias（硬 breaking），CHANGELOG 给迁移指引。
- CHANGELOG v0.18 breaking 段记录 13 个重命名 + alias + 迁移说明。

## Open Questions

- 13 个 Builder 是否全部公开（pub）？非公开的无需 alias，直接改名（减少 alias 数量）。build 阶段精确核实每个的可见性。
- 是否需要在根 crate `openlark::` re-export 层也加 alias（若 auth 类型经根 crate 再导出）。build 阶段查 re-export 链。
