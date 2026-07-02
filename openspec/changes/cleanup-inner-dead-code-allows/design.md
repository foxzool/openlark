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
