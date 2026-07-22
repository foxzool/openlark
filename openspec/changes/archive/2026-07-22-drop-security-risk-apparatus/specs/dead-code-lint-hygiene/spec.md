## ADDED Requirements

### Requirement: 零消费者 pub 分析/扩展面 SHALL 删除或接真实消费者

openlark 公开模块中的分析/扩展装置（trait + analyzer + assessment / 事件类型等，如 `SecurityRiskClassify` + `analyze_security_risk` + `SecurityRiskAssessment`）若同时满足：(a) 全仓 grep 排除定义文件与 `target/` 后 **0 生产调用者**、(b) **不在** `lib.rs` / `prelude` re-export、文档示例或主推调用路径，SHALL **删除**，或接到真实消费者（≥2 adapter）使 seam 成立；不得作为 hypothetical seam（1 adapter、0 caller）长期保留。OpenLark 是库（非长驻服务），无内建 telemetry / escalation / metrics sink，分析装置的唯一可能消费者是下游应用——零下游需求时维护该面是认知税与维护负担。本 requirement 扩展 `dead-code-lint-hygiene` 的治理范围：从「不用 `#[allow(dead_code)]` 抑制可修复死字段」延伸到「不保留零消费者 pub 分析面」（后者不被 lint 抑制、正常编译，但同样是死代码）。`openlark-security` 的风险分类装置（`error.rs:344–600`）是其首个实例。

#### Scenario: security 风险分类装置移除

- **WHEN** 变更后在 `crates/openlark-security/src/` 中 grep `SecurityRiskClassify|analyze_security_risk|SecurityRiskAssessment|SecurityRiskLevel|SecurityRiskType|SecurityAction|ComplianceImpact|SecurityErrorAnalyzer|SecurityEvent|to_security_event|security_risk_level`
- **THEN** 命中数为 0（9 类型 + trait + Ext 事件方法定义全删）

#### Scenario: 全仓无装置符号外部引用

- **WHEN** 变更后在仓库中 grep 同组符号（排除 `target/`）
- **THEN** 命中数为 0（删除前已实证零外部引用，删除后亦无残留）

#### Scenario: uuid 依赖随装置移除

- **WHEN** 变更后检查 `crates/openlark-security/Cargo.toml`
- **THEN** 不含 `uuid`（`SecurityEvent` 是其唯一消费者）；`cargo machete` 不报 unused dep

#### Scenario: 活面与真实 API 路径不受影响

- **WHEN** 变更后构造 `SecurityClient::new(config)` 并调用 ACS / 安全合规叶子（如 `client.acs.v1().users().list()`）
- **THEN** 编译通过、行为不变；`SecurityError = CoreError` 别名、`SecurityResult`、`SecurityErrorBuilder`、`map_feishu_security_error` 仍可达

#### Scenario: security crate 编译与测试通过

- **WHEN** 运行 `cargo build -p openlark-security --all-features` 与 `cargo test -p openlark-security --all-features`
- **THEN** 均通过；4 个改写的 builder/mapper 测试以直接断 `CoreError` variant 形式通过
