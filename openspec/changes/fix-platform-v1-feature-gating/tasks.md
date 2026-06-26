# Tasks — fix-platform-v1-feature-gating

> 注：以下为初步任务分解。具体修法（移除 facade 门控 vs 把 `v1` 纳入 default/full）经 design 阶段 brainstorming 确认后，第 2、3 组任务可能微调。

## 1. 调研与边界确认（build 前）

- [ ] 1.1 grep 确认 `v1`/`v2`/`v3`/`v4` feature 在测试文件中的全部 cfg 用法，明确哪些必须保留
- [ ] 1.2 确认移除 facade 门控后，`--no-default-features` 下各 service 模块（`admin`/`app-engine`/`directory`/`spark`）的编译边界与 test-gating 互不影响
- [ ] 1.3 核实 `full` 中空 `v4` feature 是否被任何测试/代码引用，决定保留或清理

## 2. 实施门控修正

- [ ] 2.1 修正 `src/admin.rs` 的 `#[cfg(feature = "v1")]` 门控（`pub fn v1()` + `pub mod admin`）
- [ ] 2.2 修正 `src/app_engine.rs` 的门控（`pub fn v1()` + `pub mod apaas`）
- [ ] 2.3 修正 `src/directory/mod.rs` 的门控（`pub fn v1()` + `pub mod directory`）
- [ ] 2.4 修正 `src/spark/mod.rs` 的门控（`pub fn v1()` + `pub mod spark`）
- [ ] 2.5 按 brainstorm 选型调整 `crates/openlark-platform/Cargo.toml` `[features]`（若选 B/C：把 `v1` 纳入 default/full）

## 3. 编译验证

- [ ] 3.1 `cargo check -p openlark-platform`（default）通过
- [ ] 3.2 `cargo check -p openlark-platform --all-features`（full）通过
- [ ] 3.3 修复门控移除后暴露的 latent 编译问题（v1 子树长期未被标准构建编译，可能有遗留问题）

## 4. 质量门

- [ ] 4.1 `cargo clippy --workspace --all-targets --no-default-features -- -D warnings` exit 0
- [ ] 4.2 `cargo clippy --workspace --all-targets --all-features -- -D warnings` exit 0
- [ ] 4.3 四个 service 现有单元测试通过（`cargo test -p openlark-platform`）

## 5. 文档与收尾

- [ ] 5.1 CHANGELOG 记录本次行为补全（非 breaking）
- [ ] 5.2 更新 AGENTS.md / 相关 feature 说明（若涉及 default/full 语义变化）
- [ ] 5.3 `cargo doc -p openlark-platform --all-features --no-deps` 确认文档可生成
