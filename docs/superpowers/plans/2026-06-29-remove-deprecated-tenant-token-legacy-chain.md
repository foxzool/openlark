---
change: remove-deprecated-tenant-token-legacy-chain
design-doc: docs/superpowers/specs/2026-06-29-remove-deprecated-tenant-token-legacy-chain-design.md
base-ref: db6d9ed704cfaa28bc52f26f66944e3ae2f75c8b
archived-with: 2026-06-29-remove-deprecated-tenant-token-legacy-chain
---

# 移除 tenant_access_token deprecated legacy 链 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 移除 `TenantAccessTokenBuilder` 的 3 个 deprecated 方法（`app_id`/`app_secret`/`app_ticket`）及其驱动的 legacy 两步换取逻辑、字段、结构体、import、依赖测试，简化 `execute_with_options` 为始终使用调用方传入的 `app_access_token`。

**Architecture:** 单文件 contained 改动（`crates/openlark-auth/src/auth/auth/v3/auth/tenant_access_token.rs`）。canonical 流程（`new(config).app_access_token(..).tenant_key(..).execute()`）的网络行为与请求体不变；legacy 路径整条删除。这是 v0.18 全仓 `#[deprecated]` 清零的最后一批。

**Tech Stack:** Rust（openlark-auth crate）、`validate_required!` 宏（借用语义允许验证后 move）、wiremock（canonical 测试）、serde。

## Global Constraints

- **单文件改动**：所有源码改动限于 `crates/openlark-auth/src/auth/auth/v3/auth/tenant_access_token.rs`。CHANGELOG 是唯一另一处落盘改动（文档）。
- **不破坏 canonical 行为**：canonical `app_access_token+tenant_key` 单步请求、请求体 `{app_access_token, tenant_key}`、`AccessTokenType::None` 不变。
- **不在库代码用 `unwrap()`/`expect()`**（沿用项目反模式约束）。
- **不硬编码 URL**：保留 `AuthApiV3::TenantAccessToken.path()` 端点生成。
- **`validate_required!` move 合法性已实证**：宏源码 `if is_empty_trimmed(&$field)` 是借用，验证结束后 move 字段合法（design 阶段对抗验证 8/8 通过，含 `cargo build -p openlark-auth` 成功）。
- **commit 粒度**：源码改动（Task 1-3）是原子提交单元——编译只在三件事都完成后才通过，不可拆分提交。Task 4（验证）不产生 commit。Task 5（CHANGELOG）独立提交。

archived-with: 2026-06-29-remove-deprecated-tenant-token-legacy-chain
---

## File Structure

| 文件 | 责任 | 本次改动 |
|------|------|---------|
| `crates/openlark-auth/src/auth/auth/v3/auth/tenant_access_token.rs` | 商店应用获取 tenant_access_token 的 Builder + 请求体 + execute | 删 legacy 方法/字段/结构体/import、简化 execute、清理测试 |
| `CHANGELOG.md` | 版本变更日志 | `[Unreleased] > ### Breaking Changes` 追加 1 条 |

源文件改动后预期结构（按文件内顺序）：

1. 模块 doc 注释（保留）
2. `use crate::models::auth::TenantAccessTokenResponse;`（保留；删 line 3 的 `use super::app_access_token::AppAccessTokenResponseData;`）
3. `use openlark_core::{...}`（保留，含 `validate_required`）
4. `use serde::{Deserialize, Serialize};`（保留）
5. `struct TenantAccessTokenBody { app_access_token, tenant_key }`（保留）
6. ~~`struct LegacyAppAccessTokenBody`~~（删）
7. `pub struct TenantAccessTokenBuilder { app_access_token, tenant_key, config }`（删 3 个 legacy 字段）
8. `TenantAccessTokenResponseData` + `impl ApiResponseTrait`（保留）
9. `impl TenantAccessTokenBuilder`：`new` / `app_access_token` / `tenant_key` / `execute` / `execute_with_options`（删 3 个 deprecated 方法；简化 execute）
10. `#[cfg(test)] mod tests`：保留 6 个 canonical 测试，删 1 个 legacy 测试，调整 `builder_new` 断言

archived-with: 2026-06-29-remove-deprecated-tenant-token-legacy-chain
---

## Task 1: 删除 legacy 方法、字段、结构体与 import，并简化 execute

> 单文件、原子提交。三件事必须在同一 commit——编译只在 import/字段/方法/结构体/execute/测试全部对齐后才通过。TDD 反向：本任务是"删除"，无新功能，canonical 正向测试 `test_execute_sends_app_token_tenant_key_and_no_authorization`（已存在，line 247-283）即是验收测试，删除 legacy 后它必须仍通过。

**Files:**
- Modify: `crates/openlark-auth/src/auth/auth/v3/auth/tenant_access_token.rs`

**Interfaces:**
- Consumes: `validate_required!`（openlark-core 导入，借用语义）、`AuthApiV3::TenantAccessToken`、`Transport::request`、`AccessTokenType::None`
- Produces: 简化后的 `TenantAccessTokenBuilder`（公开 API：`new` / `app_access_token` / `tenant_key` / `execute` / `execute_with_options`），签名不变，行为收敛为单步 canonical

- [x] **Step 1: 确认 base-ref 与工作区干净**

Run:
```bash
git status --porcelain
git rev-parse HEAD
```
Expected: 工作区干净（无输出）；HEAD = `db6d9ed704cfaa28bc52f26f66944e3ae2f75c8b`（base-ref）。若 HEAD 不符，停止并报告。

- [x] **Step 2: 删除 line 3 的 legacy import**

Edit `crates/openlark-auth/src/auth/auth/v3/auth/tenant_access_token.rs`，删除这一行：

```rust
use super::app_access_token::AppAccessTokenResponseData;
```

该类型在 `app_access_token.rs` 自身文件内仍完整使用，仅本文件 import 删除。

- [x] **Step 3: 删除 `LegacyAppAccessTokenBody` 结构体（line 27-32）**

删除整块：

```rust
#[derive(Debug, Serialize)]
struct LegacyAppAccessTokenBody {
    app_id: String,
    app_secret: String,
    app_ticket: String,
}
```

注意保留其上方的 `TenantAccessTokenBody`（line 21-25）与空行规整。

- [x] **Step 4: 删除 struct 的 3 个 legacy 字段（line 38-40）**

将 `TenantAccessTokenBuilder` 结构体定义从：

```rust
pub struct TenantAccessTokenBuilder {
    app_access_token: String,
    tenant_key: String,
    legacy_app_id: String,
    legacy_app_secret: String,
    legacy_app_ticket: String,
    /// 配置信息
    config: Config,
}
```

改为：

```rust
pub struct TenantAccessTokenBuilder {
    app_access_token: String,
    tenant_key: String,
    /// 配置信息
    config: Config,
}
```

- [x] **Step 5: 从 `new()` 移除 legacy 字段初始化（line 65-67）**

将 `new` 从：

```rust
pub fn new(config: Config) -> Self {
    Self {
        app_access_token: String::new(),
        tenant_key: String::new(),
        legacy_app_id: String::new(),
        legacy_app_secret: String::new(),
        legacy_app_ticket: String::new(),
        config,
    }
}
```

改为：

```rust
pub fn new(config: Config) -> Self {
    Self {
        app_access_token: String::new(),
        tenant_key: String::new(),
        config,
    }
}
```

- [x] **Step 6: 删除 3 个 deprecated 方法（line 84-103）**

删除整块（含 `#[deprecated]` 属性与 doc 注释）：

```rust
    /// 旧版 app_id 链式入口，保留用于编译兼容。
    #[deprecated(note = "请改用 app_access_token(...) 并设置 tenant_key(...)")]
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.legacy_app_id = app_id.into();
        self
    }

    /// 旧版 app_secret 链式入口，保留用于编译兼容。
    #[deprecated(note = "请改用 app_access_token(...) 并设置 tenant_key(...)")]
    pub fn app_secret(mut self, app_secret: impl Into<String>) -> Self {
        self.legacy_app_secret = app_secret.into();
        self
    }

    /// 旧版 app_ticket 链式入口，保留用于编译兼容。
    #[deprecated(note = "请先通过 app_ticket 换取 app_access_token，再调用 app_access_token(...)")]
    pub fn app_ticket(mut self, app_ticket: impl Into<String>) -> Self {
        self.legacy_app_ticket = app_ticket.into();
        self
    }
```

保留其前的 `app_access_token`（line 72-76）与 `tenant_key`（line 78-82）两个 canonical builder 方法。

- [x] **Step 7: 简化 `execute_with_options`（line 111-173）**

将整个方法体替换为 design §3.3 定稿形态：

```rust
    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<TenantAccessTokenResponseData> {
        validate_required!(self.app_access_token, "应用访问凭证不能为空");
        validate_required!(self.tenant_key, "租户标识不能为空");

        use crate::common::api_endpoints::AuthApiV3;
        let api_endpoint = AuthApiV3::TenantAccessToken;

        let request_body = TenantAccessTokenBody {
            app_access_token: self.app_access_token,
            tenant_key: self.tenant_key,
        };

        let api_request: ApiRequest<TenantAccessTokenResponseData> =
            ApiRequest::post(api_endpoint.path())
                .body(serde_json::to_value(&request_body)?)
                .with_supported_access_token_types(vec![AccessTokenType::None]);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取商店应用 tenant_access_token", "响应数据为空")
        })
    }
```

关键变化：
- 删除 `let app_access_token = if self.app_access_token.is_empty() { ... legacy 两步 ... } else { self.app_access_token.clone() };` 整块（原 line 121-151）。
- 新增 `validate_required!(self.app_access_token, "应用访问凭证不能为空");`。
- `request_body` 直接 move `self.app_access_token` 与 `self.tenant_key`（去 `.clone()`）——`validate_required!` 是借用，验证结束 move 合法。
- canonical POST `/auth/v3/tenant_access_token` body `{app_access_token, tenant_key}` 与 `AccessTokenType::None` 不变。

`execute`（line 106-108）不动。

- [x] **Step 8: 删除 legacy 测试 `test_execute_legacy_chain_fetches_app_token_then_tenant_token`（line 285-344）**

删除整个测试函数，含其 `#[allow(deprecated)]` 与 `#[tokio::test]` 属性：

```rust
    #[allow(deprecated)]
    #[tokio::test]
    async fn test_execute_legacy_chain_fetches_app_token_then_tenant_token() {
        // ... 整个函数体 ...
    }
```

这是 3 个 deprecated 方法的唯一调用点，删除后 deprecated 即无引用。

- [x] **Step 9: 调整 `test_tenant_access_token_builder_new`（line 194-203）**

删除 3 行 legacy 字段断言：

```rust
        assert!(builder.legacy_app_id.is_empty());
        assert!(builder.legacy_app_secret.is_empty());
        assert!(builder.legacy_app_ticket.is_empty());
```

保留前两行：

```rust
    #[test]
    fn test_tenant_access_token_builder_new() {
        let config = create_test_config();
        let builder = TenantAccessTokenBuilder::new(config);
        assert!(builder.app_access_token.is_empty());
        assert!(builder.tenant_key.is_empty());
    }
```

canonical 正向测试 `test_execute_sends_app_token_tenant_key_and_no_authorization`（line 247-283）不动——它是简化后 execute 的验收测试。

- [x] **Step 10: 编译验证 openlark-auth crate**

Run:
```bash
cargo build -p openlark-auth
```
Expected: 编译成功（exit 0）。若失败，常见原因：漏删某个 legacy 字段引用 / import 未删干净 / `new()` 残留 legacy 初始化。用 `systematic-debugging` skill 定位。

- [x] **Step 11: 运行 canonical 正向测试**

Run:
```bash
cargo test -p openlark-auth test_execute_sends_app_token_tenant_key_and_no_authorization
```
Expected: PASS。这是 simplification 后的验收信号——canonical 单步请求行为不变。

- [x] **Step 12: 运行整个 crate 测试**

Run:
```bash
cargo test -p openlark-auth
```
Expected: 全部通过（0 failed）。`builder_new` 调整后断言也应通过。

- [x] **Step 13: 格式化**

Run:
```bash
cargo fmt --package openlark-auth
```
Expected: 无 diff 或仅空白调整。若有 diff 说明前面编辑留了不规则缩进。

- [x] **Step 14: 提交源码改动**

Run:
```bash
git add crates/openlark-auth/src/auth/auth/v3/auth/tenant_access_token.rs
git commit -m "refactor(auth)!: 移除 tenant_access_token deprecated legacy 链

删除 TenantAccessTokenBuilder 的 3 个 deprecated 方法（app_id/app_secret/app_ticket）
及其驱动的 legacy 两步换取逻辑、legacy 字段、LegacyAppAccessTokenBody 结构体、
AppAccessTokenResponseData import、依赖测试。execute 简化为始终使用调用方传入的
app_access_token。v0.18 全仓 #[deprecated] 清零收尾。

BREAKING: 旧 app_id/app_secret/app_ticket 链式入口不再可用。迁移：先
AppAccessTokenBuilder 取 app_access_token，再
TenantAccessTokenBuilder::new(config).app_access_token(..).tenant_key(..)。

Refs #278"
```
Expected: commit 成功。注意 `!` 标记 breaking change（conventional commits）。

archived-with: 2026-06-29-remove-deprecated-tenant-token-legacy-chain
---

## Task 2: 全仓验证（grep 清零 + 三组 clippy + workspace 测试）

> 不产生 commit。本任务是对 spec 所有 scenario 的证据采集，对应 `spec.md` 的全部 6 个 Scenario。任一失败须用 `systematic-debugging` skill 根因定位后再修。

**Files:**
- 无文件改动（只读验证）

**Interfaces:**
- Consumes: Task 1 的提交
- Produces: 验证证据（命令输出），供 verify 阶段与 archive 引用

- [x] **Step 1: 全仓 `#[deprecated]` 清零**

Run:
```bash
grep -rn '#\[deprecated' crates/ --include='*.rs'
```
Expected: **无输出**（命中数 0）。对应 spec Scenario "全仓 deprecated 清零"。若仍有命中，说明本 change 之外存在遗留——报告但不本 change 处理（scope 仅 tenant_access_token.rs；design §2 已确认本文件是最后一批）。

- [x] **Step 2: 文件内 legacy 残留检查**

Run:
```bash
grep -n 'LegacyAppAccessTokenBody\|#\[deprecated\|legacy_app_' crates/openlark-auth/src/auth/auth/v3/auth/tenant_access_token.rs
```
Expected: **无输出**。对应 spec Scenario "legacy deprecated 方法移除" + "legacy 两步链移除"。

- [x] **Step 3: canonical 流程保留检查**

Run:
```bash
grep -c 'pub fn app_access_token\|pub fn tenant_key' crates/openlark-auth/src/auth/auth/v3/auth/tenant_access_token.rs
```
Expected: `2`（两个 canonical builder 方法均在）。对应 spec Scenario "canonical 流程保留"。

- [x] **Step 4: 三组 feature clippy（default）**

Run:
```bash
cargo clippy --workspace --all-targets -- -Dwarnings -A missing_docs
```
Expected: exit 0（无 warning，无 error）。`-Dwarnings` 把所有 warning 升级为 deny；`-A missing_docs` 允许缺文档（项目既有约定）。

- [x] **Step 5: 三组 feature clippy（all-features）**

Run:
```bash
cargo clippy --workspace --all-targets --all-features -- -Dwarnings -A missing_docs
```
Expected: exit 0。

- [x] **Step 6: 三组 feature clippy（no-default-features）**

Run:
```bash
cargo clippy --workspace --all-targets --no-default-features -- -Dwarnings -A missing_docs
```
Expected: exit 0。对应 spec Scenario "三组 feature clippy 通过"（三组全 exit 0）。

- [x] **Step 7: workspace 全量测试**

Run:
```bash
cargo test --workspace
```
Expected: 全部通过（0 failed）。对应 spec Scenario "tests 通过" + "canonical 流程行为不变"。重点确认 `openlark-auth` 测试全绿、无其他 crate 因本次改动失败。

- [x] **Step 8: 记录验证结论**

在 `openspec/changes/remove-deprecated-tenant-token-legacy-chain/` 下补 verify 证据（按 comet verify 阶段约定；若项目用 `.comet.yaml` 的 verify_result 字段则更新它，否则记入 tasks.md 复核区）。本步骤不写源码，仅落盘验证记录。

archived-with: 2026-06-29-remove-deprecated-tenant-token-legacy-chain
---

## Task 3: CHANGELOG 追加 Breaking Changes 条目

> 独立提交。镜像 v0.18 house style（中文、整标签加粗、反引号、全角括号、内联 `→`、`关联 #<issue>` 尾注）。design §4 已定稿文案。

**Files:**
- Modify: `CHANGELOG.md`（在 `## [Unreleased]` > `### Breaking Changes` 之下，`im::im` 条目之后追加）

**Interfaces:**
- Consumes: design §4 定稿文案
- Produces: CHANGELOG 新条目（issue #278 关联）

- [x] **Step 1: 定位插入点**

Run:
```bash
grep -n '## \[Unreleased\]\|### Breaking Changes\|im::im 嵌套别名' CHANGELOG.md | head
```
Expected: 看到形如：
```
8:## [Unreleased]
20:### Breaking Changes
26:- **Removed deprecated im::im 嵌套别名**：...
27:  `im::im::v1` / `im::im::v2` → `im::v1` / `im::v2`。关联 #278（F）。
```
插入点：line 27（im::im 条目结束）之后、当前 line 28 空行处起一个新空行 + 新条目。注意 im::im 条目之后还有 docs/hr 等其他 breaking 条目——新条目应紧跟 im::im 之后（保持 issue 编号相近的条目聚集），即插入在第 27 行与第 29 行（docs 条目）之间。

- [x] **Step 2: 追加条目**

在 `im::im` 条目（line 26-27）与 `docs deprecated 方法` 条目（line 29）之间，插入一个空行 + 新条目。最终形态（中间一段）：

```markdown
- **Removed deprecated im::im 嵌套别名**：移除 `im::im` 旧嵌套路径别名（deprecated since 0.15.0）→ 迁移
  `im::im::v1` / `im::im::v2` → `im::v1` / `im::v2`。关联 #278（F）。

- **Removed deprecated tenant_access_token legacy 链**：移除 `TenantAccessTokenBuilder`
  的 `app_id`/`app_secret`/`app_ticket` 旧链式入口及两步换取逻辑（deprecated legacy
  chain）→ 迁移：先 `AppAccessTokenBuilder` 取 `app_access_token`，再
  `TenantAccessTokenBuilder::new(config).app_access_token(..).tenant_key(..)`。关联 #278。

- **Removed docs deprecated 方法**：移除 `RecordFieldValue::to_value()`（deprecated since 0.15.0，→ 直接用 `RecordFieldValue` 类型）与 `impl_required_builder!` 宏生成的 `new()`（deprecated since 0.5.0，→ 用 `builder()`）。均零调用/dead。关联 #278（D+C 子集）。
```

文案逐字采用 design §4，不得改动（包括全角括号、反引号、`→`、`关联 #278。`）。

- [x] **Step 3: 确认条目落位与格式**

Run:
```bash
grep -n 'tenant_access_token legacy 链' CHANGELOG.md
```
Expected: 恰好 1 行命中，且其上下文为 `### Breaking Changes` 段内、im::im 条目之后。

- [x] **Step 4: 提交 CHANGELOG**

Run:
```bash
git add CHANGELOG.md
git commit -m "docs(changelog): 记录 tenant_access_token deprecated legacy 链移除

Refs #278"
```
Expected: commit 成功。

archived-with: 2026-06-29-remove-deprecated-tenant-token-legacy-chain
---

## Self-Review

**1. Spec coverage**（逐 Scenario 核对）：

- ✅ "legacy deprecated 方法移除"（grep `#[deprecated` = 0 in file）→ Task 1 Step 6（删方法）+ Task 2 Step 2（验证）。
- ✅ "legacy 两步链移除"（grep `LegacyAppAccessTokenBody` = 0）→ Task 1 Step 3（删结构体）+ Step 7（删 execute 两步分支）+ Task 2 Step 2（验证）。
- ✅ "canonical 流程保留"（grep `pub fn app_access_token`/`pub fn tenant_key` = 2）→ Task 1 Step 6（保留两方法）+ Task 2 Step 3（验证）。
- ✅ "全仓 deprecated 清零"（grep `#[deprecated` in `crates/` = 0）→ Task 2 Step 1。
- ✅ "canonical 流程行为不变"（`cargo test -p openlark-auth test_execute_sends_app_token_tenant_key_and_no_authorization` PASS）→ Task 1 Step 11 + Task 2 Step 7。
- ✅ "三组 feature clippy 通过" → Task 2 Step 4-6。
- ✅ "tests 通过"（`cargo test --workspace` 0 failed）→ Task 2 Step 7。
- ✅ tasks.md 1.1-5.1 全覆盖：1.1/1.2/1.3/2.1 → Task 1；3.1/3.2 → Task 1 Step 8/9；4.1/4.2/4.3 → Task 2；5.1 → Task 3。

**2. Placeholder scan**：无 TBD/TODO；所有代码块均为完整可粘贴内容；所有命令含 expected 输出。

**3. Type/命名一致性**：

- `TenantAccessTokenBuilder` 全程一致。
- canonical 方法名 `app_access_token` / `tenant_key`（snake_case）在 Step 6/9/Task 2 Step 3 一致。
- `validate_required!` 在 Step 7 与 Global Constraints 中描述一致（借用语义）。
- 请求体字段 `app_access_token` / `tenant_key` 在 Step 7 与 spec Scenario "canonical 流程行为不变" 一致。
- CHANGELOG `关联 #278` 与 im::im 条目（`关联 #278（F）`）编号一致；本条目不带子标号（design §4 原文）。

**4. 编译原子性**：Task 1 Step 2-9 在 Step 10 编译验证前可能处于不编译状态（删了字段但 execute 还引用等）——这是预期的，因为 Step 10 才是第一个编译检查点。任务说明已在开头标注"原子提交单元"。
