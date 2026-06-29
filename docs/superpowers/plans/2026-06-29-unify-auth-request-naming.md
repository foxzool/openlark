---
change: unify-auth-request-naming
design-doc: docs/superpowers/specs/2026-06-29-unify-auth-request-naming-design.md
base-ref: 6557def919fc4a59d03e664291548d75a01ef4e3
---

# unify-auth-request-naming 实施计划（#271 pilot）

> **For agentic workers:** REQUIRED SUB-SKILL: 使用 superpowers:subagent-driven-development（推荐）或 superpowers:executing-plans 按任务逐项实施。步骤使用 `- [ ]` 复选框跟踪。

**Goal:** 将 openlark-auth 的 13 个请求类型 `XxxBuilder` 重命名为规范 `XxxRequest`，旧名作 `#[deprecated]` type alias 保留（v0.18→v1.0 软迁移），不破坏构建/clippy/测试。

**Architecture:** 每个类型三步走：① `pub struct XxxBuilder` + `impl XxxBuilder` 重命名为 `XxxRequest`；② 原文件加 `#[deprecated] pub type XxxBuilder = XxxRequest;`；③ 同步 auth 内部所有 re-export 链与 service 方法返回类型注解。`#[deprecated]` type-alias 机制已用 /tmp spike 实证（旧名触发 warning、新名无 warning、方法经 alias 可调用）。

**Tech Stack:** Rust（edition 2021 / MSRV 1.88+）、cargo、clippy、wiremock（既有 auth 测试）。

## Global Constraints

- **语言**：注释/文档/commit message 使用中文（项目约定，面向中国开发者）。
- **不改的逻辑**：方法签名（除类型名）、字段、`execute()`/`execute_with_options()` 业务逻辑、API 端点、`validate_required!` 全部不变——本次只做"改名 + 加 alias + 同步引用"。
- **不改的对象**：`AuthorizationUrlBuilder`（`crates/openlark-auth/src/models/oauth/mod.rs:34`，URL builder 无 `execute()`，**禁止触碰**）；core 真·builder；其他 crate。
- **别名规格统一**：`#[deprecated(note = "renamed to XxxRequest, will be removed in v1.0 (#271)")] pub type XxxBuilder = XxxRequest;`（note 文案逐类型替换新名）。
- **新名引用**：auth 内部 re-export 与 service 方法返回类型**全部用新名**；旧名 alias 也在每个 `pub use` 处同步导出（保证旧调用方仍可达）。
- **frequent commits**：每个任务结束 commit，禁止积攒。
- **库代码禁用** `unwrap()`/`expect()`（既有约定，本次重命名不涉及新增）。
- **测试不污染输出**：旧名 alias 的 deprecation-warning 测试用 `#[allow(deprecated)]` 标注，避免 `cargo test` 刷屏。

---

## 13 个类型旧名 → 新名映射（权威表，后续任务直接引用）

| # | 旧名（struct） | 新名（struct） | 定义文件 | pub? |
|---|---|---|---|---|
| 1 | `AppAccessTokenBuilder` | `AppAccessTokenRequest` | `crates/openlark-auth/src/auth/auth/v3/auth/app_access_token.rs:28` | pub |
| 2 | `AppAccessTokenInternalBuilder` | `AppAccessTokenInternalRequest` | `crates/openlark-auth/src/auth/auth/v3/auth/app_access_token_internal.rs:18` | pub |
| 3 | `AppTicketResendBuilder` | `AppTicketResendRequest` | `crates/openlark-auth/src/auth/auth/v3/auth/app_ticket_resend.rs:21` | pub |
| 4 | `AuthorizationBuilder` | `AuthorizationRequest` | `crates/openlark-auth/src/auth/oauth/old/default/index.rs:12` | pub |
| 5 | `IdentityCreateBuilder` | `IdentityCreateRequest` | `crates/openlark-auth/src/human_authentication/human_authentication/v1/identity/create.rs:90` | pub |
| 6 | `OidcAccessTokenBuilder` | `OidcAccessTokenRequest` | `crates/openlark-auth/src/auth/authen/v1/oidc/access_token/create.rs:20` | pub |
| 7 | `OidcRefreshAccessTokenBuilder` | `OidcRefreshAccessTokenRequest` | `crates/openlark-auth/src/auth/authen/v1/oidc/refresh_access_token/create.rs:20` | pub |
| 8 | `RefreshUserAccessTokenV1Builder` | `RefreshUserAccessTokenV1Request` | `crates/openlark-auth/src/auth/authen/v1/refresh_access_token/create.rs:21` | pub |
| 9 | `TenantAccessTokenBuilder` | `TenantAccessTokenRequest` | `crates/openlark-auth/src/auth/auth/v3/auth/tenant_access_token.rs:27` | pub |
| 10 | `TenantAccessTokenInternalRequestBuilder` | `TenantAccessTokenInternalRequest` | `crates/openlark-auth/src/auth/auth/v3/auth/tenant_access_token_internal.rs:18` | pub |
| 11 | `UserAccessTokenV1Builder` | `UserAccessTokenV1Request` | `crates/openlark-auth/src/auth/authen/v1/access_token/create.rs:20` | pub |
| 12 | `UserInfoBuilder` | `UserInfoRequest` | `crates/openlark-auth/src/auth/authen/v1/user_info/get.rs:20` | pub |
| 13 | `VerificationGetBuilder` | `VerificationGetRequest` | `crates/openlark-auth/src/verification_information/verification/v1/verification/get.rs:17` | pub |

> 全部为 `pub struct`，全部需要 `#[deprecated]` alias（无内部类型可省 alias）。

## 影响面全景（核实结果，直接用，无需再探索）

**A. re-export 链（每个 `pub use` 行：新名替换旧名 + 额外加旧名 alias 导出）**

| 文件 | 涉及的旧名 |
|------|-----------|
| `crates/openlark-auth/src/auth/mod.rs:13-16` | AppAccessTokenBuilder, AppAccessTokenInternalBuilder, AppTicketResendBuilder, TenantAccessTokenBuilder, TenantAccessTokenInternalRequestBuilder（5 个，与 AuthServiceV3 同行） |
| `crates/openlark-auth/src/auth/auth/v3/auth/mod.rs:15-19` | AppAccessTokenBuilder, AppAccessTokenInternalBuilder, AppTicketResendBuilder, TenantAccessTokenBuilder, TenantAccessTokenInternalRequestBuilder（5 个） |
| `crates/openlark-auth/src/auth/oauth/mod.rs:7` | AuthorizationBuilder |
| `crates/openlark-auth/src/auth/oauth/old/mod.rs:6` | AuthorizationBuilder |
| `crates/openlark-auth/src/auth/oauth/old/default/mod.rs:6` | AuthorizationBuilder |
| `crates/openlark-auth/src/auth/authen/mod.rs:12-13` | OidcAccessTokenBuilder, OidcRefreshAccessTokenBuilder, RefreshUserAccessTokenV1Builder, UserAccessTokenV1Builder, UserInfoBuilder（5 个） |
| `crates/openlark-auth/src/auth/authen/v1/mod.rs:16-19` | UserAccessTokenV1Builder, OidcAccessTokenBuilder, OidcRefreshAccessTokenBuilder, RefreshUserAccessTokenV1Builder, UserInfoBuilder（5 个） |
| `crates/openlark-auth/src/auth/authen/v1/access_token/mod.rs:8` | UserAccessTokenV1Builder |
| `crates/openlark-auth/src/auth/authen/v1/refresh_access_token/mod.rs:8` | RefreshUserAccessTokenV1Builder |
| `crates/openlark-auth/src/auth/authen/v1/oidc/mod.rs:9-10` | OidcAccessTokenBuilder, OidcRefreshAccessTokenBuilder |
| `crates/openlark-auth/src/auth/authen/v1/user_info/mod.rs:8` | UserInfoBuilder |
| `crates/openlark-auth/src/human_authentication/human_authentication/v1/identity/mod.rs:5` | IdentityCreateBuilder |
| `VerificationGetBuilder` | **无 re-export**（直接经模块路径使用，跳过 A 步） |

**B. service 方法返回类型注解（公开 API 表面，必须改新名）— 9 个方法**

| 文件 | 方法 |
|------|------|
| `crates/openlark-auth/src/auth/auth/v3/auth/mod.rs:37,42,47,52,57` | `AuthServiceV3::app_access_token/app_access_token_internal/tenant_access_token/tenant_access_token_internal/app_ticket_resend`（5 个） |
| `crates/openlark-auth/src/auth/authen/v1/oidc/mod.rs:28,33` | `OidcService::access_token/refresh_access_token`（2 个） |
| `crates/openlark-auth/src/auth/authen/v1/mod.rs:44,49` | `AuthenServiceV1::access_token/refresh_access_token`（2 个） |

**C. 既有测试**（13 个定义文件内的 `#[cfg(test)] mod tests`，用旧名 `XxxBuilder::new(...)`）：改新名（旧名 alias 测试单独加，见 Task 3）。

**D. 外部引用**（examples/ / tests/ / 业务 docs/）：`rg` 实证**无任何引用**（仅 superpowers specs/plans 文档提及，属历史记录，不在范围）。→ Task 3 的 examples/docs 更新退化为"无需更新"，只剩 alias 行为测试。

**E. 根 crate**：`src/lib.rs:48` 仅 `pub use openlark_auth as auth;`（命名空间 re-export），**不直接再导出 13 个类型名**。→ 无需改根 crate。

---

## Task 1: 精确核实可见性与影响面（写代码前的基线）

**Files:**
- Read: 上述 13 个定义文件 + 13 个 re-export 文件 + 3 个 service 方法文件
- 无修改

**Interfaces:**
- Consumes: Design Doc「关键事实」
- Produces: 一份核实清单（即上文「13 个映射表 + 影响面全景」），作为后续任务的权威依据。本计划已内置该核实结果，此 Task 的执行=用 rg 复跑一遍命令确认未漂移。

- [ ] **Step 1: 复跑核实命令，确认 13 个 struct 仍为 `pub struct` 且位置未变**

Run:
```bash
rg -n "^pub struct (AppAccessTokenBuilder|AppAccessTokenInternalBuilder|AppTicketResendBuilder|AuthorizationBuilder|IdentityCreateBuilder|OidcAccessTokenBuilder|OidcRefreshAccessTokenBuilder|RefreshUserAccessTokenV1Builder|TenantAccessTokenBuilder|TenantAccessTokenInternalRequestBuilder|UserAccessTokenV1Builder|UserInfoBuilder|VerificationGetBuilder)\b" crates/openlark-auth/src/
```
Expected: 恰好 13 行命中，文件路径 + 行号与本计划「映射表」一致。

- [ ] **Step 2: 确认 AuthorizationUrlBuilder 存在且本次不动**

Run:
```bash
rg -n "struct AuthorizationUrlBuilder" crates/openlark-auth/src/models/oauth/mod.rs
```
Expected: 1 行命中（`pub struct AuthorizationUrlBuilder {`），作为"禁止改动"对照。

- [ ] **Step 3: 确认 examples/tests/docs 无外部引用**

Run:
```bash
rg -l "\b(AppAccessTokenBuilder|AppAccessTokenInternalBuilder|AppTicketResendBuilder|AuthorizationBuilder|IdentityCreateBuilder|OidcAccessTokenBuilder|OidcRefreshAccessTokenBuilder|RefreshUserAccessTokenV1Builder|TenantAccessTokenBuilder|TenantAccessTokenInternalRequestBuilder|UserAccessTokenV1Builder|UserInfoBuilder|VerificationGetBuilder)\b" examples/ tests/ 2>/dev/null
```
Expected: 空输出（无引用）。

- [ ] **Step 4: 确认根 crate 仅命名空间 re-export**

Run:
```bash
rg -n "openlark_auth" src/lib.rs
```
Expected: 仅 `pub use openlark_auth as auth;`，无 13 个类型名直接 re-export。

> 核实不通过则停止，回 design 修正；通过则进入 Task 2（不单独 commit 核实）。

---

## Task 2: v3 auth 子系统重命名 + alias（5 个类型，批次 1）

5 个 v3 auth 请求类型：AppAccessToken / AppAccessTokenInternal / AppTicketResend / TenantAccessToken / TenantAccessTokenInternal（特例）。这是 surface 最广的一批（牵动 `AuthServiceV3` 5 个方法 + 2 层 re-export），先做以尽早暴露风险。

**Files:**
- Modify: `crates/openlark-auth/src/auth/auth/v3/auth/app_access_token.rs`
- Modify: `crates/openlark-auth/src/auth/auth/v3/auth/app_access_token_internal.rs`
- Modify: `crates/openlark-auth/src/auth/auth/v3/auth/app_ticket_resend.rs`
- Modify: `crates/openlark-auth/src/auth/auth/v3/auth/tenant_access_token.rs`
- Modify: `crates/openlark-auth/src/auth/auth/v3/auth/tenant_access_token_internal.rs`
- Modify: `crates/openlark-auth/src/auth/auth/v3/auth/mod.rs`（5 个 pub use + AuthServiceV3 5 个方法返回类型）
- Modify: `crates/openlark-auth/src/auth/mod.rs:13-16`（5 个 pub use）

**Interfaces:**
- Consumes: 无（第一批）
- Produces: 5 个新 `XxxRequest` struct（含 `new()`/setter/`execute()`/`execute_with_options()`）、5 个 `#[deprecated] pub type XxxBuilder = XxxRequest;` alias；`AuthServiceV3` 5 方法返回新名。

- [ ] **Step 1: 单类型重命名 AppAccessTokenBuilder → AppAccessTokenRequest**

在 `app_access_token.rs`：
1. `pub struct AppAccessTokenBuilder {` → `pub struct AppAccessTokenRequest {`
2. `impl AppAccessTokenBuilder {` → `impl AppAccessTokenRequest {`
3. 文件内 `#[cfg(test)] mod tests` 所有 `AppAccessTokenBuilder::new(...)` → `AppAccessTokenRequest::new(...)`（5 处：`test_app_access_token_builder_new/chain/app_id_chained/app_secret_chained/app_ticket_chained` + `test_execute_sends_app_ticket_and_no_authorization`）
4. 文件末尾（`impl` 块之后、`#[cfg(test)]` 之前）追加：
```rust
/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to AppAccessTokenRequest, will be removed in v1.0 (#271)")]
pub type AppAccessTokenBuilder = AppAccessTokenRequest;
```

- [ ] **Step 2: 其余 4 个 v3 类型同理重命名 + 加 alias**

按 Step 1 的 4 步法，对：
- `app_access_token_internal.rs`：`AppAccessTokenInternalBuilder` → `AppAccessTokenInternalRequest`
- `app_ticket_resend.rs`：`AppTicketResendBuilder` → `AppTicketResendRequest`
- `tenant_access_token.rs`：`TenantAccessTokenBuilder` → `TenantAccessTokenRequest`
- `tenant_access_token_internal.rs`：**特例** `TenantAccessTokenInternalRequestBuilder` → `TenantAccessTokenInternalRequest`（去 `Builder`，避免双 `Request`）；alias 写作：
```rust
#[deprecated(note = "renamed to TenantAccessTokenInternalRequest, will be removed in v1.0 (#271)")]
pub type TenantAccessTokenInternalRequestBuilder = TenantAccessTokenInternalRequest;
```

每个文件末尾追加对应 `#[deprecated] pub type <旧名> = <新名>;`。

- [ ] **Step 3: 同步 auth/auth/v3/auth/mod.rs 的 5 个 pub use + AuthServiceV3 5 方法**

将 `mod.rs:15-19`：
```rust
pub use app_access_token::AppAccessTokenBuilder;
pub use app_access_token_internal::AppAccessTokenInternalBuilder;
pub use app_ticket_resend::AppTicketResendBuilder;
pub use tenant_access_token::TenantAccessTokenBuilder;
pub use tenant_access_token_internal::TenantAccessTokenInternalRequestBuilder;
```
改为（新名 + 旧名 alias 双导出）：
```rust
pub use app_access_token::AppAccessTokenRequest;
pub use app_access_token_internal::AppAccessTokenInternalRequest;
pub use app_ticket_resend::AppTicketResendRequest;
pub use tenant_access_token::TenantAccessTokenRequest;
pub use tenant_access_token_internal::TenantAccessTokenInternalRequest;
#[allow(deprecated)]
pub use app_access_token::AppAccessTokenBuilder;
#[allow(deprecated)]
pub use app_access_token_internal::AppAccessTokenInternalBuilder;
#[allow(deprecated)]
pub use app_ticket_resend::AppTicketResendBuilder;
#[allow(deprecated)]
pub use tenant_access_token::TenantAccessTokenBuilder;
#[allow(deprecated)]
pub use tenant_access_token_internal::TenantAccessTokenInternalRequestBuilder;
```
> `#[allow(deprecated)]` 必要：re-export deprecated item 会触发 lint。

并将 `AuthServiceV3` 5 个方法（`mod.rs:37,42,47,52,57`）返回类型与新名 `::new(...)` 调用改为新名：
```rust
pub fn app_access_token(&self) -> AppAccessTokenRequest {
    AppAccessTokenRequest::new(self.config.clone())
}
pub fn app_access_token_internal(&self) -> AppAccessTokenInternalRequest {
    AppAccessTokenInternalRequest::new(self.config.clone())
}
pub fn tenant_access_token(&self) -> TenantAccessTokenRequest {
    TenantAccessTokenRequest::new(self.config.clone())
}
pub fn tenant_access_token_internal(&self) -> TenantAccessTokenInternalRequest {
    TenantAccessTokenInternalRequest::new(self.config.clone())
}
pub fn app_ticket_resend(&self) -> AppTicketResendRequest {
    AppTicketResendRequest::new(self.config.clone())
}
```

- [ ] **Step 4: 同步 auth/mod.rs:13-16 的 5 个 pub use**

将：
```rust
pub use self::auth::v3::auth::{
    AppAccessTokenBuilder, AppAccessTokenInternalBuilder, AppTicketResendBuilder, AuthServiceV3,
    TenantAccessTokenBuilder, TenantAccessTokenInternalRequestBuilder,
};
```
改为：
```rust
pub use self::auth::v3::auth::{
    AppAccessTokenRequest, AppAccessTokenInternalRequest, AppTicketResendRequest, AuthServiceV3,
    TenantAccessTokenRequest, TenantAccessTokenInternalRequest,
};
#[allow(deprecated)]
pub use self::auth::v3::auth::{
    AppAccessTokenBuilder, AppAccessTokenInternalBuilder, AppTicketResendBuilder,
    TenantAccessTokenBuilder, TenantAccessTokenInternalRequestBuilder,
};
```

- [ ] **Step 5: 增量验证（仅 auth crate，快速反馈）**

Run: `cargo build -p openlark-auth`
Expected: exit 0（可能伴随其他未改类型的既有 warning，但无 error）。

Run: `cargo test -p openlark-auth --lib auth::auth::v3 2>&1 | tail -20`
Expected: v3 auth 子模块测试全过。

- [ ] **Step 6: Commit**

```bash
git add crates/openlark-auth/src/auth/auth/v3/auth/ crates/openlark-auth/src/auth/mod.rs
git commit -m "refactor(auth): v3 auth 5 个请求类型 Builder→Request + #[deprecated] alias（#271 pilot 批次 1）"
```

---

## Task 3: authen v1 子系统重命名 + alias（5 个类型，批次 2）

5 个 authen v1 类型：UserAccessTokenV1 / RefreshUserAccessTokenV1 / OidcAccessToken / OidcRefreshAccessToken / UserInfo。牵动 `AuthenServiceV1`（2 方法）+ `OidcService`（2 方法）+ 3 层 re-export。

**Files:**
- Modify: `crates/openlark-auth/src/auth/authen/v1/access_token/create.rs`
- Modify: `crates/openlark-auth/src/auth/authen/v1/refresh_access_token/create.rs`
- Modify: `crates/openlark-auth/src/auth/authen/v1/oidc/access_token/create.rs`
- Modify: `crates/openlark-auth/src/auth/authen/v1/oidc/refresh_access_token/create.rs`
- Modify: `crates/openlark-auth/src/auth/authen/v1/user_info/get.rs`
- Modify: `crates/openlark-auth/src/auth/authen/v1/access_token/mod.rs:8`
- Modify: `crates/openlark-auth/src/auth/authen/v1/refresh_access_token/mod.rs:8`
- Modify: `crates/openlark-auth/src/auth/authen/v1/oidc/mod.rs:9-10,28-29,33-34`（pub use + OidcService 2 方法）
- Modify: `crates/openlark-auth/src/auth/authen/v1/user_info/mod.rs:8`
- Modify: `crates/openlark-auth/src/auth/authen/v1/mod.rs:16-19,44-45,49-50`（pub use + AuthenServiceV1 2 方法）
- Modify: `crates/openlark-auth/src/auth/authen/mod.rs:12-13`

**Interfaces:**
- Consumes: Task 2 的模式（struct+impl 重命名 + alias）
- Produces: 5 个新 `XxxRequest` + 5 alias；`AuthenServiceV1`/`OidcService` 方法返回新名。

- [ ] **Step 1: 5 个 create.rs/get.rs 各自重命名 struct+impl+文件内测试，并加 alias**

按 Task 2 Step 1-2 的 4 步法逐文件处理：
- `access_token/create.rs`：`UserAccessTokenV1Builder` → `UserAccessTokenV1Request`
- `refresh_access_token/create.rs`：`RefreshUserAccessTokenV1Builder` → `RefreshUserAccessTokenV1Request`
- `oidc/access_token/create.rs`：`OidcAccessTokenBuilder` → `OidcAccessTokenRequest`
- `oidc/refresh_access_token/create.rs`：`OidcRefreshAccessTokenBuilder` → `OidcRefreshAccessTokenRequest`
- `user_info/get.rs`：`UserInfoBuilder` → `UserInfoRequest`

每个文件末尾追加 `#[deprecated(note = "renamed to <新名>, will be removed in v1.0 (#271)")] pub type <旧名> = <新名>;`。

- [ ] **Step 2: 同步 oidc/mod.rs（2 pub use + OidcService 2 方法）**

`pub use` 改为新名 + `#[allow(deprecated)]` 导出旧名：
```rust
pub use access_token::create::OidcAccessTokenRequest;
pub use refresh_access_token::create::OidcRefreshAccessTokenRequest;
#[allow(deprecated)]
pub use access_token::create::OidcAccessTokenBuilder;
#[allow(deprecated)]
pub use refresh_access_token::create::OidcRefreshAccessTokenBuilder;
```
`OidcService` 方法（行 28-29、33-34）：
```rust
pub fn access_token(&self) -> OidcAccessTokenRequest {
    OidcAccessTokenRequest::new(self.config.clone())
}
pub fn refresh_access_token(&self) -> OidcRefreshAccessTokenRequest {
    OidcRefreshAccessTokenRequest::new(self.config.clone())
}
```

- [ ] **Step 3: 同步 authen/v1/mod.rs（4 pub use + AuthenServiceV1 2 方法）**

`pub use`（行 16-19）：
```rust
pub use access_token::UserAccessTokenV1Request;
pub use oidc::{OidcAccessTokenRequest, OidcRefreshAccessTokenRequest, OidcService};
pub use refresh_access_token::RefreshUserAccessTokenV1Request;
pub use user_info::{UserInfoRequest, UserInfoService};
#[allow(deprecated)]
pub use access_token::UserAccessTokenV1Builder;
#[allow(deprecated)]
pub use oidc::{OidcAccessTokenBuilder, OidcRefreshAccessTokenBuilder};
#[allow(deprecated)]
pub use refresh_access_token::RefreshUserAccessTokenV1Builder;
#[allow(deprecated)]
pub use user_info::UserInfoBuilder;
```
`AuthenServiceV1` 方法（行 44-45、49-50）：
```rust
pub fn access_token(&self) -> UserAccessTokenV1Request {
    UserAccessTokenV1Request::new(self.config.clone())
}
pub fn refresh_access_token(&self) -> RefreshUserAccessTokenV1Request {
    RefreshUserAccessTokenV1Request::new(self.config.clone())
}
```

- [ ] **Step 4: 同步 access_token/mod.rs、refresh_access_token/mod.rs、user_info/mod.rs 各 1 个 pub use**

各自把单行 `pub use create::<旧名>;` 改为新名，并补 `#[allow(deprecated)] pub use create::<旧名>;`。例如 `access_token/mod.rs:8`：
```rust
pub use create::UserAccessTokenV1Request;
#[allow(deprecated)]
pub use create::UserAccessTokenV1Builder;
```
其余两个文件同模式（RefreshUserAccessTokenV1Request / UserInfoRequest）。

- [ ] **Step 5: 同步 authen/mod.rs:12-13 顶层 pub use**

将（与既有 AuthenServiceV1/OidcService/UserInfoService 等同行）旧名换新名 + `#[allow(deprecated)]` 加旧名块（参照 Task 2 Step 4 的双块写法）。

- [ ] **Step 6: 增量验证**

Run: `cargo build -p openlark-auth`
Expected: exit 0。

Run: `cargo test -p openlark-auth --lib auth::authen 2>&1 | tail -20`
Expected: authen 子模块测试全过。

- [ ] **Step 7: Commit**

```bash
git add crates/openlark-auth/src/auth/authen/
git commit -m "refactor(auth): authen v1 5 个请求类型 Builder→Request + #[deprecated] alias（#271 pilot 批次 2）"
```

---

## Task 4: oauth + human_authentication + verification 子系统（3 个类型，批次 3）

剩余 3 个类型分散在三个子系统：`AuthorizationBuilder`（oauth，3 层 re-export 链）、`IdentityCreateBuilder`（human_authentication，1 个 re-export）、`VerificationGetBuilder`（verification，**无 re-export**，仅改定义文件）。

**Files:**
- Modify: `crates/openlark-auth/src/auth/oauth/old/default/index.rs`（struct+impl+文件内测试+alias）
- Modify: `crates/openlark-auth/src/auth/oauth/old/default/mod.rs:6`
- Modify: `crates/openlark-auth/src/auth/oauth/old/mod.rs:6`
- Modify: `crates/openlark-auth/src/auth/oauth/mod.rs:7`
- Modify: `crates/openlark-auth/src/human_authentication/human_authentication/v1/identity/create.rs`（struct+impl+测试+alias）
- Modify: `crates/openlark-auth/src/human_authentication/human_authentication/v1/identity/mod.rs:5`
- Modify: `crates/openlark-auth/src/verification_information/verification/v1/verification/get.rs`（struct+impl+测试+alias；**无 re-export 同步**）

**Interfaces:**
- Consumes: Task 2/3 模式
- Produces: 3 个新 `XxxRequest` + 3 alias。

- [ ] **Step 1: AuthorizationBuilder → AuthorizationRequest（定义文件 + alias）**

`oauth/old/default/index.rs`：`pub struct AuthorizationBuilder` → `pub struct AuthorizationRequest`、`impl AuthorizationBuilder` → `impl AuthorizationRequest`、文件内测试与 `AuthorizationUrlBuilder::new(...)` 调用（注意：`AuthorizationUrlBuilder` 是 URL builder，**保持不动**，只改 `AuthorizationBuilder` 自身引用）。文件末尾加：
```rust
#[deprecated(note = "renamed to AuthorizationRequest, will be removed in v1.0 (#271)")]
pub type AuthorizationBuilder = AuthorizationRequest;
```

- [ ] **Step 2: AuthorizationBuilder 3 层 re-export 链同步**

`oauth/old/default/mod.rs:6`、`oauth/old/mod.rs:6`、`oauth/mod.rs:7` 三处均为：
```rust
pub use default::{AuthorizationBuilder, OAuthServiceOld};
```
（或 `pub use old::{...}` / `pub use index::{...}`）。每处改为新名 + `#[allow(deprecated)]` 旧名块：
```rust
pub use default::{AuthorizationRequest, OAuthServiceOld};
#[allow(deprecated)]
pub use default::AuthorizationBuilder;
```
（按各文件实际路径调整 `default`/`old`/`index` 前缀，保持其余 `OAuthServiceOld` 等不动。）

- [ ] **Step 3: IdentityCreateBuilder → IdentityCreateRequest（定义 + alias + 1 re-export）**

`identity/create.rs`：struct+impl+测试重命名，文件末尾加 alias。`identity/mod.rs:5`：
```rust
pub use create::{HumanAuthenticationUserIdType, IdentityCreateRequest, IdentityCreateResponse};
#[allow(deprecated)]
pub use create::IdentityCreateBuilder;
```

- [ ] **Step 4: VerificationGetBuilder → VerificationGetRequest（仅定义 + alias，无 re-export）**

`verification/v1/verification/get.rs`：struct+impl+测试重命名，文件末尾加：
```rust
#[deprecated(note = "renamed to VerificationGetRequest, will be removed in v1.0 (#271)")]
pub type VerificationGetBuilder = VerificationGetRequest;
```
**不**需要改任何 re-export（已确认无）。

- [ ] **Step 5: 增量验证**

Run: `cargo build -p openlark-auth`
Expected: exit 0。

Run: `cargo test -p openlark-auth --lib auth::oauth human_authentication verification_information 2>&1 | tail -20`
Expected: 全过。

- [ ] **Step 6: Commit**

```bash
git add crates/openlark-auth/src/auth/oauth/ crates/openlark-auth/src/human_authentication/ crates/openlark-auth/src/verification_information/
git commit -m "refactor(auth): oauth/human_authentication/verification 3 个请求类型 Builder→Request + #[deprecated] alias（#271 pilot 批次 3）"
```

---

## Task 5: alias deprecation warning 行为测试（新增针对性测试）

为 `#[deprecated]` alias 的核心行为加测试：旧名经 alias 可调用 `new()`（证明源码兼容 + warning），新名无 warning。集中在 `app_access_token.rs` 既有 `mod tests`（代表性类型 `AppAccessTokenBuilder`/`AppAccessTokenRequest`），避免新增文件。

**Files:**
- Modify: `crates/openlark-auth/src/auth/auth/v3/auth/app_access_token.rs`（既有 `#[cfg(test)] mod tests`，追加测试）

**Interfaces:**
- Consumes: Task 2 产出的 `AppAccessTokenRequest` + `AppAccessTokenBuilder` alias
- Produces: 2 个新测试函数，覆盖 alias 行为。

- [ ] **Step 1: 追加旧名 alias 可调用测试（`#[allow(deprecated)]` 抑制 warning）**

在 `app_access_token.rs` 的 `mod tests` 内追加：
```rust
#[test]
#[allow(deprecated)]
fn test_app_access_token_legacy_alias_still_callable() {
    // 旧名 alias 经 deprecated type alias 解析到新类型的方法，
    // 必须仍可调用（源码兼容），仅在编译期产生 deprecation warning。
    let config = create_test_config();
    let builder = AppAccessTokenBuilder::new(config)
        .app_id("legacy")
        .app_secret("legacy_secret")
        .app_ticket("legacy_ticket");
    assert_eq!(builder.app_id, "legacy");
    assert_eq!(builder.app_secret, "legacy_secret");
    assert_eq!(builder.app_ticket, "legacy_ticket");
}

#[test]
fn test_app_access_token_new_name_no_deprecation() {
    // 新名正常调用，无 deprecation warning。
    let config = create_test_config();
    let builder = AppAccessTokenRequest::new(config)
        .app_id("new")
        .app_secret("new_secret")
        .app_ticket("new_ticket");
    assert_eq!(builder.app_id, "new");
    assert_eq!(builder.app_secret, "new_secret");
    assert_eq!(builder.app_ticket, "new_ticket");
}
```

- [ ] **Step 2: 跑测试确认通过**

Run: `cargo test -p openlark-auth --lib app_access_token 2>&1 | tail -20`
Expected: 全过（含两个新测试）。整个 `cargo test -p openlark-auth` 不应因 alias 产生 failed。

- [ ] **Step 3: Commit**

```bash
git add crates/openlark-auth/src/auth/auth/v3/auth/app_access_token.rs
git commit -m "test(auth): 覆盖 AppAccessTokenBuilder alias deprecation 行为（#271 pilot）"
```

---

## Task 6: CHANGELOG v0.18 Breaking Changes 记录

**Files:**
- Modify: `CHANGELOG.md`（`## [Unreleased]` → `### Breaking Changes` 段，与既有 v0.18 breaking 条目并列）

**Interfaces:**
- Consumes: 13 个重命名映射
- Produces: 1 条 CHANGELOG 条目。

- [ ] **Step 1: 在 `### Breaking Changes` 段顶部追加（v0.18 软 breaking，alias 兼容）**

```markdown
- **auth 请求类型统一 `Request` 后缀**（#271 pilot，软 breaking）：openlark-auth 的
  13 个请求类型 `XxxBuilder` 重命名为规范 `XxxRequest`，旧名作 `#[deprecated]` type
  alias 保留至 v1.0（调用方使用旧名仍可编译，仅产生 deprecation warning）。迁移：
  `XxxBuilder` → `XxxRequest`（含 `AppAccessToken/AppAccessTokenInternal/AppTicketResend/
  Authorization/IdentityCreate/OidcAccessToken/OidcRefreshAccessToken/RefreshUserAccessTokenV1/
  TenantAccessToken/UserAccessTokenV1/UserInfo/VerificationGet` 各 `Builder` → `Request`；
  特例 `TenantAccessTokenInternalRequestBuilder` → `TenantAccessTokenInternalRequest`，
  去 `Builder` 避免双 `Request`）。`AuthorizationUrlBuilder`（URL builder）不动。v1.0 移除 alias。
```

- [ ] **Step 2: Commit**

```bash
git add CHANGELOG.md
git commit -m "docs(changelog): 记录 auth 请求类型 Builder→Request 软 breaking（#271 pilot）"
```

---

## Task 7: 全量验证（build / clippy×3 / test / alias 实证 / grep）

**Files:** 无修改（验证 only；如发现失败回 Task 对应步骤修，不在此 Task 改）

- [ ] **Step 1: 全 feature 构建**

Run: `cargo build --workspace --all-features`
Expected: exit 0。

- [ ] **Step 2: 三组 feature clippy（default / all-features / no-default-features，均 -D warnings）**

Run:
```bash
cargo clippy --workspace --all-targets -D warnings
cargo clippy --workspace --all-targets --all-features -D warnings
cargo clippy --workspace --all-targets --no-default-features -D warnings
```
Expected: 三组均 exit 0。
> 若 `--no-default-features` 因其他 crate 既有问题失败，单独排查是否与本次相关；本次改动不应引入新 clippy 违例。`#[deprecated]` alias 的 `deprecated` lint **不属** clippy `-D warnings` 范围，不应因此失败；re-export deprecated item 的 lint 已用 `#[allow(deprecated)]` 抑制。

- [ ] **Step 3: auth 测试全过**

Run: `cargo test -p openlark-auth`
Expected: 0 failed。

- [ ] **Step 4: alias deprecation warning 实证（临时代码，验证后删除）**

在 `/tmp` 建临时 binary 验证（不污染仓库），或临时在 `app_access_token.rs` 顶部加一行 `let _ = AppAccessTokenBuilder::new;` 触发 warning：
Run: `cargo build -p openlark-auth 2>&1 | rg "AppAccessTokenBuilder.*deprecated"`
Expected: 命中 1 行 `use of deprecated type alias ... AppAccessTokenBuilder ... renamed to AppAccessTokenRequest`。
**验证后立即还原**（删掉临时行，`git diff` 确认无残留）。

- [ ] **Step 5: grep 确认改名完整性 + AuthorizationUrlBuilder 未动**

Run:
```bash
# 13 个新 struct 存在
rg -n "^pub struct (AppAccessTokenRequest|AppAccessTokenInternalRequest|AppTicketResendRequest|AuthorizationRequest|IdentityCreateRequest|OidcAccessTokenRequest|OidcRefreshAccessTokenRequest|RefreshUserAccessTokenV1Request|TenantAccessTokenRequest|TenantAccessTokenInternalRequest|UserAccessTokenV1Request|UserInfoRequest|VerificationGetRequest)\b" crates/openlark-auth/src/ | wc -l
```
Expected: `13`。

Run:
```bash
# 13 个 #[deprecated] type alias 存在
rg -B0 "^#\[deprecated\(note = \"renamed to .*Request, will be removed in v1\.0 \(#271\)\"\)\]\s*$" crates/openlark-auth/src/ | wc -l
rg "^pub type (AppAccessTokenBuilder|AppAccessTokenInternalBuilder|AppTicketResendBuilder|AuthorizationBuilder|IdentityCreateBuilder|OidcAccessTokenBuilder|OidcRefreshAccessTokenBuilder|RefreshUserAccessTokenV1Builder|TenantAccessTokenBuilder|TenantAccessTokenInternalRequestBuilder|UserAccessTokenV1Builder|UserInfoBuilder|VerificationGetBuilder) =" crates/openlark-auth/src/ | wc -l
```
Expected: 两命令均 `13`。

Run:
```bash
# AuthorizationUrlBuilder 仍是 struct（未被动）
rg -n "^pub struct AuthorizationUrlBuilder" crates/openlark-auth/src/models/oauth/mod.rs
```
Expected: 1 行命中。

Run:
```bash
# auth src 内不应再有作为 struct 定义的裸旧名（alias 是 pub type，不是 struct）
rg -n "^pub struct (AppAccessTokenBuilder|AppAccessTokenInternalBuilder|AppTicketResendBuilder|AuthorizationBuilder|IdentityCreateBuilder|OidcAccessTokenBuilder|OidcRefreshAccessTokenBuilder|RefreshUserAccessTokenV1Builder|TenantAccessTokenBuilder|TenantAccessTokenInternalRequestBuilder|UserAccessTokenV1Builder|UserInfoBuilder|VerificationGetBuilder)\b" crates/openlark-auth/src/
```
Expected: 空输出（13 个旧名已全部从 struct 定义退化为 type alias）。

- [ ] **Step 6: 最终全量校验通过则标记本计划完成**

若 Step 1-5 全过，本 change 实施完成，进入 verify 阶段（comet-state next）。若任一失败，回对应 Task 用 superpowers:systematic-debugging 定位根因再修，不在此 Task 内盲改。

---

## 自检（Self-Review 结果）

- **Spec 覆盖**：spec 三条 Requirement（统一 Request 后缀 / 旧名 deprecated alias / 不破坏 build/clippy/test）→ Task 2-4（重命名）+ Task 2-4 alias 行 + Task 5（warning 测试）+ Task 7（build/clippy/test 验证）。5 个 Scenario 全有对应步骤（13 struct 存在→Task 7 Step 5；旧名不再作 struct→Task 7 Step 5；AuthorizationUrlBuilder 不动→Task 1 Step 2 + Task 7 Step 5；alias deprecated→Task 7 Step 5；旧名 warning→Task 7 Step 4；新名无 warning→Task 5/7；build/clippy/test 三关→Task 7 Step 1-3）。无遗漏。
- **占位符扫描**：无 TBD/TODO；每个改动步骤都给了确切文件路径、行号、新旧代码块。
- **类型一致性**：13 个新名在映射表、各 Task、CHANGELOG、grep 命令中完全一致（含特例 `TenantAccessTokenInternalRequest`）。re-export 双块写法（新名 + `#[allow(deprecated)]` 旧名）在 Task 2/3/4 统一。
