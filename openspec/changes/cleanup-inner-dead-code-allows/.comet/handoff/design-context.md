# Comet Design Handoff

- Change: cleanup-inner-dead-code-allows
- Phase: design
- Mode: compact
- Context hash: b8e7e81fbb03c2e904e652d6b1124f127c78ed2cadb1b3523be28e66bbe4e770

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/cleanup-inner-dead-code-allows/proposal.md

- Source: openspec/changes/cleanup-inner-dead-code-allows/proposal.md
- Lines: 1-34
- SHA256: 8e4e1ad25996af613c73543484b987bb4450dbb8e8eb2c1927dde595560fb111

```md
## Why

#267（`cleanup-dead-code-allows`）清除了 392 处**外层** `#[allow(dead_code)]`，但清理正则 `#[` 不匹配 `#![`，**漏掉 7 处内层 `#![allow(dead_code)]`**（crate/mod 级）。这 7 处目前掩盖 **104 处死代码**，且被 CI 脚本 `tools/check_no_dead_code_allows.sh` 登记为 `KNOWN_INNER_DEBT` 例外——一个**人为开口**，违背 #267 建立的「dead_code lint 信号保持有效」契约。本 change 闭合这个内层属性缺口、清零开口。

实证（临时移除 7 处后跑 `cargo check --workspace --lib`）：104 处死代码 = hr 84 / core 13 / mail 7；其中 `openlark-bot`、`openlark-docs/explorer` 两处 `#![allow]]` **掩盖 0 处**（stale）。关键修正：hr 的 84 处**不是**「codegen 待接线」（issue #277 的原假设），而是**整模块已自声明废弃**（`//! ⚠️ 此模块已废弃，推荐使用 common::api_endpoints`），替代系统已存在，84 常量 0 引用。

来源：issue [#277](https://github.com/foxzool/openlark/issues/277)。

## What Changes

- **删除 7 处 `#![allow(dead_code)]` + 其掩盖的 104 处死代码**，逐项按 #267 范式处理（删 cruft / `_` 前缀 / `#[expect]]`）：
  - **openlark-hr**：删除整个已废弃的 `endpoints/` 模块（84 常量，`mod endpoints;` 私有，0 引用）+ lib.rs 的 `mod endpoints;` 声明。
  - **openlark-core**：删除 `observability.rs`（9 项，`pub(crate)` 私有，0 引用）、`query_params.rs` 死项（3）、`header_builder.rs::add_headers`（1）。
  - **openlark-mail**：删除孤儿 codegen 字段 `delete_id`/`patch_id`（6，私有字段，路径实际用 `user_mailbox_id`+`alias_id`）；处理 `User.config` 死字段（design 阶段定：删字段 / 删整个 `User` struct / `#[expect]]`）。
  - **openlark-bot / openlark-docs(explorer)**：删除 2 处 stale `#![allow]]`（掩盖 0 处，纯删除）。
- **BREAKING（Cargo feature 层，已收窄）**：移除 `openlark-core` 的 `tracing-init` / `otel` 两个 feature（仅门控已删的 `observability.rs`，删码后无目标）；**`testing` feature 保留并解耦**为 `testing = []`——design 探查发现 `pub mod testing`（TestConfigBuilder 等）被 hr/docs 测试大量使用、自包含、不依赖 observability，不能删。连带删仅服务 `tracing-init`/`otel` 的依赖（`opentelemetry`、`opentelemetry_sdk`、`opentelemetry-otlp`、`tracing-opentelemetry`、`tracing-subscriber`）+ 根 `[workspace.dependencies]` 同步删。
- 清空 `tools/check_no_dead_code_allows.sh` 的 `KNOWN_INNER_DEBT` 例外块。

## Capabilities

### New Capabilities
<!-- 无新 capability -->

### Modified Capabilities
- `dead-code-lint-hygiene`: 扩展禁令至**内层 `#![allow(dead_code)]`**（#267 仅覆盖外层 `#[allow]]`）；要求 CI 脚本 `KNOWN_INNER_DEBT` 例外清单为空（不得保留人为开口）；废弃模块 / 0 引用的 `pub(crate)` 脚手架 SHALL 直接删除，不得用 blanket `#![allow]]` 抑制。

## Impact

- **代码**：`openlark-hr`（删 `endpoints/` 模块）、`openlark-core`（删 `observability.rs` + `query_params.rs`/`header_builder.rs` 死项）、`openlark-mail`（删孤儿字段 + 处理 `User.config`）、`openlark-bot`/`openlark-docs`（删 stale allow）。
- **Cargo manifest（BREAKING，已收窄）**：`openlark-core` 移除 `tracing-init`/`otel` 2 feature（`testing` 解耦保留）；删 5 个仅服务它们的依赖 + 根 `[workspace.dependencies]` 同步。下游若显式启用 `otel`/`tracing-init` 会「unknown feature」——但原 feature 只编译死代码、无行为，且无 workspace crate 直接启用这俩（仅 hr/docs 启用 `testing`，保留）。契合 v0.18 breaking 清理波。
- **CI**：`tools/check_no_dead_code_allows.sh` 的 `KNOWN_INNER_DEBT` 清空。
- **API/行为**：无（所有删除项均为 0 引用死代码）；唯一对外影响是 feature manifest。
- **质量**：dead_code lint 信号彻底恢复；issue #277 闭环；`dead-code-lint-hygiene` spec 缺口补齐。
- **非目标**：① 不给 hr 接线新 endpoint 访问器（`endpoints` 是废弃旧物，非「待接线」）；② 不升级到 `#![deny(warnings)]`（#273 已决定不升 deny）；③ 不改任何 public API 行为；④ 不重构 codegen 工具。
```

## openspec/changes/cleanup-inner-dead-code-allows/design.md

- Source: openspec/changes/cleanup-inner-dead-code-allows/design.md
- Lines: 1-69
- SHA256: df56f66dc27f532ce0f2eca93972cc938049a4ecbf8762926f9510f40a4bab10

```md
## Context

#267（`cleanup-dead-code-allows`）建立了 `dead-code-lint-hygiene` capability 并清除 392 处**外层** `#[allow(dead_code)]`，但其清理正则 `#[` 不匹配 `#![`，漏掉 **7 处内层 `#![allow(dead_code)]`**（crate/mod 级）。这 7 处被 CI 脚本 `tools/check_no_dead_code_allows.sh` 登记为 `KNOWN_INNER_DEBT` 例外——一个违背「lint 信号保持有效」契约的人为开口。

本 change 是 #267 的**内层属性收尾篇**。临时移除 7 处后 `cargo check --workspace --lib` 实证：**104 处死代码**（hr 84 / core 13 / mail 7），另 `openlark-bot`、`openlark-docs/explorer` 两处 `#![allow]]` 掩盖 0 处（stale）。

关键事实修正（推翻 issue #277 原假设）：hr 的 84 处**不是**「codegen 待接线」，而是 `endpoints/mod.rs` **整模块已自声明废弃**（`//! ⚠️ 此模块已废弃，推荐使用 common::api_endpoints`），替代系统 `common/api_endpoints.rs` 已存在，84 常量 0 引用，模块声明 `mod endpoints;` 为**私有**。

## Goals / Non-Goals

**Goals:**
- 移除 7 处 `#![allow(dead_code)]` 及其掩盖的 104 处死代码，dead_code lint 信号彻底恢复。
- 清空 CI 脚本 `KNOWN_INNER_DEBT`，消除人为开口。
- 删除承载死代码的废弃模块 / 0 引用脚手架（含其专属 feature 与依赖）。

**Non-Goals:**
- 不给 hr 接线新 endpoint 访问器（`endpoints` 是废弃旧物，非「待接线」；接线属 #274/#275 类 feature 工作）。
- 不给 mail `User` 接线 `query()` 访问器（同上，另案）。
- 不升级到 `#![deny(warnings)]`（#273 已决定不升 deny）。
- 不改任何 public API 行为；不重构 codegen 工具。

## Decisions

### D1. HR `endpoints/` 模块：整模块删除（非接线）
**选择**：删除整个 `crates/openlark-hr/src/endpoints/` 目录 + `lib.rs` 的 `mod endpoints;` 声明 + 相关 `pub use`（若有）。
**理由**：模块 `//!` 自声明废弃、替代系统 `common::api_endpoints` 已落地、84 常量 0 引用、`mod endpoints;` 私有（非 public API，删除断向后兼容零风险）。
**备选（否决）**：逐常量 `#[expect(dead_code)]` —— 保留废弃模块无价值，反而误导后续维护者以为常量在用。

### D2. CORE `observability.rs`：删码 + 移除 tracing-init/otel + 解耦 testing（用户确认 Option A，design 探查修正）
**选择**：删除 `observability.rs` 全文 + `pub(crate) mod observability;`；移除 `openlark-core/Cargo.toml` 的 `tracing-init` / `otel` 两个 feature；**`testing` feature 保留并解耦**为 `testing = []`（去掉 `["tracing-init"]`）；移除 5 个仅服务这些 feature 的依赖（`opentelemetry`、`opentelemetry_sdk`、`opentelemetry-otlp`、`tracing-opentelemetry`、`tracing-subscriber`）+ 根 `[workspace.dependencies]` 同步删。
**理由（实证 + 探查修正）**：5 依赖在 `openlark-core/src/` 排除 `observability.rs` 后 0 引用，workspace 其他 crate 也 0 引用；`tracing-init`/`otel` 仅门控 `observability.rs`。**但 `testing` feature 探查发现门控 `pub mod testing`（TestConfigBuilder 等自包含测试助手），被 hr/docs 测试大量使用、不依赖 observability**——故保留并解耦，非移除。
**保留**：`tracing` 本体（`tracing::Span`/`span!` 等在他处用，非 5 个删除依赖之一）；`pub mod testing` 全保留。
**BREAKING（收窄）**：仅 `tracing-init`/`otel` 移除是 manifest breaking；无 workspace crate 直接启用这俩（仅 hr/docs 启用 `testing`，保留），workspace 零破坏；crates.io 公众理论 breaking 但原 feature 无行为。

### D3. CORE `query_params.rs`（整文件删）/ `header_builder.rs`（项级删）——探查已定粒度
**`query_params.rs` 整文件删**：1085 行，顶层仅 `pub struct QueryParams` + `pub struct QueryParamsBuilder` 两个死 struct（+ 关联常量），0 外部 `use`。删文件 + `pub(crate) mod query_params;` 声明。
**`header_builder.rs` 项级删**：仅删 `add_headers` 函数（+ 文件顶 `#![allow(dead_code)]`）。`HeaderBuilder::build_headers`/`add_header` **活**——`request_builder/mod.rs:46,48` 在用。保留文件其余。
**可见性**：`observability`/`query_params` 均已证 `pub(crate)`，删除无 public API 影响。

### D4. MAIL 孤儿字段：删；`User.config`：`#[expect]]` + 注释
- **`delete_id` / `patch_id`（6 处）**：直接删除字段 + `new()` 中的初始化。私有字段，路径实际用 `user_mailbox_id`+`alias_id`，这些字段从不 set/read，纯 codegen 残留。零行为影响。
- **`User.config`（1 处）**：`#[expect(dead_code)]` + 注释「导航 struct，accessor 待补（见 #274/#275 范式）；本 change 不接线」。
**理由**：`User` 经 `MailV1::user() -> user::User::new(config)` 接入服务树（pub navigation API，不能删），但缺 `query()` 等访问器导致 `config` 未读——与 #267 修正的 platform v1 入口 struct 同型。本 change 范围是清债不接线，故用 `#[expect]]` 显式标注意图（#267 范式的「显式处理」路径）；接线 `query()` 留作另案 feature 工作。

### D5. BOT / DOCS(explorer)：删 stale allow 行
**选择**：直接删除 `openlark-bot/src/lib.rs:1` 与 `openlark-docs/.../explorer/mod.rs:1` 的 `#![allow(dead_code)]` 行。两处掩盖 0 处死代码，纯防御性残留。

### D6. CI 脚本：清空 `KNOWN_INNER_DEBT`
**选择**：编辑 `tools/check_no_dead_code_allows.sh`，将 `KNOWN_INNER_DEBT` heredoc 清空（或改为空），更新脚本注释。7 处全清后脚本对 inner-attribute 一视同仁。

### D7. 处理哲学沿用 #267
逐项按 `删 cruft / _ 前缀 / #[expect]]` 三选一，已在 D1–D4 落实。不引入新 lint 策略（deny 升级被 #273 否决）。

## Risks / Trade-offs

- **[BREAKING feature 移除]** → `openlark-core` 失去 `tracing-init`/`otel`/`testing` feature，下游启用会 unknown feature。**缓解**：CHANGELOG 记录；build 前先 `grep -rn 'openlark-core.*\(testing\|tracing-init\|otel\)'` 确认 workspace 内无其他 crate 启用这些 feature（dev-dependency 也要查）；原 feature 无可观测行为，用户零损失。
- **[query_params.rs / header_builder.rs 粒度不定]** → 可能整文件死或文件内混活项。**缓解**：build 阶段先删死项跑 clippy，按剩余警告定是否删文件；保守先删项。
- **[mail `User.config` 选择 `#[expect]]` 而非接线]** → 留下「导航 struct 无访问器」的半成品。**缓解**：`#[expect]]` 注释显式标注「accessor 待补」并指向 #274/#275 范式；不在本 change 扩大范围。
- **[删 hr `endpoints` 模块丢失端点常量表]** → 常量表是废弃的旧 registry，替代系统 `common::api_endpoints` 已是 source of truth。**缓解**：design 已确认替代系统存在且 0 引用旧表；无信息丢失。

## Migration Plan

- 无运行时迁移（所有删除项均 0 引用死代码）。
- v0.18 CHANGELOG 的 breaking 区追加：`openlark-core` 移除 `tracing-init`/`otel`/`testing` feature 及 5 个仅服务它们的依赖；迁移指引「若启用过这些 feature，直接移除，无行为变化」。
- 回滚策略：纯 git revert（无数据/schema 迁移）。

## Open Questions

- 无重大未决项（范围已与用户锁定）。build 阶段细化 D3 的文件 vs 项粒度、D2 的 workspace 内 feature 启用面核查。
```

## openspec/changes/cleanup-inner-dead-code-allows/tasks.md

- Source: openspec/changes/cleanup-inner-dead-code-allows/tasks.md
- Lines: 1-49
- SHA256: b4951f31afe14d45a33631959854d265ca66e44825b17c82505bea6a96b1a024

```md
## 1. 前置安全核查（D2 风险缓解）

- [ ] 1.1 核查 workspace 内无 crate 直接启用 `openlark-core` 的 `tracing-init`/`otel`（design 探查已证：仅 hr/docs 启用 `testing`，保留；无 crate 启用 tracing-init/otel）。`grep -rn 'openlark-core' crates/ --include=Cargo.toml` 复核。
- [ ] 1.2 核查无测试/示例引用已删的 `otel`/`tracing-init` feature 符号（design 探查已证无）；`pub mod testing`（被 hr/docs 大量用）保留。

## 2. openlark-core：删 observability + 移除 feature/依赖（D2）

- [ ] 2.1 删除 `crates/openlark-core/src/observability.rs` 全文 + `lib.rs` 的 `pub(crate) mod observability;` 声明。
- [ ] 2.2 从 `crates/openlark-core/Cargo.toml` 移除 `tracing-init`/`otel` feature（含注释）；**`testing` 解耦为 `testing = []`（保留，去掉 `["tracing-init"]`）**。
- [ ] 2.3 从 `[dependencies]` 移除 4 个 optional 依赖及其 `[dependencies.X]` 表：`opentelemetry`、`opentelemetry_sdk`、`opentelemetry-otlp`、`tracing-opentelemetry`；从 `[dev-dependencies]` 移除 `tracing-subscriber`；逐个 `grep` 核实无他处引用（保留 `tracing` 本体）。根 `Cargo.toml` `[workspace.dependencies]` 同步删这 5 项。
- [ ] 2.4 确认 `pub mod testing` 完整保留、hr/docs 的 `features = ["testing"]` 仍工作（解耦后 testing 不再拉 tracing-init）。
- [ ] 2.5 同步更新 `.github/msrv/Cargo.lock`（删依赖的 change 必须同步，否则 CI msrv `--locked` 失败）。
- [ ] 2.6 `cargo check -p openlark-core`（default / `--all-features` / `--no-default-features`）三组均编译通过。

## 3. openlark-core：删 query_params / header_builder 死项（D3）

- [ ] 3.1 **整文件删** `query_params.rs`（1085 行，顶层仅 QueryParams/QueryParamsBuilder 两死 struct，0 外部 use）+ `lib.rs` 的 `pub(crate) mod query_params;` 声明。
- [ ] 3.2 **项级删** `request_builder/header_builder.rs`：仅删 `add_headers` 函数 + 文件顶 `#![allow(dead_code)]`；保留 `HeaderBuilder`/`build_headers`/`add_header`（活于 `request_builder/mod.rs:46,48`）。

## 4. openlark-hr：删废弃 endpoints 模块（D1）

- [ ] 4.1 删除 `crates/openlark-hr/src/endpoints/` 整个目录。
- [ ] 4.2 移除 `lib.rs:69` 的 `mod endpoints;` 声明，及 `lib.rs:67` 附近「端点保留（已废弃…）」注释。
- [ ] 4.3 `cargo check -p openlark-hr` 确认无悬空引用、0 dead_code 警告。

## 5. openlark-mail：删孤儿字段 + User.config 显式处理（D4）

- [ ] 5.1 删除 6 处孤儿字段 `delete_id` / `patch_id`（alias、folder、mail_contact、rule 的 delete+patch）+ 各 `new()` 中的对应初始化。
- [ ] 5.2 `mail/v1/user/mod.rs` 的 `User.config` 字段加 `#[expect(dead_code)]` + 注释「导航 struct，accessor 待补（见 #274/#275 范式），本 change 不接线」。
- [ ] 5.3 移除 `crates/openlark-mail/src/lib.rs:1` 的 `#![allow(dead_code)]`。
- [ ] 5.4 `cargo check -p openlark-mail` 确认 0 dead_code 警告。

## 6. openlark-bot / openlark-docs：删 stale allow（D5）

- [ ] 6.1 移除 `crates/openlark-bot/src/lib.rs:1` 的 `#![allow(dead_code)]`。
- [ ] 6.2 移除 `crates/openlark-docs/src/ccm/explorer/explorer/mod.rs:1` 的 `#![allow(dead_code)]`。

## 7. CI 脚本收口（D6）

- [ ] 7.1 编辑 `tools/check_no_dead_code_allows.sh`：清空 `KNOWN_INNER_DEBT` heredoc 内容并更新脚本尾部文案（inner-attribute 不再享受豁免）。
- [ ] 7.2 运行 `bash tools/check_no_dead_code_allows.sh` 确认 PASS。

## 8. 全量验证（spec 验收场景）

- [ ] 8.1 `cargo fmt --check`（CI lint 第一步，避免重蹈 #270/#280 漏 fmt 致 lint fail）。
- [ ] 8.2 `cargo clippy --workspace --all-targets` 三组——default / `--all-features` / `--no-default-features`——均 0 dead_code 警告、0 `#![allow(dead_code)]` 残留（对应 spec「全 workspace 内外层均无 cruft 残留」+「废弃模块被删除而非抑制」scenario）。
- [ ] 8.3 `cargo test --workspace` 全绿（删除项均 0 引用，无行为回归）。
- [ ] 8.4 `cargo build --workspace --all-features` 与 `--no-default-features` 均通过（feature 移除后矩阵仍绿）。
- [ ] 8.5 更新 CHANGELOG v0.18 breaking 区：记录移除的 3 feature + 5 依赖，附迁移指引「若启用过这些 feature，直接移除，无行为变化」。
```

## openspec/changes/cleanup-inner-dead-code-allows/specs/dead-code-lint-hygiene/spec.md

- Source: openspec/changes/cleanup-inner-dead-code-allows/specs/dead-code-lint-hygiene/spec.md
- Lines: 1-20
- SHA256: 1ec68b211c00043a3c22efdb0d8963212d1792ee73213a4fc6ce9e9be09135be

```md
## MODIFIED Requirements

### Requirement: 不用 #[allow(dead_code)] 掩盖可修复的死字段
openlark 公开源代码 SHALL 不使用外层 `#[allow(dead_code)]` **或内层 `#![allow(dead_code)]`**（crate/mod 级）抑制可修复的 dead_code 警告。废弃模块、0 引用的 `pub(crate)`/私有脚手架 SHALL 直接删除；真死字段 SHALL 修正（读取/移除）或显式处理（`_` 前缀 + 注释 / `#[expect(dead_code)]`）。CI 死代码守卫脚本 SHALL 不保留 `KNOWN_INNER_DEBT` 类人为开口（inner-attribute 例外清单必须为空）。历次清理（#267 外层 392 处 + 本 change 内层 7 处/104 项）后，dead_code lint 信号 SHALL 对未来真死字段保持有效。

#### Scenario: HR crate 内外层均无残留
- **WHEN** 在 `crates/openlark-hr/` 中 grep `#!?\[allow\(dead_code\)\]`
- **THEN** 命中数为 0（原 361 外层 cruft + 已删除的废弃 `endpoints/` 模块 84 内层常量全清）

#### Scenario: 全 workspace 内外层均无 cruft 残留
- **WHEN** 在 `crates/` + `src/` 中 grep `#!?\[allow\(dead_code\)\]`（排除 `#[cfg(test)]` 测试代码）
- **THEN** 命中数为 0，或仅保留带显式注释说明的 `_` 前缀字段 / `#[expect(dead_code)]` 项

#### Scenario: CI 死代码守卫无人为开口
- **WHEN** 运行 `just no-dead-code-allows`（即 `tools/check_no_dead_code_allows.sh`）
- **THEN** `KNOWN_INNER_DEBT` 例外清单为空，inner-attribute 不再享受豁免

#### Scenario: 废弃模块与 0 引用脚手架被删除而非抑制
- **WHEN** 移除全部 `#![allow(dead_code)]` 后运行 `cargo clippy --workspace --all-targets`
- **THEN** 0 dead_code 警告，且承载原死代码的废弃模块（hr `endpoints/`）/ 0 引用脚手架（core `observability.rs` 等）已从源码删除，而非以 `#[expect]]` 保留
```

