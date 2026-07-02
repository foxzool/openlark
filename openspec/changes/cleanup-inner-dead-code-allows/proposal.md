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
