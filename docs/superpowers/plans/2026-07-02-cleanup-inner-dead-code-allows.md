---
change: cleanup-inner-dead-code-allows
design-doc: docs/superpowers/specs/2026-07-02-cleanup-inner-dead-code-allows-design.md
base-ref: 8aedb2de3ae1d9bb3309e717d3689cf2e5023020
---

# cleanup-inner-dead-code-allows 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: 使用 `superpowers:subagent-driven-development`（推荐）或 `superpowers:executing-plans` 逐任务实施本计划。步骤用 checkbox（`- [ ]`）跟踪。

> **事实源**：OpenSpec delta spec `dead-code-lint-hygiene`（`openspec/changes/cleanup-inner-dead-code-allows/specs/dead-code-lint-hygiene/spec.md`）+ tasks.md 8 组任务。技术设计与探查修正（D2/D3）见 Design Doc：`docs/superpowers/specs/2026-07-02-cleanup-inner-dead-code-allows-design.md`。

**Goal：** 移除 7 处 `#![allow(dead_code)]` 及其掩盖的 104 处死代码，清零 CI 脚本 `KNOWN_INNER_DEBT` 人为开口，使 dead_code lint 信号彻底恢复。

**Architecture：** 纯删除型清理——按 crate 分块逐项删除死代码（hr 废弃 endpoints 模块 / core observability + query_params + header_builder 死项 / mail 6 孤儿字段 / bot+docs stale allow），连带移除仅服务已删代码的 Cargo feature（`tracing-init`/`otel`）与 5 个依赖（`testing` feature 解耦保留）；同步更新 CI 脚本与 msrv lockfile。不新增 crate、不接线访问器、不改 public API 行为。

**Tech Stack：** Rust（cargo + clippy），Bash（CI 守卫脚本 `tools/check_no_dead_code_allows.sh`），GitHub Actions（CI lint/msrv job）。

## Global Constraints

- **基线 commit**：`8aedb2de3ae1d9bb3309e717d3689cf2e5023020`（base-ref）。所有改动在此基础上叠加。
- **不写新单测**：死代码清理 change，删除项均 0 引用、无可测行为。验证靠 clippy/build/CI 守卫矩阵，**不设计 TDD 红绿循环**。
- **macOS 环境**（BSD sed/grep）：批量编辑用 Read+Edit 精确替换，避免 sed -i 平台差异。
- **不提交 git**（implementer 视角）：每个 task 末尾的 commit 由主会话（coordinator）执行；implementer subagent 只做代码改动。
- **spec 验收标准**（delta spec）：
  - 三组 feature clippy（default / `--all-features` / `--no-default-features`）+ `--all-targets` 均 0 dead_code 警告、0 `#![allow(dead_code)]` 残留。
  - `tools/check_no_dead_code_allows.sh` 的 `KNOWN_INNER_DEBT` 清空、脚本 PASS。
  - `cargo test --workspace` 全绿、`cargo build` 双 feature 组绿。
- **删依赖必同步 msrv lockfile**：`.github/msrv/Cargo.lock` 必须在 Task 8 同步，否则 CI msrv `--locked` fail（本地复现不出的已知坑——见 MEMORY）。
- **fmt 必须显式 check**：CI lint 第一步是 `cargo fmt --check`（#270/#280 重蹈覆辙），clippy 通过 ≠ fmt 通过。
- **Cargo feature 收窄**：仅移除 `tracing-init`/`otel`；`testing` feature **保留并解耦**为 `testing = []`（去掉 `["tracing-init"]`），因 `pub mod testing` 被 hr/docs 测试大量使用。
- **`tracing` 本体保留**：不在删除清单（`tracing::Span`/`span!` 在他处用）。删除的 5 依赖是 `opentelemetry`/`opentelemetry_sdk`/`opentelemetry-otlp`/`tracing-opentelemetry` + `[dev-dependencies]` 的 `tracing-subscriber`。

## 关键技术发现（写计划时已实测，必读）

这些事实是计划编写阶段实测的结果，**修正了 design 阶段若干表述**。执行时务必采用本节精确位置。

1. **`header_builder.rs` 的 `add_headers`（复数）有 3 个 `#[cfg(test)]` 测试引用**：
   - 函数本体：`crates/openlark-core/src/request_builder/header_builder.rs:48-56`（`pub fn add_headers`）。
   - 测试：同文件 `#[cfg(test)] mod tests` 内的 `test_add_headers_empty_list`（L207）、`test_add_headers_multiple`（L218）、`test_add_headers_duplicate_keys`（L233）。
   - **删除 `add_headers` 函数必须连同删 3 个测试**，否则测试模块编译失败（`cannot find function add_headers`）。
   - 活函数：`build_headers`（L16）、`add_header`（L43，单数）——`request_builder/mod.rs:46,48` 在用，**保留**。

2. **`tracing-subscriber` 在 core Cargo.toml 出现两次**：
   - `[dependencies]` L29：`tracing-subscriber = { workspace = true, optional = true }`（被 `tracing-init` feature 拉）。
   - `[dev-dependencies]` L83：`tracing-subscriber = { workspace = true }`（仅 observability 用）。
   - **两处都删**（design D2 明确「从 `[dev-dependencies]` 移除 `tracing-subscriber`」；`[dependencies]` 的 optional 版本因 `tracing-init` feature 删除而失去目标，一并删）。

3. **hr `endpoints/` 目录仅含 `mod.rs`**（已查证）：`crates/openlark-hr/src/endpoints/` 下只有 `mod.rs` 一个文件。删目录 = 删该文件 + 目录。
   - `lib.rs:69`（实测）声明：`#[allow(deprecated)]`（L68）+ `mod endpoints;`（L69），上方注释在 L67「// 端点保留（已废弃，请使用 common::api_endpoints 中的枚举系统）」。
   - 3 行（L67-69）整段删，含注释 + `#[allow(deprecated)]` + `mod endpoints;`。

4. **mail 孤儿字段精确路径**（6 文件，每文件含 1 字段 + 1 `new()` 初始化）：
   - `crates/openlark-mail/src/mail/mail/v1/user_mailbox/alias/delete.rs` — `delete_id`
   - `crates/openlark-mail/src/mail/mail/v1/user_mailbox/folder/patch.rs` — `patch_id`
   - `crates/openlark-mail/src/mail/mail/v1/user_mailbox/folder/delete.rs` — `delete_id`
   - `crates/openlark-mail/src/mail/mail/v1/user_mailbox/rule/delete.rs` — `delete_id`
   - `crates/openlark-mail/src/mail/mail/v1/user_mailbox/mail_contact/patch.rs` — `patch_id`
   - `crates/openlark-mail/src/mail/mail/v1/user_mailbox/mail_contact/delete.rs` — `delete_id`

5. **mail `User.config` 位置**：`crates/openlark-mail/src/mail/mail/v1/user/mod.rs:8-10`（`pub struct User { config: Arc<Config>, }`，字段在 L9）。MSRV 1.88 支持 `#[expect(dead_code)]`，写法是字段上方注解 + 中文注释。

6. **CI 脚本 `KNOWN_INNER_DEBT` heredoc 结构**：`tools/check_no_dead_code_allows.sh:8-17`（`cat <<'EOF' ... EOF`），含 7 个文件路径。尾部 echo 文案在 L34。清空 = 删 heredoc 内 7 行 + 改 echo 文案 + 删 grep 排除（`-vFf` 那行 L25，因清单空可整段简化）。

7. **CHANGELOG v0.18 Breaking Changes 区**：`CHANGELOG.md:29` `### Breaking Changes`。新条目追加到该区现有条目之后（按现有格式 `- **标题**（说明）：...`）。

## File Structure

| 文件 | 改动类型 | 职责 |
|------|----------|------|
| `crates/openlark-core/src/observability.rs` | 整文件删 | D2: 删 observability 脚手架（0 引用） |
| `crates/openlark-core/src/lib.rs:17` | Edit（删 1 行） | D2: 删 `pub(crate) mod observability;` |
| `crates/openlark-core/src/lib.rs:18` | Edit（删 1 行） | D3: 删 `pub(crate) mod query_params;` |
| `crates/openlark-core/src/query_params.rs` | 整文件删 | D3: 删 1085 行死代码（2 死 struct，0 use） |
| `crates/openlark-core/src/request_builder/header_builder.rs` | Edit（删函数+测试+allow） | D3: 删 `add_headers` 函数 + 3 测试 + 文件顶 `#![allow]]` |
| `crates/openlark-core/Cargo.toml` | Edit（features + deps） | D2: 删 `tracing-init`/`otel` feature、`testing = []`、4 optional dep + 表 + dev-dep `tracing-subscriber` |
| `Cargo.toml` (根) | Edit（workspace deps） | D2: 同步删 5 个 workspace 依赖 |
| `crates/openlark-hr/src/endpoints/` | 整目录删 | D1: 删废弃模块（仅含 mod.rs） |
| `crates/openlark-hr/src/lib.rs:67-69` | Edit（删 3 行） | D1: 删注释 + `#[allow(deprecated)]` + `mod endpoints;` |
| `crates/openlark-mail/src/mail/mail/v1/user_mailbox/{alias/folder/rule/mail_contact}/*.rs` | Edit（删字段+初始化） | D4: 删 6 孤儿字段 |
| `crates/openlark-mail/src/mail/mail/v1/user/mod.rs:9` | Edit（加注解） | D4: `User.config` 加 `#[expect(dead_code)]` + 注释 |
| `crates/openlark-mail/src/lib.rs:1` | Edit（删 1 行） | D4: 删 stale `#![allow]]` |
| `crates/openlark-bot/src/lib.rs:1` | Edit（删 1 行） | D5: 删 stale `#![allow]]` |
| `crates/openlark-docs/src/ccm/explorer/explorer/mod.rs:1` | Edit（删 1 行） | D5: 删 stale `#![allow]]` |
| `tools/check_no_dead_code_allows.sh` | Edit（清空 heredoc + 文案） | D6: CI 脚本收口 |
| `.github/msrv/Cargo.lock` | 重生成 | Task 8: 删依赖必同步 |
| `CHANGELOG.md` | Edit（追加） | Task 8: v0.18 breaking 记录 |

---

## Task 1: 前置安全核查（D2 风险缓解）

**Files:**
- 验证（无文件改动）：workspace 内 crate 的 Cargo.toml + 测试代码

**Interfaces:**
- 无（纯核查 task，为后续 task 锁定前提）

**目的**：在删 Cargo feature / 依赖前，复核 workspace 内无 crate 直接启用 `tracing-init`/`otel`，无测试代码引用这些 feature 的符号。探查阶段已证，本 task 在执行环境再跑一遍 grep 留证据。

- [ ] **Step 1: 复核无 crate 启用 tracing-init/otel**

Run:
```bash
grep -rn 'openlark-core' crates/ --include=Cargo.toml | grep -E 'tracing-init|otel'
grep -rnE 'features\s*=\s*\[.*"(tracing-init|otel)"' crates/ --include=Cargo.toml
```
Expected: **两条命令均无输出**（exit 0 但空）。若有输出，停止——意味着某 crate 显式启用，需在删 feature 前先处理。

- [ ] **Step 2: 复核 hr/docs 仅启用 testing（保留的 feature）**

Run:
```bash
grep -rnE 'features\s*=\s*\[.*"testing"' crates/ --include=Cargo.toml
```
Expected: 输出 `crates/openlark-hr/...` 与 `crates/openlark-docs/...` 的 `features = ["testing"]` 行（证明 testing 被使用、须保留）。

- [ ] **Step 3: 复核无测试代码引用 otel/tracing-init feature 符号**

Run:
```bash
grep -rnE 'cfg\(feature\s*=\s*"(tracing-init|otel)"\)' crates/ src/
```
Expected: 无输出（探查已证）。

- [ ] **Step 4: 记录核查结论，进入 Task 2**

无需 commit（本 task 无代码改动）。在执行日志记录「grep 全空，前提成立」。

---

## Task 2: openlark-core 删 observability + Cargo feature/依赖（D2）

**Files:**
- Delete: `crates/openlark-core/src/observability.rs`（整文件）
- Modify: `crates/openlark-core/src/lib.rs:17`
- Modify: `crates/openlark-core/Cargo.toml`（features + 4 optional dep + dev-dep + 4 个 `[dependencies.X]` 表）
- Modify: `Cargo.toml`（根，`[workspace.dependencies]` 5 项）

**Interfaces:**
- Consumes: Task 1 核查结论（无 crate 启用 tracing-init/otel）
- Produces: `openlark-core` 失去 `tracing-init`/`otel` feature；`testing` 解耦为 `testing = []`；`observability` 模块消失；5 个依赖从 workspace 移除。

- [ ] **Step 1: 删 `observability.rs` 全文**

```bash
rm crates/openlark-core/src/observability.rs
```

- [ ] **Step 2: 删 lib.rs 的 `pub(crate) mod observability;`（L17）**

Edit `crates/openlark-core/src/lib.rs`，删除该行（保留 `pub(crate) mod query_params;` 等其他行——query_params 在 Task 3 处理）。

- [ ] **Step 3: 改 `crates/openlark-core/Cargo.toml` 的 `[features]` 段**

删除：
- `# Tracing initialization support` 注释 + `tracing-init = ["tracing-subscriber"]` 行
- `# OpenTelemetry support` 注释 + `otel = ["tracing-init", "opentelemetry", "opentelemetry_sdk", "opentelemetry-otlp", "tracing-opentelemetry"]` 行

改：
- `# Testing utilities (for other crates to use in tests)` 注释下的 `testing = ["tracing-init"]` → `testing = []`

保留：`default = []`、`websocket = [...]`。

- [ ] **Step 4: 删 4 个 `[dependencies]` 行 + 4 个 `[dependencies.X]` 表**

从 `[dependencies]` 段删：`tracing-subscriber = { workspace = true, optional = true }`（L29）。

删 4 个独立表（连带其前的空行保持文件整洁）：
```
[dependencies.opentelemetry]
workspace = true
optional = true

[dependencies.opentelemetry_sdk]
workspace = true
optional = true

[dependencies.opentelemetry-otlp]
workspace = true
optional = true

[dependencies.tracing-opentelemetry]
workspace = true
optional = true
```

注意：`opentelemetry`/`opentelemetry_sdk`/`opentelemetry-otlp`/`tracing-opentelemetry` 仅在 `[dependencies.X]` 表声明（未在主 `[dependencies]` 列），故仅删这 4 个表 + 主表里的 `tracing-subscriber` 行。

- [ ] **Step 5: 删 `[dev-dependencies]` 的 `tracing-subscriber`**

从 `[dev-dependencies]` 段删：`tracing-subscriber = { workspace = true }`（L83）。

保留：`rstest`、`tracing-test`、`wiremock`（测试基础设施，非 observability 专属）。

- [ ] **Step 6: 根 `Cargo.toml` 同步删 5 个 workspace 依赖**

Edit `Cargo.toml` 根 `[workspace.dependencies]` 段，删除这 5 行（约 L107-111）：
```
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
tracing-opentelemetry = { version = "0.25" }
opentelemetry = { version = "0.24" }
opentelemetry_sdk = { version = "0.24", features = ["rt-tokio"] }
opentelemetry-otlp = { version = "0.17" }
```

保留：`tracing = ...`（本体，他处用）。

- [ ] **Step 7: 验证三组 feature 编译**

Run:
```bash
cargo check -p openlark-core
cargo check -p openlark-core --all-features
cargo check -p openlark-core --no-default-features
```
Expected: 三组均编译通过、0 dead_code 警告。若 `--all-features` 报「unknown feature `otel`/`tracing-init`」→ Step 3 删漏；若报 `cannot find crate 'opentelemetry'` → Step 4/5/6 残留引用。

- [ ] **Step 8: commit**

由 coordinator 执行：
```bash
git add -A
git commit -m "refactor(core): 删 observability + 移除 tracing-init/otel feature 及 5 依赖 (#277)

- 删 crates/openlark-core/src/observability.rs + lib.rs mod 声明
- Cargo.toml: 删 tracing-init/otel feature; testing 解耦为 testing=[]
- 删 4 optional dep (opentelemetry*4) + [dev-dep] tracing-subscriber
- 根 [workspace.dependencies] 同步删 5 项
- tracing 本体与 pub mod testing 保留

part of cleanup-inner-dead-code-allows (#277)"
```

---

## Task 3: openlark-core 删 query_params + header_builder 死项（D3）

**Files:**
- Delete: `crates/openlark-core/src/query_params.rs`（整文件，1085 行）
- Modify: `crates/openlark-core/src/lib.rs:18`
- Modify: `crates/openlark-core/src/request_builder/header_builder.rs`（删 `add_headers` 函数 + 3 测试 + 文件顶 `#![allow]]`）

**Interfaces:**
- Consumes: Task 2 完成（core 编译干净）
- Produces: `query_params` 模块消失；`HeaderBuilder::add_headers`（复数）消失；`build_headers`/`add_header`（单数）保留。

- [ ] **Step 1: 删 `query_params.rs` 整文件**

```bash
rm crates/openlark-core/src/query_params.rs
```

- [ ] **Step 2: 删 lib.rs 的 `pub(crate) mod query_params;`（L18）**

Edit `crates/openlark-core/src/lib.rs`，删除该行。

- [ ] **Step 3: 删 `header_builder.rs:1` 的 `#![allow(dead_code)]`**

Edit `crates/openlark-core/src/request_builder/header_builder.rs`，删除第 1 行 `#![allow(dead_code)]`。

- [ ] **Step 4: 删 `add_headers` 函数（L47-56）**

Edit 同文件，删除整个函数（含其上方 `/// 批量添加请求头（工具方法）` doc 注释）：
```rust
    /// 批量添加请求头（工具方法）
    pub fn add_headers(
        mut req_builder: RequestBuilder,
        headers: &[(String, String)],
    ) -> RequestBuilder {
        for (key, value) in headers {
            req_builder = req_builder.header(key, value);
        }
        req_builder
    }
```

注意：保留 `build_headers`（L16）与 `add_header`（L43，单数）。删除后 `impl HeaderBuilder { ... }` 块以 `add_header` 函数的 `}` 结尾。

- [ ] **Step 5: 删 3 个 `add_headers` 测试用例**

Edit 同文件的 `#[cfg(test)] mod tests`，删除这 3 个 `#[test]` 函数：
- `test_add_headers_empty_list`（约 L206-215）
- `test_add_headers_multiple`（约 L217-230）
- `test_add_headers_duplicate_keys`（约 L232-245）

注意：每个测试上方可能有空行分隔，删除时保持测试模块内空行风格一致。保留 `test_add_header*`（单数）测试与 `test_build_headers_*` 测试。

- [ ] **Step 6: 验证 core 三组编译 + 测试通过**

Run:
```bash
cargo check -p openlark-core
cargo check -p openlark-core --all-features
cargo check -p openlark-core --no-default-features
cargo test -p openlark-core
```
Expected: 三组 check 编译通过、0 dead_code 警告；`cargo test` 全绿（`add_headers` 测试已删，不应有失败；其余 header_builder 测试不受影响）。

- [ ] **Step 7: commit**

由 coordinator 执行：
```bash
git add -A
git commit -m "refactor(core): 删 query_params 模块 + header_builder::add_headers 死项 (#277)

- 删 crates/openlark-core/src/query_params.rs 整文件 (1085 行, 2 死 struct, 0 use) + mod 声明
- header_builder.rs: 删 add_headers 函数 + 3 个 add_headers 测试 + 文件顶 #![allow(dead_code)]
- 保留 build_headers / add_header (活于 request_builder/mod.rs:46,48)

part of cleanup-inner-dead-code-allows (#277)"
```

---

## Task 4: openlark-hr 删废弃 endpoints 模块（D1）

**Files:**
- Delete: `crates/openlark-hr/src/endpoints/`（整目录，仅含 `mod.rs`）
- Modify: `crates/openlark-hr/src/lib.rs:67-69`

**Interfaces:**
- Consumes: 无（hr 与 core 改动独立）
- Produces: `openlark-hr::endpoints` 模块消失（私有模块，无 public API 影响）

- [ ] **Step 1: 删 `endpoints/` 目录**

```bash
rm -rf crates/openlark-hr/src/endpoints
```

- [ ] **Step 2: 删 lib.rs 的注释 + allow + mod 声明（L67-69）**

Edit `crates/openlark-hr/src/lib.rs`，删除这 3 行（含上方空行保持整洁）：
```rust
// 端点保留（已废弃，请使用 common::api_endpoints 中的枚举系统）
#[allow(deprecated)]
mod endpoints;
```

- [ ] **Step 3: 验证 hr 编译 + testing feature 仍工作**

Run:
```bash
cargo check -p openlark-hr
cargo check -p openlark-hr --features testing
cargo test -p openlark-hr
```
Expected: 编译通过、0 dead_code 警告；测试全绿（hr 测试用 `openlark_core::testing::prelude::TestConfigBuilder`，testing feature 在 Task 2 已解耦保留）。

- [ ] **Step 4: commit**

由 coordinator 执行：
```bash
git add -A
git commit -m "refactor(hr): 删废弃 endpoints 模块 (84 死常量, 0 引用) (#277)

- 删 crates/openlark-hr/src/endpoints/ 目录 (仅含 mod.rs)
- lib.rs: 删 mod endpoints; + #[allow(deprecated)] + 废弃注释
- 替代系统 common/api_endpoints 已是 source of truth

part of cleanup-inner-dead-code-allows (#277)"
```

---

## Task 5: openlark-mail 删孤儿字段 + User.config 显式处理（D4）

**Files:**
- Modify: 6 个 `crates/openlark-mail/src/mail/mail/v1/user_mailbox/{alias/folder/rule/mail_contact}/{delete,patch}.rs`（删字段 + `new()` 初始化）
- Modify: `crates/openlark-mail/src/mail/mail/v1/user/mod.rs:9`（`User.config` 加 `#[expect]]` + 注释）
- Modify: `crates/openlark-mail/src/lib.rs:1`（删 `#![allow]]`）

**Interfaces:**
- Consumes: 无
- Produces: 6 个 `Request` struct 不再有 `delete_id`/`patch_id` 字段；`User` struct 的 `config` 字段带显式 `#[expect(dead_code)]`。

- [ ] **Step 1: 删 6 处孤儿字段 + 初始化**

逐个 Edit 这 6 个文件，删除 `delete_id` 或 `patch_id` 字段声明 + struct 字面量初始化：
- `.../user_mailbox/alias/delete.rs` — `delete_id`
- `.../user_mailbox/folder/patch.rs` — `patch_id`
- `.../user_mailbox/folder/delete.rs` — `delete_id`
- `.../user_mailbox/rule/delete.rs` — `delete_id`
- `.../user_mailbox/mail_contact/patch.rs` — `patch_id`
- `.../user_mailbox/mail_contact/delete.rs` — `delete_id`

每个文件改动：删字段定义行（如 `delete_id: String,`）+ `new()`/构造器里的 `delete_id: ...,` 初始化行。私有字段，路径实际用 `user_mailbox_id` + `alias_id`/`folder_id`/`rule_id`/`contact_id`。

- [ ] **Step 2: `User.config` 加 `#[expect(dead_code)]` + 注释**

Edit `crates/openlark-mail/src/mail/mail/v1/user/mod.rs`，将：
```rust
pub struct User {
    config: Arc<Config>,
}
```
改为：
```rust
pub struct User {
    /// 导航 struct，accessor 待补（见 #274/#275 范式），本 change 不接线。
    #[expect(dead_code)]
    config: Arc<Config>,
}
```

注意：字段 doc 注释放在 `#[expect]]` 上方（或合并为一段）；MSRV 1.88 支持 `#[expect]]`。`User::new(config)` 保留（pub navigation API，`MailV1::user()` 接入服务树）。

- [ ] **Step 3: 删 `crates/openlark-mail/src/lib.rs:1` 的 `#![allow]]`**

Edit `crates/openlark-mail/src/lib.rs`，删除第 1 行 `#![allow(dead_code)]`。保留 `#![allow(clippy::module_inception)]`（L2，非本 change 范围）。

- [ ] **Step 4: 验证 mail 编译 0 dead_code**

Run:
```bash
cargo check -p openlark-mail
cargo clippy -p openlark-mail --all-targets
```
Expected: 编译通过、0 dead_code 警告（`User.config` 被 `#[expect]]` 显式标注，不触发警告；6 孤儿字段已删）。若有剩余 dead_code → Step 1 漏字段。

- [ ] **Step 5: commit**

由 coordinator 执行：
```bash
git add -A
git commit -m "refactor(mail): 删 6 孤儿字段 + User.config 显式标注 (#277)

- 删 alias/folder/rule/mail_contact 的 delete+patch 的 delete_id/patch_id 私有字段
  + 各 new() 初始化 (路径实际用 user_mailbox_id + 各资源 id)
- User.config 加 #[expect(dead_code)] + 注释指向 #274/#275 accessor 范式
- 删 lib.rs:1 #![allow(dead_code)]
- 本 change 不接线 User::query() accessor (另案 feature 工作)

part of cleanup-inner-dead-code-allows (#277)"
```

---

## Task 6: openlark-bot / openlark-docs 删 stale allow（D5）

**Files:**
- Modify: `crates/openlark-bot/src/lib.rs:1`
- Modify: `crates/openlark-docs/src/ccm/explorer/explorer/mod.rs:1`

**Interfaces:**
- Consumes: 无
- Produces: 两处 `#![allow(dead_code)]` 消失（掩盖 0 处死代码，纯 stale 残留）。

- [ ] **Step 1: 删 bot lib.rs:1 的 `#![allow]]`**

Edit `crates/openlark-bot/src/lib.rs`，删除第 1 行 `#![allow(dead_code)]`。保留 `#![allow(clippy::module_inception)]`（L2）。

- [ ] **Step 2: 删 docs explorer mod.rs:1 的 `#![allow]]`**

Edit `crates/openlark-docs/src/ccm/explorer/explorer/mod.rs`，删除第 1 行 `#![allow(dead_code)]`。保留 `#![allow(unused_variables)]`（L2）与 `#![allow(unused_imports)]`（L3）——非本 change 范围。

- [ ] **Step 3: 验证两 crate 编译 0 dead_code**

Run:
```bash
cargo check -p openlark-bot
cargo check -p openlark-docs
```
Expected: 两 crate 编译通过、0 dead_code 警告（探查已证两处 allow 掩盖 0 处死代码，删除后不应冒新警告）。

- [ ] **Step 4: commit**

由 coordinator 执行：
```bash
git add -A
git commit -m "refactor(bot,docs): 删 stale #![allow(dead_code)] (掩盖 0 处) (#277)

- crates/openlark-bot/src/lib.rs:1
- crates/openlark-docs/src/ccm/explorer/explorer/mod.rs:1
- 纯防御性残留, 删除后 0 dead_code 警告

part of cleanup-inner-dead-code-allows (#277)"
```

---

## Task 7: CI 脚本收口（D6）

**Files:**
- Modify: `tools/check_no_dead_code_allows.sh`（清空 `KNOWN_INNER_DEBT` heredoc + 更新文案 + 简化 grep 排除）

**Interfaces:**
- Consumes: Task 2-6 完成（7 处 inner allow 全删）
- Produces: CI 脚本对 inner-attribute 一视同仁、无例外清单。

- [ ] **Step 1: 清空 `KNOWN_INNER_DEBT` heredoc（L6-17）**

Edit `tools/check_no_dead_code_allows.sh`。删除这两段：
- L6-7 的注释（`# 已知的 inner-attribute ... 移除本排除项。`）
- L8-17 的 heredoc 赋值（`KNOWN_INNER_DEBT=$(cat <<'EOF' ... EOF )`）

- [ ] **Step 2: 简化 grep 排除（L22-26）**

将原 grep（含 `grep -vFf <(printf '%s\n' "$KNOWN_INNER_DEBT")`）改为：
```bash
hits=$(grep -rn --include='*.rs' -E '^[[:space:]]*#!?\[allow\(dead_code\)\][[:space:]]*$' crates/ src/ \
  | grep -v '/tests/' \
  | grep -v '#\[cfg(test)\]' \
  || true)
```
即删掉 `grep -vFf` 那行（清单已空，不再需要例外排除）。

- [ ] **Step 3: 更新脚本头注释 + 尾部 echo 文案**

L2 注释更新为（保持头部说明准确性）：
```bash
# 检查非测试代码中没有 #[allow(dead_code)] / #![allow(dead_code)]（issue #267 / #277 防复发）
```

L34 echo 改为：
```bash
echo "✅ 无非测试 #[allow(dead_code)] / #![allow(dead_code)] 残留"
```
（去掉「#277 的 7 处 inner-attribute 已登记为例外」字样）。

- [ ] **Step 4: 验证脚本 PASS**

Run:
```bash
bash tools/check_no_dead_code_allows.sh
```
Expected: exit 0，输出 `✅ 无非测试 #[allow(dead_code)] / #![allow(dead_code)] 残留`。若 exit 1 → 说明 workspace 内仍有 inner allow 漏删，回查 Task 2-6。

- [ ] **Step 5: commit**

由 coordinator 执行：
```bash
git add -A
git commit -m "chore(ci): 清空 KNOWN_INNER_DEBT, inner-attribute 死代码守卫一视同仁 (#277)

- tools/check_no_dead_code_allows.sh: 删 KNOWN_INNER_DEBT heredoc (7 文件) + grep 例外排除
- 脚本头/尾文案更新: #277 inner-attribute 收尾完成
- 7 处 #![allow(dead_code)] 全清 (Task 2-6), CI 无人为开口

part of cleanup-inner-dead-code-allows (#277)"
```

---

## Task 8: 全量验证 + msrv lockfile + CHANGELOG

**Files:**
- Regenerate: `.github/msrv/Cargo.lock`
- Modify: `CHANGELOG.md`（v0.18 Breaking Changes 区追加）

**Interfaces:**
- Consumes: Task 2-7 全部完成
- Produces: CI 守卫矩阵全绿；msrv lockfile 与新依赖图一致；CHANGELOG 记录 breaking。

**这是收尾 task——所有验证命令在此一次跑齐。**

- [ ] **Step 1: fmt check（CI lint 第一步）**

Run:
```bash
cargo fmt --all -- --check
```
Expected: exit 0、无 diff 输出。若有 diff → `cargo fmt --all` 修复后重跑（避免重蹈 #270/#280 漏 fmt 致 CI lint fail）。

- [ ] **Step 2: clippy 三组 feature × `--all-targets` 0 dead_code**

Run:
```bash
cargo clippy --workspace --all-targets 2>&1 | tee /tmp/clippy-default.log
cargo clippy --workspace --all-targets --all-features 2>&1 | tee /tmp/clippy-all.log
cargo clippy --workspace --all-targets --no-default-features 2>&1 | tee /tmp/clippy-nodef.log
```
Expected: 三组均无 `dead_code` 警告、无 `#[allow(dead_code)]` 残留。grep 复核：
```bash
grep -c 'dead_code' /tmp/clippy-default.log /tmp/clippy-all.log /tmp/clippy-nodef.log
```
Expected: 三个文件均输出 `0`。对应 spec「全 workspace 内外层均无 cruft 残留」+「废弃模块被删除而非抑制」scenario。

- [ ] **Step 3: 全 workspace 测试通过**

Run:
```bash
cargo test --workspace
```
Expected: 全绿。证伪「删 0 引用代码致行为回归」（hr endpoints 0 引用、mail 字段私有、observability 0 引用）。

- [ ] **Step 4: 双 feature 组 build 通过**

Run:
```bash
cargo build --workspace --all-features
cargo build --workspace --no-default-features
```
Expected: 两组均编译通过。证伪 testing 解耦破坏 hr/docs（hr/docs 的 `features = ["testing"]` 仍有效，只是不再拉 tracing-init）。

- [ ] **Step 5: 同步 `.github/msrv/Cargo.lock`（删依赖必做）**

Run:
```bash
cargo generate-lockfile
cp Cargo.lock .github/msrv/Cargo.lock
```
Expected: `.github/msrv/Cargo.lock` 不再含 5 个删除的依赖（`opentelemetry`/`opentelemetry_sdk`/`opentelemetry-otlp`/`tracing-opentelemetry`/`tracing-subscriber`）。否则 CI msrv `--locked` 会 fail（本地复现不出的已知坑——见 MEMORY）。

复核：
```bash
grep -E 'name = "(opentelemetry|opentelemetry_sdk|opentelemetry-otlp|tracing-opentelemetry|tracing-subscriber)"' .github/msrv/Cargo.lock
```
Expected: 无输出（5 依赖已从 lockfile 消失）。

- [ ] **Step 6: 更新 CHANGELOG v0.18 Breaking Changes 区**

Edit `CHANGELOG.md`，在 `### Breaking Changes`（L29）现有条目之后追加：

```markdown
- **openlark-core 移除 `tracing-init` / `otel` feature 及 5 个仅服务它们的依赖**（#277 inner-attribute 收尾）：
  `openlark-core` 的 `tracing-init` 与 `otel` feature 仅门控已删的 `observability.rs`（0 引用死代码），
  移除。连带删 5 个依赖：`opentelemetry`、`opentelemetry_sdk`、`opentelemetry-otlp`、
  `tracing-opentelemetry`、`tracing-subscriber`（根 `[workspace.dependencies]` 同步）。
  `testing` feature **保留**并解耦为 `testing = []`（不再拉 `tracing-init`，因 `pub mod testing`
  被 hr/docs 测试大量使用、自包含、不依赖 observability）。
  **迁移**：若启用过 `tracing-init`/`otel` feature，直接从 `Cargo.toml` 移除即可，无行为变化
  （原 feature 只编译死代码）。`tracing` 本体与其他 feature 不受影响。
```

- [ ] **Step 7: commit msrv lockfile + CHANGELOG**

由 coordinator 执行：
```bash
git add .github/msrv/Cargo.lock CHANGELOG.md
git commit -m "chore: 同步 msrv lockfile + CHANGELOG v0.18 breaking (#277)

- .github/msrv/Cargo.lock: 删 5 个删除依赖 (opentelemetry*4 + tracing-subscriber)
  (删依赖的 change 必须同步 msrv lockfile, 否则 CI msrv --locked fail)
- CHANGELOG: v0.18 breaking 区记录移除 tracing-init/otel feature + 5 依赖 + 迁移指引

part of cleanup-inner-dead-code-allows (#277)"
```

---

## Self-Review

**1. Spec coverage（逐条对照 tasks.md 8 组 + design 6 决策）：**

- tasks.md §1 前置核查 → Task 1 ✓
- tasks.md §2.1-2.6（core observability + feature/dep + msrv）→ Task 2（2.5 msrv 拆到 Task 8 Step 5 因属全量验证）✓
- tasks.md §3.1-3.2（query_params / header_builder）→ Task 3 ✓
- tasks.md §4.1-4.3（hr endpoints）→ Task 4 ✓
- tasks.md §5.1-5.4（mail 字段 + User + allow）→ Task 5 ✓
- tasks.md §6.1-6.2（bot + docs stale allow）→ Task 6 ✓
- tasks.md §7.1-7.2（CI 脚本）→ Task 7 ✓
- tasks.md §8.1-8.5（全量验证 + msrv + CHANGELOG）→ Task 8 ✓
- design D1-D6 全覆盖 ✓
- design D2 探查修正（testing 保留解耦）→ Global Constraints + Task 2 Step 3 ✓
- design D3 粒度（文件级 vs 项级）→ Task 3 ✓
- design §4 验证矩阵（fmt/clippy×3/test/build×2/msrv）→ Task 8 全覆盖 ✓

**2. Placeholder 扫描：** 无 TBD/TODO/"add appropriate X"。每步含具体命令、文件路径、行号、预期输出。

**3. Type/命名一致性：** `add_headers`（复数，删）vs `add_header`（单数，保留）贯穿 Task 3 一致；`delete_id`/`patch_id` 在 Task 5 与「关键技术发现 §4」一致；`tracing-subscriber` 两次出现（`[dependencies]` + `[dev-dependencies]`）在 Task 2 Step 4/5 与「关键技术发现 §2」一致。

**4. 关键修正点（计划比 design 更精确处）：**
- header_builder.rs 的 `add_headers` **必须连同删 3 个测试**（design 未提及测试，实测发现）。
- `tracing-subscriber` 在 core Cargo.toml 出现**两次**（design 仅说 `[dev-dep]`，实测 `[dependencies]` 也有 optional 版本，两处都删）。
- CI 脚本 grep 排除行（`-vFf`）须一并简化（design 仅说清空 heredoc，实测 heredoc 清空后该 grep 失去目标）。
