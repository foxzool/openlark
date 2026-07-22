# Tasks

## 1. 删除风险装置代码 + 处置测试（crates/openlark-security/src/error.rs）

- [x]1.1 删除 `error.rs:344–600`：`SecurityErrorExt`（trait + impl）· `SecurityEvent` · `SecurityErrorAnalyzer` + `analyze_security_risk` · `SecurityRiskAssessment` · `SecurityRiskLevel` · `SecurityRiskClassify`（trait + `CoreError` impl）· `SecurityRiskType` · `SecurityAction` · `ComplianceImpact`。cut 点在 343/344 空行（紧邻活面 `map_feishu_security_error`），保留 `SecurityErrorBuilder` 与 `map_feishu_security_error`
- [x]1.2 删除 10 个装置测试：`test_security_event_generation` · `test_security_risk_assessment` · `analyze_security_risk_preserves_escalation_policy` · 7 个 `security_risk_level_*`（含 `#477 SecurityRiskClassify` 整段注释块）
- [x]1.3 改写 4 个 builder/mapper 测试，断言改为直接断 `CoreError` variant：
  - `test_security_error_creation`：`is_device_error()` → `matches!(error, CoreError::Validation { .. })` + ctx 含 `device_id`
  - `test_permission_error`：`is_permission_error()` → `matches!(error, CoreError::Authentication { code: ErrorCode::PermissionMissing, .. })`
  - `test_compliance_error`：`is_compliance_error()` → 断 ctx 含 `compliance_type`
  - `test_feishu_error_mapping`：`is_permission_error()` → `matches!(error, CoreError::Authentication { code: ErrorCode::PermissionMissing, .. })`

## 2. 依赖与 import 清理

- [x]2.1 从 `crates/openlark-security/Cargo.toml` 移除 `uuid = { workspace = true }`（仅 `SecurityEvent` 使用）
- [x]2.2 删 `error.rs:11` 的 `use serde::Serialize;`（装置删除后变成无用 import）
- [x]2.3 同步 `.github/msrv/Cargo.lock`（删 uuid 致 Cargo.lock 变动）

## 3. CHANGELOG 迁移条目

- [x]3.1 在 CHANGELOG 加 v0.19.0 breaking 条目：移除 security 风险评估装置公开类型，附迁移说明 + 例外理由（零消费者 / 非主推面 / 引 ADR-0001 + policy line 141）

## 4. 验证（CI 三元组 + 标准）

- [x]4.1 `cargo fmt --check` 通过
- [x]4.2 `cargo clippy --workspace --all-targets --all-features -- -Dwarnings` 通过
- [x]4.3 `cargo clippy --workspace --all-targets --no-default-features -- -Dwarnings` 通过
- [x]4.4 `cargo test --workspace --all-features` 通过（改写后的 4 测试 + 既有 security 测试不破坏）
- [x]4.5 `cargo doc --workspace --all-features` 通过（intra-doc 链无断；无残留装置符号 doc）
- [x]4.6 `cargo machete` 通过（uuid 已无引用）
- [x]4.7 msrv `--locked` 通过（`.github/msrv/Cargo.lock` 同步后）
- [x]4.8 真实 API 路径未受影响：grep 确认 `SecurityClient` / `SecurityError` 别名 / `SecurityResult` / ACS 与安全合规叶子未改动
