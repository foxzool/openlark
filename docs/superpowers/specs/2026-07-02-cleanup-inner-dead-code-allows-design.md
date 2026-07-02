---
comet_change: cleanup-inner-dead-code-allows
role: technical-design
canonical_spec: openspec
---

# Design Doc: cleanup-inner-dead-code-allows

> OpenSpec 产物（proposal/design/tasks + delta spec `dead-code-lint-hygiene`）是上游事实源。本 Design Doc 做深度技术设计，并记录对 open 阶段 design.md 的探查修正（D2/D3）。

## 1. 背景与目标

#267 清除 392 处外层 `#[allow(dead_code)]`，但正则 `#[` 漏匹配 `#![`，遗留 7 处内层 `#![allow(dead_code)]`（crate/mod 级），掩盖 104 处死代码，被 CI 脚本登记为 `KNOWN_INNER_DEBT` 例外。本 change 是 #267 的内层属性收尾：删 7 allow + 104 死项 + 清 CI 开口。

**目标**：dead_code lint 信号彻底恢复；CI 无人为开口；废弃模块/脚手架直接删除。**非目标**：不接线 hr endpoint（废弃旧物）、不接线 mail `User` accessor、不升 deny、不改 public API 行为。

## 2. 探查修正（推翻 open 阶段表述）

> **build Task 2 实测再修正（最重要）**：`observability.rs` **非全文件死**——`response_handler.rs` import 并使用 `observability::ResponseTracker`（3 处 `ResponseTracker::start`）。故 D2 改为**重写 `observability.rs` 仅保留 `ResponseTracker` + 其 4 测试**，删死 tracker（`OperationTracker`/`HttpTracker`/`AuthTracker`）/`trace_*` 函数/5 个 `trace_*` 宏/`tracing-init`+`otel` 门控 init 块 + 文件顶 `#![allow(dead_code)]`，**保留 `pub(crate) mod observability;`**。死代码扫描本就未标记 `ResponseTracker`（仅标记三个死 tracker + trace 函数）。另：根 `Cargo.toml` 有 `otel = ["openlark-core/otel"]` 转发 feature（Task 1 的 `crates/`-only grep 漏检根 crate），一并删。

### 2.1 D2 修正：`testing` feature 保留并解耦（非移除）

**open 阶段原判**：移除 `tracing-init`/`otel`/`testing` 三 feature。

**探查事实**：
- `openlark-core/src/lib.rs:24` `#[cfg(feature = "testing")] pub mod testing;` —— `testing` feature 门控一整套**自包含测试助手**（`testing/{assertions,mock_server,fixtures,mock_context}.rs`），暴露 `prelude::TestConfigBuilder` 等。
- 该模块**不引用 observability/tracing**。
- 被 hr/docs 测试**大量使用**：`grep` 显示 `openlark_core::testing::prelude::TestConfigBuilder` 出现在数十个 hr `attendance` 测试模块中。
- `openlark-hr`/`openlark-docs` 的 `[dev-dependencies]` 显式 `features = ["testing"]`。
- `testing = ["tracing-init"]` 的依赖链唯一去向是（已删的）observability。

**修正决策**：保留 `testing` feature，解耦为 `testing = []`（去掉 `["tracing-init"]`）。仅移除 `tracing-init` + `otel`。删 `observability.rs` + `pub(crate) mod observability;`。`pub mod testing` 全保留。

**BREAKING 面收窄**：原「三 feature 移除」→ 实际仅 `tracing-init`/`otel` 移除；无 workspace crate 直接启用这俩（仅 `testing` 被 hr/docs 启用，保留），故对 workspace 零破坏；对 crates.io 公众理论上 breaking（features 变少），但原 feature 只编译死代码、无行为。

### 2.2 D3 精确化：文件级 vs 项级

- **`query_params.rs`（整文件删）**：1085 行，顶层仅 `pub struct QueryParams` + `pub struct QueryParamsBuilder` 两个死 struct，0 外部 `use`。删文件 + `pub(crate) mod query_params;` 声明。
- **`header_builder.rs`（项级删）**：`HeaderBuilder::build_headers`/`add_header` **活**——`request_builder/mod.rs:46,48` 在用（`HeaderBuilder::build_headers(req_builder, config, option)` / `HeaderBuilder::add_header(...)`）。仅 `add_headers`（复数）函数死。删 `add_headers` 函数 + 文件顶部 `#![allow(dead_code)]`，保留文件其余。

### 2.3 依赖精确清单

`openlark-core/Cargo.toml` 删：
- `[features]`：`tracing-init`、`otel`（整行 + 注释）；`testing` 改 `testing = []`。
- `[dependencies]`：`opentelemetry`、`opentelemetry_sdk`、`opentelemetry-otlp`、`tracing-opentelemetry`（皆 `optional = true`，+ 其 `[dependencies.X]` 表）。
- `[dev-dependencies]`：`tracing-subscriber`（仅 observability 用）。

根 `Cargo.toml` `[workspace.dependencies]` 同步删上述 5 项（仅 openlark-core 用过）。

## 3. Build 执行顺序与验证点

| 步 | 操作 | 验证 |
|----|------|------|
| 1 | 前置核查：`grep` 确认无 crate 启用 `tracing-init`/`otel`（已证仅 hr/docs 启用 `testing`） | grep 输出 |
| 2 | core: 删 `observability.rs` + `lib.rs` 的 `pub(crate) mod observability;` | — |
| 3 | core: Cargo.toml 删 `tracing-init`/`otel` feature；`testing = []`；删 4 optional dep + `[dev-dep] tracing-subscriber` + 各 `[dependencies.X]` 表；根 workspace deps 删 5 项 | `cargo check -p openlark-core` 三组（default/--all-features/--no-default-features）绿 |
| 4 | core: 删 `query_params.rs` 整文件 + `mod query_params;`；删 `header_builder::add_headers` + 文件 `#![allow]]` | `cargo check -p openlark-core` 三组绿 |
| 5 | hr: 删 `endpoints/` 目录 + `lib.rs:69 mod endpoints;` + 附近注释 | `cargo check -p openlark-hr` 绿（testing 仍可用） |
| 6 | mail: 删 6 孤儿字段（alias/folder/mail_contact/rule 的 delete+patch 的 delete_id/patch_id）+ 各 `new()` 初始化；`User.config` 加 `#[expect(dead_code)]` + 注释；删 `lib.rs:1 #![allow]]` | `cargo check -p openlark-mail` 0 dead_code |
| 7 | bot: 删 `lib.rs:1 #![allow]]`；docs: 删 `explorer/mod.rs:1 #![allow]]` | — |
| 8 | CI: `tools/check_no_dead_code_allows.sh` 清空 `KNOWN_INNER_DEBT` + 更新文案 | `bash tools/check_no_dead_code_allows.sh` PASS |
| 9 | 全量：`cargo fmt --check`；`cargo clippy --workspace --all-targets` × 3 feature 组；`cargo test --workspace`；`cargo build` 双组；同步 `.github/msrv/Cargo.lock`；CHANGELOG | 全绿 |

## 4. 测试策略

死代码删除**不写新单测**（删除项均 0 引用，无可测行为）。验证靠守卫矩阵：

- **dead_code 归零**：`cargo clippy --workspace --all-targets` × {default, `--all-features`, `--no-default-features`} = 0 dead_code 警告、0 `#![allow(dead_code)]` 残留（覆盖 delta spec「全 workspace 内外层均无 cruft 残留」+「废弃模块被删除而非抑制」scenario）。
- **行为不回归**：`cargo test --workspace` 全绿（证伪「删 0 引用代码致行为回归」；hr endpoints 0 引用、mail 字段私有、observability 0 引用）。
- **feature 解耦不破坏**：`cargo build --workspace --all-features` + `--no-default-features` 双组绿（证伪 testing 解耦破坏 hr/docs）。
- **CI 长期信号**：`check_no_dead_code_allows.sh` 清空 `KNOWN_INNER_DEBT` 后对 inner-attr 一视同仁（覆盖 spec「CI 死代码守卫无人为开口」scenario）。
- **fmt**：`cargo fmt --check`（CI lint 第一步，避免重蹈 #270/#280）。
- **msrv**：删依赖须同步 `.github/msrv/Cargo.lock`，否则 CI msrv `--locked` fail（本地复现不出——已知坑）。

## 5. 边界条件（已全部定位）

- `testing` 解耦后 hr/docs 的 `features = ["testing"]` 仍有效（feature 在，只是不再拉 tracing-init）。
- `User.config` `#[expect(dead_code)]]` 精确写法：字段上方注解 + 中文注释「导航 struct，accessor 待补（#274/#275 范式），本 change 不接线」。MSRV 1.88 支持 `#[expect]]`。
- 无测试/示例直接引用 `otel`/`tracing-init` feature 符号。
- `tracing` 本体保留（`tracing::Span`/`span!` 等在他处用，非 5 个删除依赖之一）。

## 6. 风险与缓解

- **[BREAKING feature 移除]** → CHANGELOG 记录；workspace 内零破坏（仅 crates.io 公众理论 breaking，原 feature 无行为）。
- **[删依赖致 msrv lockfile 漂移]** → task 9 显式同步 `.github/msrv/Cargo.lock`。
- **[mail `User` 留半成品]** → `#[expect]]` + 注释指向 #274/#275 范式；不扩范围。
- **[query_params 1085 行误删活项]** → 探查已证顶层仅 2 死 struct + 0 外部 use；build 删前再 `grep` 复核。

## 7. OpenSpec 产物同步修正

写本 Design Doc 时同步修正 OpenSpec artifacts（design 阶段发现的事实修正，非范围变更）：
- `proposal.md`：BREAKING 表述「移除 tracing-init/otel/**testing** 三 feature」→ 「移除 tracing-init/otel；**testing 解耦保留**」；依赖/Impact 段同步。
- `design.md` D2/D3：按本 Doc §2 修正。
- delta spec 无需改（feature 解耦属 Impact，非 lint-hygiene 需求）。
