# Brainstorm Summary

- Change: fix-platform-v1-feature-gating
- Date: 2026-06-26

## 确认的技术方案

**方案 A（已用户确认）：移除 10 处 `#[cfg(feature = "v1")]` facade/intermediate 门控，其余不动。**

改动范围（6 文件 / 10 处 cfg 属性）：
- `src/admin.rs:29`（`pub fn v1()` 方法）、`src/admin.rs:35`（`pub mod admin`）
- `src/app_engine.rs:29`（v1 方法）、`src/app_engine.rs:35`（`pub mod apaas`）
- `src/directory/mod.rs:29`（v1 方法）、`src/directory/mod.rs:35`（`pub mod directory`）
- `src/directory/directory/mod.rs:3`（`pub mod v1`，intermediate）
- `src/spark/mod.rs:26`（v1 方法）、`src/spark/mod.rs:32`（`pub mod spark`）
- `src/spark/spark/mod.rs:3`（`pub mod v1`，intermediate）

效果：4 service 已在 `default` → 移除门控后 API 在 default 与 full 均可达，对齐 hr/communication/meeting。`service.v1()` 访问器保留（`lib.rs:166` 测试使用），`v1` flag 保留（`lib.rs~161` 测试门控 `cfg(all(feature="spark", feature="v1"))` 依赖）。**不改 Cargo.toml、不删 feature flag、不动 API。**

## 关键取舍与风险

- **取舍**：方案 A vs B（v1 加进 default+full，留 facade 门控）vs C（仅 full）。选 A——最小且自洽，不引入 platform 本不用的版本 feature 间接层。
- **风险①[默认构建变重]**：default 多编译 ~163 文件 → 可接受（service 默认启用的应有语义）。
- **风险②[latent 编译错误]**：v1 子树长期未被标准构建编译，移除门控可能暴露遗留问题 → build 阶段 `cargo check -p openlark-platform`（default+full）逐个修复。
- **风险③[clippy test-gating 回归]** → build 阶段强制 `--no-default-features` 与 `--all-features` 两组 clippy。

## 测试策略

- `cargo check -p openlark-platform`（default）+ `--all-features`（full）均通过。
- `cargo clippy --workspace --all-targets --no-default-features -- -D warnings` exit 0。
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` exit 0。
- `cargo test -p openlark-platform`（4 service 现有单测 + lib.rs:166 spark v1 链路测试）通过。
- 验证 `lib.rs~161` 的 `cfg(all(feature="spark", feature="v1"))` 测试仍编译（v1 flag 保留）。

## D2 处置（v4 空特征）

**保持现状不动**（已用户确认）。v4 不门控任何代码、仅出现在自身 `full` 定义中；v1 必须保留（测试依赖），v2/v3/v4 留作无害 no-op。最小改动原则，不顺手清理。

## Spec Patch

无。delta spec `platform-service-access` 的 4 条需求与方案 A 完全匹配（default/full 可达、clippy 不回归、公开符号不删），无需补充或修改验收场景。
