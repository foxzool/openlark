# Comet Design Handoff

- Change: unify-auth-request-naming
- Phase: design
- Mode: compact
- Context hash: 93e8fde34c33c70c0f347b4d62d46ca321f1a559510dfbe4279667a7fb560303

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/unify-auth-request-naming/proposal.md

- Source: openspec/changes/unify-auth-request-naming/proposal.md
- Lines: 1-40
- SHA256: 6103d4845788cab33ffbbeb6f04cf6aa60f4d6518b0ecb72feafd4d2b0db927d

```md
## Why

issue #271（2026-06 架构审计立项「Request vs Builder 命名统一」）：请求类型命名跨 crate 不一致——模式 A（platform/ai/auth/hr）的请求类型用 `Builder` 后缀（如 `AppAccessTokenBuilder`），模式 B（docs/communication）用 `Request` 后缀（如 `ListClassificationRequest`），模式 C（helpdesk）用 `XxxRequest` + `XxxRequestBuilder` 分离模式。用户需记忆不同 crate 的命名惯例，SDK 表面不一致。

本 change 是 **pilot**：只统一 **openlark-auth** 的请求类型命名（`Builder` → `Request`），并确立 `#[deprecated]` type-alias 迁移机制（v0.18→v1.0 渐进 breaking）。验证模式后，其余 crate（platform 98 / ai 29 / hr 13 / cardkit 10 等）按批次另开 change。全局统计：229 个 `Builder` 中约 167 个是模式 A 请求类型，但 1670 个已是 `Request`——方向 `Builder → Request` 有数据支撑。

## What Changes

- 将 openlark-auth 的 **13 个请求类型 `XxxBuilder` 重命名为规范 `XxxRequest`**（经核实均有 `execute()`，确为请求类型）：
  - `AppAccessTokenBuilder` → `AppAccessTokenRequest`
  - `AppAccessTokenInternalBuilder` → `AppAccessTokenInternalRequest`
  - `AppTicketResendBuilder` → `AppTicketResendRequest`
  - `AuthorizationBuilder` → `AuthorizationRequest`
  - `IdentityCreateBuilder` → `IdentityCreateRequest`
  - `OidcAccessTokenBuilder` → `OidcAccessTokenRequest`
  - `OidcRefreshAccessTokenBuilder` → `OidcRefreshAccessTokenRequest`
  - `RefreshUserAccessTokenV1Builder` → `RefreshUserAccessTokenV1Request`
  - `TenantAccessTokenBuilder` → `TenantAccessTokenRequest`
  - `TenantAccessTokenInternalRequestBuilder` → `TenantAccessTokenInternalRequest`（已有 `Request`，仅去 `Builder`，避免双 `Request`）
  - `UserAccessTokenV1Builder` → `UserAccessTokenV1Request`
  - `UserInfoBuilder` → `UserInfoRequest`
  - `VerificationGetBuilder` → `VerificationGetRequest`
- **保留旧名作为 `#[deprecated]` type alias**（`pub type XxxBuilder = XxxRequest;` + `#[deprecated(note="...")]`），实现 v0.18→v1.0 渐进迁移：调用方仍可编译，但用旧名会有 deprecation warning。
- 同步更新 auth 内部引用、prelude/re-export、examples、docs、tests 到新名。
- **BREAKING**（软）：公开类型重命名；但通过 `#[deprecated]` alias 保持源码兼容（仅 warning，非硬破坏），v1.0 移除 alias。

## Capabilities

### New Capabilities
- `auth-request-naming`: openlark-auth 的请求类型 SHALL 统一使用 `Request` 后缀；旧 `Builder` 名 SHALL 作为 `#[deprecated]` type alias 保留至 v1.0，调用方使用旧名时编译可通过但产生 deprecation warning。

### Modified Capabilities
<!-- 无（无既有 auth 命名相关 main spec） -->

## Impact

- **openlark-auth**：13 个请求类型 struct 重命名 + 13 个 `#[deprecated]` type alias + 内部引用/prelude 同步 + 模块路径调整。
- **examples/docs/tests**：auth 相关引用更新到新名（旧名仍可编译但有 warning，可渐进）。
- **破坏性**：软 breaking——公开类型重命名，但 `#[deprecated]` type alias 保证源码兼容（编译通过 + warning）。二进制兼容性不受影响（type alias 是零成本）。v1.0 移除 alias 时变硬 breaking。
- **非目标**：不动其他 crate（platform/ai/hr/cardkit 等，后续 change）；不动 core 真·builder（`ConfigBuilder`/`HeaderBuilder`/`MultipartBuilder`/...）；不动 helpdesk `XxxRequestBuilder`（已是 Request 模式）；不动 auth 的 `AuthorizationUrlBuilder`（无 `execute()`，是 URL builder 非请求类型）。
```

## openspec/changes/unify-auth-request-naming/design.md

- Source: openspec/changes/unify-auth-request-naming/design.md
- Lines: 1-55
- SHA256: 450152796fde478814596d7f7e321b15f9ae0b10fb415143e228771597f46474

```md
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

### 决策 1：方向 Builder → Request（非 Request → Builder）
**选择**：模式 A 的 `XxxBuilder` → `XxxRequest`。
**理由**：全局 1670 个已是 `Request`（模式 B）+ helpdesk（模式 C）的请求类型也是 `Request`，仅约 167 个模式 A 用 `Builder`。改少数对齐多数，改动面最小。`Request` 更语义化（表明这是 API 请求载体）。
**备选**：Request → Builder——拒绝，要动 1670+ 个，改动面大 10 倍。

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
```

## openspec/changes/unify-auth-request-naming/tasks.md

- Source: openspec/changes/unify-auth-request-naming/tasks.md
- Lines: 1-30
- SHA256: c5d2d24f057f44913940352777fb29d5a5aafbc0d8d0668c38c316c24c3b2234

```md
# Tasks — unify-auth-request-naming（#271 pilot）

> issue #271 命名统一 pilot：openlark-auth 13 个请求类型 `XxxBuilder` → `XxxRequest` + `#[deprecated]` type alias 软迁移。
> 验证模式后，其余 crate（platform/ai/hr/cardkit）按批次另开 change。

## 1. 精确化范围

- [ ] 1.1 核实 13 个 Builder 的可见性（pub 与否）+ auth prelude/re-export 链 + 根 crate `openlark::` 是否再导出；确定哪些需 `#[deprecated]` alias（公开的）、哪些可直接改名（内部的）

## 2. 重命名 + alias

- [ ] 2.1 将 13 个请求类型 struct `XxxBuilder` 重命名为 `XxxRequest`（`TenantAccessTokenInternalRequestBuilder` → `TenantAccessTokenInternalRequest`，去 Builder 避免双 Request）；同步 struct 定义、impl 块、new() 返回类型
- [ ] 2.2 为 13 个旧名添加 `#[deprecated(note = "renamed to XxxRequest, will be removed in v1.0 (#271)")] pub type XxxBuilder = XxxRequest;`
- [ ] 2.3 同步 auth 内部所有引用（同 crate 内 `use`/类型标注）到新名；prelude/re-export 同时导出新名 + 旧名 alias

## 3. examples / docs / tests

- [ ] 3.1 更新 auth 相关 examples、docs、tests 到新名（旧名 alias 保留兼容，但新代码用新名）；为 alias 的 deprecation warning 行为加针对性测试

## 4. CHANGELOG

- [ ] 4.1 CHANGELOG v0.18 Breaking Changes 段记录 13 个重命名 + `#[deprecated]` alias + v1.0 移除指引 + 迁移说明（`XxxBuilder` → `XxxRequest`）

## 5. 验证

- [ ] 5.1 `cargo build --workspace --all-features` exit 0
- [ ] 5.2 三组 feature clippy（default / `--all-features` / `--no-default-features` + `-D warnings`）均 exit 0
- [ ] 5.3 `cargo test -p openlark-auth` 全部通过（0 failed）
- [ ] 5.4 实证 alias：用旧名 `AppAccessTokenBuilder` 写临时代码 build → 成功 + deprecation warning；用新名 → 成功无 warning；还原
- [ ] 5.5 grep 确认：13 个 `XxxRequest` struct 存在、13 个 `XxxBuilder` type alias 带 `#[deprecated]`、`AuthorizationUrlBuilder` 未被改动
```

## openspec/changes/unify-auth-request-naming/specs/auth-request-naming/spec.md

- Source: openspec/changes/unify-auth-request-naming/specs/auth-request-naming/spec.md
- Lines: 1-46
- SHA256: 778a8935f99e7e1a015f5cc658a271ff8511b1414f9e7ddc98f8be7981aeffb2

```md
## ADDED Requirements

### Requirement: auth 请求类型统一使用 Request 后缀
openlark-auth 的 API 请求类型 SHALL 统一使用 `Request` 后缀命名（对齐 docs/communication/helpdesk 的既有惯例）。13 个原 `XxxBuilder` 请求类型 SHALL 重命名为规范 `XxxRequest`；`TenantAccessTokenInternalRequestBuilder` SHALL 重命名为 `TenantAccessTokenInternalRequest`（去 `Builder`，避免双 `Request`）。

#### Scenario: 13 个请求类型重命名为 Request
- **WHEN** 在 `crates/openlark-auth/src/` 中 grep `pub struct XxxRequest`（针对 AppAccessToken/AppAccessTokenInternal/AppTicketResend/Authorization/IdentityCreate/OidcAccessToken/OidcRefreshAccessToken/RefreshUserAccessTokenV1/TenantAccessToken/TenantAccessTokenInternal/UserAccessTokenV1/UserInfo/VerificationGet）
- **THEN** 13 个 `XxxRequest` struct 均存在（重命名完成）

#### Scenario: auth 不再有裸 Builder 请求类型
- **WHEN** 在 `crates/openlark-auth/src/` 中 grep 请求类型的 `XxxBuilder`（排除 `AuthorizationUrlBuilder` 这个 URL builder）
- **THEN** 13 个请求 Builder 名不再作为 struct 定义存在（已重命名为 Request）

#### Scenario: AuthorizationUrlBuilder 不被误改
- **WHEN** 检查 `crates/openlark-auth/src/`
- **THEN** `AuthorizationUrlBuilder`（URL builder，非请求类型）保留原样不动

### Requirement: 旧 Builder 名作 #[deprecated] type alias 保留
原 13 个 `XxxBuilder` 名 SHALL 作为 `#[deprecated]` type alias 保留（`#[deprecated] pub type XxxBuilder = XxxRequest;`），实现 v0.18→v1.0 软迁移：调用方使用旧名时编译可通过但产生 deprecation warning。v1.0 移除 alias。

#### Scenario: 旧名 type alias 存在且标 deprecated
- **WHEN** 在 `crates/openlark-auth/src/` 中 grep `pub type XxxBuilder`（13 个旧名）
- **THEN** 13 个 type alias 存在，且各自带 `#[deprecated]` 属性

#### Scenario: 旧名调用产生 deprecation warning
- **WHEN** 用旧名 `AppAccessTokenBuilder`（alias）写测试代码并 `cargo build -p openlark-auth`
- **THEN** 构建成功（源码兼容）且产生 `AppAccessTokenBuilder is deprecated` warning

#### Scenario: 新名无 deprecation warning
- **WHEN** 用新名 `AppAccessTokenRequest` 写测试代码并 `cargo build -p openlark-auth`
- **THEN** 构建成功且无该类型的 deprecation warning

### Requirement: 重命名不破坏构建、lint 与测试
本次重命名 + alias SHALL 不导致 workspace 构建、clippy 或测试失败，SHALL 同步更新 auth 内部引用、prelude/re-export、examples、docs、tests 到新名。

#### Scenario: 全 feature 构建通过
- **WHEN** 运行 `cargo build --workspace --all-features`
- **THEN** exit 0

#### Scenario: 三组 feature clippy 通过
- **WHEN** 运行 `cargo clippy --workspace --all-targets` 分别以 default、`--all-features`、`--no-default-features` + `-D warnings`（deprecation warning 不属 clippy `-D warnings` 抑制范围，alias 自身的 `#[deprecated]` 不在 clippy lint 内）
- **THEN** 三组均 exit 0

#### Scenario: auth 测试通过
- **WHEN** 运行 `cargo test -p openlark-auth`
- **THEN** 全部通过（0 failed；测试用新名，旧名 alias 测试单独覆盖 warning 行为）
```

