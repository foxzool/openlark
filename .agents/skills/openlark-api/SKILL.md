---
name: openlark-api
description: OpenLark 项目 API 接口实现规范（速查）。用于添加/重构飞书开放平台 API：确定落盘路径、实现 Body/Response + Builder(Request)、对齐 endpoints 常量/enum、补齐 mod.rs 导出，并明确"调用服务端 API"的方法签名/RequestOption 传递约定。触发关键词：API 接口、API 文件、飞书 API、添加 API、调用服务端 API
argument-hint: "[api-id|path|bizTag]"
allowed-tools: Bash, Read, Grep, Glob, Edit
---

# OpenLark API 接口实现规范（速查）

## 🧭 技能路由指南

**本技能适用场景：**
- 添加/重构单个飞书开放平台 API
- 需要确定 API 落盘路径（bizTag → crate → 文件路径）
- 需要参考代码模板（Body/Response + Builder）
- 需要了解端点规范、RequestOption 约定、Service 链式调用

**其他技能：**
- 项目级规范体检（架构/API/导出/校验一体）→ `Skill(openlark-code-standards)`
- 审查整体设计规范 → `Skill(openlark-design-review)`
- 统一 `validate()` 写法 → `Skill(openlark-validation-style)`

### 关键词触发映射

- 新增 API、重构 API、Builder、Request/Response、mod.rs 导出、RequestOption → `openlark-api`
- 代码规范、规范检查、风格一致性、体检 → `openlark-code-standards`
- 架构设计、public API、收敛方案、feature gating、兼容策略 → `openlark-design-review`
- validate、必填校验、validate_required、空白字符串、校验聚合 → `openlark-validation-style`
- 覆盖率、缺失 API、实现数量、CSV 对比、验证脚本 → `openlark-api-validation`

### 双向跳转规则

- 若实现问题本质是架构范式冲突（Request/Service 边界），转 `openlark-design-review`。
- 若实现前需要先做全仓规范体检，先跑 `openlark-code-standards`。
- 若实现完成后要核验覆盖率与缺失清单，转 `openlark-api-validation`。

---

本文件只保留"可执行的最小流程"，标准示例与 docPath 抓取能力见 `references/` 与 `scripts/`。

## 🔒 核心契约（所有 crate 必须遵守，不可违反）

> 这些是仓库**唯一规范**，违反任何一条都会导致接口不统一/调用失败。完整正确模板见 `references/standard-example.md`。

1. **新代码默认 `config: Config`（owned）；现有 `Arc<Config>` 的 Service/Client 保持现状，勿为统一所有权单独重构**。`openlark_core::Config` 内部已 `Arc<ConfigInner>`，clone 廉价。新 Request/Service 默认用 `config: Config`，构造用 `Config::build()`（直接返回 `Config`，**不要** `.unwrap()`）。但仓库现有 573+ 文件仍用 `Arc<Config>`（如 `openlark-docs` 的 `DocsClient`/`BaseClient`/`CcmClient`，见 `crates/openlark-docs/src/common/chain.rs:611-629`）——这些保持现状，**不为统一所有权单独重构**。两种形态都不持 HTTP client，"走 Transport"是硬约束。

2. **`R`（`ApiRequest<R>` 泛型）是响应 `data` 字段的内容类型，不是包装层**。`Transport::request` 返回 `ApiResponse<R> = {code,msg,data: R,...}`，`resp.data: Option<R>`。
   - 无 schema/透传：`R = serde_json::Value`，`execute` 返回 `SDKResult<serde_json::Value>`。
   - 有 schema：`R` 就是 data 内容的 typed struct，并 `impl ApiResponseTrait { fn data_format() -> ResponseFormat::Data }`。
   - **❌ 禁止**写成 `XxxResponse { data: Option<T> }` 外面再包一层——core 已自动把 `R` 当作 data 内容解析，再包会**双重嵌套**（运行时才暴露，极难发现）。

3. **禁止绕过 `Transport`**：业务 crate（除 `openlark-core`）**不得** `reqwest::Client::new()` 或自建 HTTP client、不得手工塞 `Authorization` 头、不得手工取 token。全部走 `openlark_core::http::Transport::request`。`Service`/`Request` 只持 `Config`，不持任何 HTTP client 字段。

4. **Token 类型必须显式声明**（应用级接口尤其重要）：`ApiRequest` 默认 `supported_access_token_types = [User, Tenant]`。应用级接口（如 acs）**必须** `.with_supported_access_token_types(vec![AccessTokenType::App])`，否则 Transport 解析到 Tenant/User token 被飞书拒绝。判断方法：接口文档要求 `tenant_access_token` 的 → App token。

5. **端点路径用常量/enum**（禁止手写 `"/open-apis/..."` 字符串字面量散落），**必填校验统一用 `validate_required!`/`validate_required_list!`**，**每个 Request 必须提供 `execute_with_options(..., RequestOption)` 并把 option 透传到 `Transport::request(..., Some(option))`**。

## 0. 快速工作流（新增一个 API）

1) **定位 API**：在 `./api_list_export.csv` 拿到 `bizTag`、`meta.Project`、`meta.Version`、`meta.Resource`、`meta.Name`
   - 若有 `docPath`，用脚本抓取请求/响应体定义（见 §4）
2) **选 crate**：根据 bizTag 选择 feature crate（见 §1）
3) **定路径**：`crates/{crate}/src/{bizTag}/{project}/{version}/{resource...}/{name}.rs`
4) **写代码**：`Body/Response` + Builder（`execute/send`）+ 端点常量/enum
   - **必须支持 RequestOption**：用于 `user_access_token` / `tenant_key` / 自定义 header
5) **补导出**：在 `mod.rs` 中 `pub mod ...` / `pub use ...`
6) **补链路**：在约定入口补齐链式调用（默认 `service.rs`，但 `openlark-docs` 例外，见 §2）
7) **验证**：
   - 新增 API 所属 crate feature 已在 `Cargo.toml [features]` 声明；
   - 涉及该 feature 的测试用 `#[cfg(test)]` 模块内 `#![cfg(feature = "...")]`（或模块级 `#[cfg(feature)]`）门控；
   - 若新增 `examples/` 示例，须在 `Cargo.toml [[example]]` 声明 `required-features`；
   - 跑 `just fmt && just lint && just test`，其中 `just lint` 须含 `--all-targets`（覆盖 examples + tests）。

## 1. Feature Crate ↔ bizTag

仓库以 `tools/api_coverage.toml` 作为 **crate→bizTag** 的唯一来源。

```bash
# 查看所有映射
python3 tools/validate_apis.py --list-crates

# 验证特定 crate 的覆盖率
python3 tools/validate_apis.py --crate openlark-docs
```

**反查技巧**：落盘路径以"目标 crate 现有结构"为准，参考 `references/file-layout.md`

## 2. Service 链式调用（实现 + 调用约定）

> 本节提供"如何实现"的技术规范。若需要审查"是否应该统一范式"（Request 自持 Config vs Builder → Service），见 `Skill(openlark-design-review) §1`。

### 2.1 实现侧：service.rs

> **event/webhook 类 P2 模块不进统一 `service.rs` 链路**：这类模块（如长连接回调、事件订阅）的入口形态与普通 CRUD API 不同，放在各自独立入口，**不强行塞进** `client.<biz>.service()...` 链。

目标：让 `openlark-client` 能走 `client.<biz>.service().<project>().<version>()...<api>()`

- 若 crate 已有 `src/service.rs`：在顶层 service 新增 `pub fn {bizTag}(&self) -> ...`
- 若没有：创建 `src/service.rs` 并在 `lib.rs` 中 `pub mod service;`
- `openlark-docs` 特例：为避免 strict API 校验脚本把"链式入口"计为 API 实现文件，链式入口放在 `crates/openlark-docs/src/common/chain.rs`，只做模块级入口与 Config 透传，不为 200+ API 手写方法。

#### ⚠️ Service 层标准模式

> 注：这是 `openlark-docs` crate 的真实写法（用 `Arc<Config>`，见 `crates/openlark-docs/src/common/chain.rs:611-629` 的 `DocsClient`）。
> **核心契约 1 已放宽**：新代码默认 owned（`config: Config`），现有 `Arc<Config>` 的 Service/Client 保持现状、勿为统一所有权单独重构。两种形态都**不持 HTTP client**，这点是硬约束。

**正确示例**（参考 `openlark-docs/src/common/chain.rs`）：

```rust
use std::sync::Arc;
use openlark_core::config::Config;

/// DocClient 只持有 Arc<Config>
#[derive(Debug, Clone)]
pub struct DocClient {
    config: Arc<Config>,
}

impl DocClient {
    pub fn new(config: Config) -> Self {
        Self { config: Arc::new(config) }
    }

    /// 子 Service 只透传 Arc<Config>
    pub fn drive(&self) -> DriveService {
        DriveService::new(self.config.clone())
    }
}

/// Service 层只持有 Arc<Config>，不持有独立 HTTP client
#[derive(Debug, Clone)]
pub struct DriveService {
    config: Arc<Config>,
}

impl DriveService {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn v1(&self) -> DriveV1 {
        DriveV1::new(self.config.clone())
    }
}
```

**❌ 禁止模式**：
- ❌ **`reqwest::Client::new()` 或任何自建 HTTP client**（业务 crate 除 core 外全部禁止，见核心契约 3）
- ❌ 手工塞 `Authorization` 头 / 手工取 token（由 `Transport` 自动注入）
- ❌ Service/Request 持有独立的 HTTP client 字段（只持 `Config`）
- ❌ 使用 `LarkClient` 作为具体类型（它是 trait）
- ❌ 在测试中使用 `.unwrap()` 调用 `Config::build()`（build() 直接返回 Config）

**✅ 正确模式**：
- ✅ Service 只持有 `Arc<Config>`
- ✅ `Config::build()` 直接返回 `Config`，不需要 `.unwrap()`
- ✅ HTTP 传输由 `openlark_core::Transport` 处理

### 2.2 调用侧：RequestOption 约定

**必须提供** `execute_with_options(..., RequestOption)` 或等价签名，并将 option 透传到 `Transport::request(..., Some(option))`

**使用场景**：
- 用户态 API → `user_access_token`
- 商店应用 → `tenant_key` / `app_ticket`
- 链路追踪 → `request_id` / 自定义 header

> ⚠️ 不要只调用 `ApiRequest::request_option(...)`，它仅合并 header，token 推断需要走 Transport

详细示例见 `references/standard-example.md`

## 3. API 模板（以仓库现有风格为准）

> 以下提供两种仓库中真实存在的风格。**实现时优先模仿目标 crate 的现有文件风格**，避免在同一 project/version 内混用多种范式。
>
> 范式一致性审查见 `Skill(openlark-design-review) §1`。

### 3.1 Request / Response

```rust
use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {Name}Body {
    // 字段按官方文档，用 serde rename 对齐
    // 可选：Option<T> + #[serde(skip_serializing_if = "Option::is_none")]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {Name}Response {
    // 字段按官方文档
}
```

### 3.2 Builder + execute/send

> ⚠️ 下面的模板是**骨架**，必须配合 §"🔒 核心契约" 理解。完整正确示例见
> `references/standard-example.md`（以仓库现有风格为准）。

```rust
use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

// R（ApiRequest<R> 泛型）：无 schema 用 serde_json::Value；有 schema 用 typed struct
// （见 references/standard-example.md 的 B 范式 + impl ApiResponseTrait）。
// ❌ 不要在外面再包一层 XxxResponse { data: Option<...> }——会双重嵌套。

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {Name}Body {
    // 字段按官方文档，用 serde rename 对齐
    // 可选：Option<T> + #[serde(skip_serializing_if = "Option::is_none")]
}

pub struct {Name}Request {
    config: Config,            // owned（见核心契约 1）
    // 路径/查询参数（按需）
}

impl {Name}Request {
    pub fn new(config: Config) -> Self { /* ... */ }

    pub async fn execute(self, body: {Name}Body) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        body: {Name}Body,
        option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(body.<必填字段>, "<字段> 不能为空"); // 见核心契约 5
        // 端点必须复用 crate 的 endpoints 常量或 enum（禁止手写 "/open-apis/..."）
        let req: ApiRequest<serde_json::Value> = ApiRequest::post({ENDPOINT_CONST_OR_ENUM})
            .with_supported_access_token_types(vec![AccessTokenType::App]); // 见核心契约 4（按需）
        let resp = Transport::request(req, &self.config, Some(option)).await?; // 见核心契约 3
        // resp.data: Option<serde_json::Value>，R 是 data 内容（见核心契约 2）
        resp.data.ok_or_else(|| openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据"))
    }
}
```

## 4. 提交前检查清单

- [ ] 落盘路径正确（与同模块现有结构一致）
- [ ] Request/Response 字段对齐官方文档（含 `serde(rename)`）
- [ ] **核心契约 1**：`config: Config`（owned），未用 `Arc<Config>`
- [ ] **核心契约 2**：`R` 是响应 data 内容类型，未在外面再包 `XxxResponse{data}`
- [ ] **核心契约 3**：无 `reqwest::Client::new()` / 手工 token，全走 `Transport::request`
- [ ] **核心契约 4**：应用级接口已加 `.with_supported_access_token_types([App])`
- [ ] **核心契约 5**：端点用常量/enum（禁手写 URL）；必填字段用 `validate_required!`
- [ ] `execute_with_options(..., RequestOption)` 已提供并透传到 Transport
- [ ] `mod.rs` 已导出；`service.rs`/链式入口已补
- [ ] 新增 feature 已在 `Cargo.toml [features]` 声明；测试/示例的 `#[cfg(feature)]` 与 `[[example]] required-features` 已补
- [ ] `just fmt && just lint --all-targets && just test` 通过

## 5. docPath 网页读取

```bash
python3 .agents/skills/openlark-api/scripts/fetch_docpath.py "<docPath>" --format md --out /tmp/doc.md
```

## 6. References

- 目录规范与反查：`references/file-layout.md`
- CSV 映射规则：`references/csv-mapping.md`
- 标准示例（照抄结构）：`references/standard-example.md`
