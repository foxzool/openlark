## Why

`openlark-security/src/error.rs:344–600` 的整套安全风险分类装置——`SecurityRiskClassify` trait（+ `CoreError` impl）、`SecurityErrorAnalyzer::analyze_security_risk`、`SecurityRiskAssessment`、`SecurityRiskLevel` / `SecurityRiskType` / `SecurityAction` / `ComplianceImpact`、`SecurityEvent`、以及 `SecurityErrorExt` 的事件/谓词方法——经全仓 grep 实证 **零生产调用者**：排除定义文件与 `target/` 后，`SecurityRiskClassify` / `analyze_security_risk` / `SecurityRiskAssessment` / `SecurityRiskLevel` / `SecurityRiskType` / `SecurityAction` / `ComplianceImpact` / `SecurityErrorAnalyzer` / `SecurityEvent` / `to_security_event` / `security_risk_level` 均无任何 crate / example / 其它 security 子模块引用，仅 `error.rs` 自身的测试使用。

根因：OpenLark 是**库**（AGENTS.md 明示「不是长驻服务」），无内建 telemetry / escalation / metrics sink（grep `telemetry|escalat|audit|metrics_sink` 命中的 `platform/.../audit_log/` 是飞书审计日志 API 端点，非 SDK 自身风险评估的消费者）。风险评估装置的唯一可能消费者是**下游应用代码**，而它显示了零需求。

`SecurityRiskClassify`（#472 / #477）本身理由成立——把 `CoreError → SecurityRiskLevel` 分类表收口到一处 match、依赖方向正确（security 依赖 core）。但它服务于一个**无消费者的功能**：一条修得很好的、通往虚无的 seam。保留它即持续付出维护与认知税（CLAUDE.md §3「为未来版本预留是猜未来，通常错」；ADR-0001 理由 5 同此口径）。v0.19.0 breaking 窗口将至，是删除时机。

## What Changes

- 删除 `error.rs:344–600` 的 9 个类型 + trait：`SecurityErrorExt`（trait + `SecurityError` impl）、`SecurityEvent`、`SecurityErrorAnalyzer` + `analyze_security_risk`、`SecurityRiskAssessment`、`SecurityRiskLevel`、`SecurityRiskClassify`（trait + `CoreError` impl）、`SecurityRiskType`、`SecurityAction`、`ComplianceImpact`
- 删除 10 个仅测装置的测试（`test_security_event_generation` / `test_security_risk_assessment` / `analyze_security_risk_preserves_escalation_policy` / 7 个 `security_risk_level_*`）
- 改写 4 个 builder/mapper 测试（`test_security_error_creation` / `test_permission_error` / `test_compliance_error` / `test_feishu_error_mapping`）：断言从 `is_*_error()`（`SecurityErrorExt` 谓词，将删）改为直接断 `CoreError` variant
- 移除 `uuid` 依赖（仅 `SecurityEvent` 使用）+ 删变成无用的 `use serde::Serialize`
- **BREAKING**：移除上述公开类型（`pub mod error` 内 `pub` 项，零外部引用）；走例外跳过废弃周期
- 保留 `SecurityError = CoreError` 别名、`SecurityResult`、`SecurityErrorBuilder`、`map_feishu_security_error`、core 错误 helper re-export（叶子经 `Transport::request_typed → decode → CoreError` 构造错误，不走 `SecurityErrorBuilder`）

## Capabilities

### New Capabilities
（无）

### Modified Capabilities
- `dead-code-lint-hygiene`：新增 requirement。现有 requirement 治理 `#[allow(dead_code)]` 抑制的可修复死字段；本 requirement 扩到另一类死代码——**零消费者 pub 分析/扩展面**（trait + analyzer + assessment 等装置，不被 lint 抑制、正常编译，但 0 生产调用者、不在 prelude/re-export/文档主推面）。此类面 SHALL 删除或接真实消费者（≥2 adapter），不得作为 hypothetical seam 长期保留。security 风险装置是其首个实例。

## Impact

- **crates/openlark-security**（改动集中于此，不跨 crate）：
  - `src/error.rs`：删 344–600 装置 + 10 测试、改写 4 测试、删 `use serde::Serialize`
  - `Cargo.toml`：移除 `uuid` 依赖
- **公开 API（v0.19.0 breaking）**：移除 9 个公开类型 + 1 个 trait（零外部引用，删除无外部消费方成本）
- **真实 API 路径**：不受影响——ACS / 安全合规叶子经 `Transport::request_typed` 调用，`SecurityClient` / `SecurityError` 别名 / `SecurityResult` 删除前后均可达且行为不变
- **依赖**：移除 `uuid`；`chrono` 保留（`openapi_logs/list_data.rs` 活叶子使用）；无新增
- **后续**：`SecurityErrorBuilder` + `map_feishu_security_error` 同为零消费者测试专属死代码，另案 #500 跟进（本 change 不扩大范围）
