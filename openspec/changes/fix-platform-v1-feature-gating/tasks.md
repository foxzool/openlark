# Tasks — fix-platform-v1-feature-gating

> 方案 A 已执行：移除 6 文件 10 处 `#[cfg(feature = "v1")]` 门控，不改 Cargo.toml、不删 flag、不动 API。

## 1. 调研与边界确认（build 前）

- [x] 1.1 grep 确认 `v1`/`v2`/`v3`/`v4` feature 在测试文件中的全部 cfg 用法，明确哪些必须保留
- [x] 1.2 确认移除 facade 门控后，`--no-default-features` 下各 service 模块（`admin`/`app-engine`/`directory`/`spark`）的编译边界与 test-gating 互不影响
- [x] 1.3 核实 `full` 中空 `v4` feature 是否被任何测试/代码引用，决定保留或清理 → v4 无代码引用，保持现状（D2）

## 2. 实施门控修正

- [x] 2.1 修正 `src/admin.rs` 的 `#[cfg(feature = "v1")]` 门控（`pub fn v1()` + `pub mod admin`）
- [x] 2.2 修正 `src/app_engine.rs` 的门控（`pub fn v1()` + `pub mod apaas`）
- [x] 2.3 修正 `src/directory/mod.rs` 的门控（`pub fn v1()` + `pub mod directory`）+ `directory/directory/mod.rs:3` intermediate
- [x] 2.4 修正 `src/spark/mod.rs` 的门控（`pub fn v1()` + `pub mod spark`）+ `spark/spark/mod.rs:3` intermediate
- [x] 2.5 ~~按 brainstorm 选型调整 Cargo.toml~~ → N/A（方案 A 不改 Cargo.toml）
- [x] 2.6 保留 `/// V1 版本 API` 文档注释（crate 有 `missing_docs`，删注释会触发 warning）

## 3. 编译验证

- [x] 3.1 `cargo check -p openlark-platform`（default）通过，0 warning
- [x] 3.2 `cargo check -p openlark-platform --all-features`（full）通过，0 warning
- [x] 3.3 修复门控移除后暴露的 latent 问题：4 处 `missing_docs` warning（v1 方法），通过保留文档注释修复

## 4. 质量门

- [x] 4.1 `cargo clippy --workspace --all-targets --no-default-features -- -D warnings` exit 0
- [x] 4.2 `cargo clippy --workspace --all-targets --all-features -- -D warnings` exit 0
- [x] 4.3 `cargo test -p openlark-platform`（default 219 passed）+ `--all-features`（13 contract passed，含 `spark_id_convert_contracts` 验证 v1 链）

## 5. 文档与收尾

- [x] 5.1 CHANGELOG `[Unreleased] > Fixed` 记录本次行为补全
- [x] 5.2 ~~更新 AGENTS.md~~ → N/A（AGENTS.md 不涉及 platform v1 门控细节）
- [x] 5.3 `cargo doc -p openlark-platform --all-features --no-deps` 生成成功（153 个 `bare_urls` warning 为既有问题，非本次引入，architecture-audit-review 已登记后续清理）
