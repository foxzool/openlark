# dead-code-lint-hygiene Specification

## Purpose
TBD - created by archiving change cleanup-dead-code-allows. Update Purpose after archive.
## Requirements
### Requirement: 不用 #[allow(dead_code)] 掩盖可修复的死字段
openlark 公开源代码 SHALL 不使用外层 `#[allow(dead_code)]` **或内层 `#![allow(dead_code)]`**（crate/mod 级）抑制可修复的 dead_code 警告。废弃模块、0 引用的 `pub(crate)`/私有脚手架 SHALL 直接删除；真死字段 SHALL 修正（读取/移除）或显式处理（`_` 前缀 + 注释 / `#[expect(dead_code)]`）。CI 死代码守卫脚本 SHALL 不保留 `KNOWN_INNER_DEBT` 类人为开口（inner-attribute 例外清单必须为空）。历次清理（#267 外层 392 处 + 本 change 内层 7 处/104 项）后，dead_code lint 信号 SHALL 对未来真死字段保持有效。

#### Scenario: HR crate 内外层均无残留
- **WHEN** 在 `crates/openlark-hr/` 中 grep `#!?\[allow\(dead_code\)\]`
- **THEN** 命中数为 0（原 361 外层 cruft + 已删除的废弃 `endpoints/` 模块 84 内层常量全清）

#### Scenario: 全 workspace 内外层均无 cruft 残留
- **WHEN** 在 `crates/` + `src/` 中 grep `#!?\[allow\(dead_code\)\]`（排除 `#[cfg(test)]` 测试代码）
- **THEN** 命中数为 0，或仅保留带显式注释说明的 `_` 前缀字段 / `#[expect(dead_code)]` 项

#### Scenario: CI 死代码守卫无人为开口
- **WHEN** 运行 `just no-dead-code-allows`（即 `tools/check_no_dead_code_allows.sh`）
- **THEN** `KNOWN_INNER_DEBT` 例外清单为空，inner-attribute 不再享受豁免

#### Scenario: 废弃模块与 0 引用脚手架被删除而非抑制
- **WHEN** 移除全部 `#![allow(dead_code)]` 后运行 `cargo clippy --workspace --all-targets`
- **THEN** 0 dead_code 警告；废弃模块（hr `endpoints/`）整模块删除；0 引用脚手架（core `observability.rs` 死 tracker/trace 函数/宏、`query_params.rs` 整文件、`header_builder::add_headers`）从源码删除而非 blanket 抑制；**活代码保留**（`observability::ResponseTracker` 被 `response_handler` 使用故保留）；导航/服务 struct 的条件死字段（feature 关闭时死）用 `#[expect(dead_code)]` / `cfg_attr` 显式标注

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

