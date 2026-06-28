# dead-code-lint-hygiene Specification

## Purpose
TBD - created by archiving change cleanup-dead-code-allows. Update Purpose after archive.
## Requirements
### Requirement: 不用 #[allow(dead_code)] 掩盖可修复的死字段
openlark 公开源代码 SHALL 不使用 `#[allow(dead_code)]` 抑制可修复的字段级 dead_code 警告。本次变更 SHALL 移除全部 392 处既有 `#[allow(dead_code)]`（其中 389 处经实证为不必要 cruft，3 处为需修正的真死字段）。

#### Scenario: HR crate 移除后无残留
- **WHEN** 在 `crates/openlark-hr/` 中 grep `#[allow(dead_code)]`
- **THEN** 命中数为 0（361 处 cruft 全部移除）

#### Scenario: 全 workspace 移除后无 cruft 残留
- **WHEN** 在 `crates/` + `src/` 中 grep `#[allow(dead_code)]`（排除 `#[cfg(test)]` 测试代码）
- **THEN** 命中数为 0，或仅保留带显式注释说明的 `_` 前缀字段

### Requirement: 真死字段必须修正或显式处理
3 个 platform v1 入口 struct 的 `config` 真死字段（`admin/admin/v1`、`app_engine/apaas/v1`、`directory/directory/v1`）SHALL 被修正（读取并传递给子 API）或显式处理（移除/`_` 前缀 + 注释），不得用 `#[allow(dead_code)]` 掩盖。

#### Scenario: platform v1 入口 struct config 字段不再触发 dead_code
- **WHEN** 移除 `#[allow(dead_code)]` 后运行 `cargo check -p openlark-platform`
- **THEN** 不再出现 `field config is never read` 警告（3 处全部解决）

### Requirement: dead_code lint 信号保持有效
本次变更后，dead_code lint SHALL 能检出未来引入的真死字段（信号未被 mass-suppression 淹没）。

#### Scenario: 三组 feature clippy 零 warning
- **WHEN** 运行 `cargo clippy --workspace --all-targets` 分别以 `（default）`、`--all-features`、`--no-default-features` + `-D warnings`
- **THEN** 三组均 exit 0

#### Scenario: 测试不回归
- **WHEN** 运行 `cargo test --workspace`
- **THEN** 全部测试通过（389 删除无行为影响；3 字段修正不破坏 v1 API）

