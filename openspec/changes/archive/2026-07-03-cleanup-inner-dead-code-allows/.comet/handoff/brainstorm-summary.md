# Brainstorm Summary

- Change: cleanup-inner-dead-code-allows
- Date: 2026-07-02

## 确认的技术方案

逐 crate 清除 7 处 inner `#![allow(dead_code)]` + 其掩盖的 104 处死代码，沿用 #267 范式（删 cruft / `_` 前缀 / `#[expect]]`）。**open 阶段 design.md 的 D2/D3 经探查修正**：

- **D2（修正）**：`testing` feature **保留并解耦**为 `testing = []`（原误判「移除」）。`pub mod testing`（TestConfigBuilder 等）被 hr/docs 测试大量使用、自包含、不依赖 observability。仅移除 `tracing-init` + `otel` feature + 删 `observability.rs`。删 5 个仅服务这些 feature 的依赖（openlark-core 4 optional + 1 dev-dep）+ 根 `[workspace.dependencies]` 同步删。
- **D3（精确化）**：`query_params.rs` **整文件删**（1085 行，2 死 struct，0 use）；`header_builder.rs` **项级删**（仅 `add_headers`，`build_headers`/`add_header` 活于 `request_builder/mod.rs`）。
- D1（hr 废弃 endpoints 整模块删）、D4（mail 删 6 孤儿字段 + `User.config` `#[expect]]`）、D5（bot/docs stale allow 删）、D6（CI KNOWN_INNER_DEBT 清空）、D7（沿用 #267）不变。

Build 顺序：前置核查 → core(observability→Cargo→query_params/header_builder) → hr → mail → bot/docs → CI → 全量验证。每步带 `cargo check -p` 验证点。

## 关键取舍与风险

- **取舍**：`User.config` 用 `#[expect]]` 而非接线 `query()` accessor——本 change 是清债不接线，接线留作 #274/#275 类另案 feature 工作。
- **风险**：① 移除 `tracing-init`/`otel` 是 Cargo manifest breaking（但无 workspace crate 直接启用，仅理论 breaking；`testing` 保留故 hr/docs 不受影响）；② 删依赖须同步 `.github/msrv/Cargo.lock` 否则 CI msrv `--locked` fail。

## 测试策略

死代码删除靠守卫，非单测：`cargo clippy --workspace --all-targets` × {default, --all-features, --no-default-features} 三组 0 dead_code/0 `#![allow]]`；`cargo test --workspace` 全绿（证伪行为回归）；`cargo build` 双 feature 组绿（证伪 testing 解耦破坏）；`cargo fmt --check`（CI lint 第一步）；CI `check_no_dead_code_allows.sh` 清空 KNOWN_INNER_DEBT 后一视同仁。

## Spec Patch

无。delta spec（`dead-code-lint-hygiene` MODIFIED，4 scenario）覆盖完整；feature 解耦/移除属 Impact 不入 spec。**但需修正 OpenSpec `proposal.md`/`design.md` 两处过时表述**（testing「保留解耦」非「移除」、BREAKING 面收窄为仅 tracing-init/otel），在写 Design Doc 时同步改。
