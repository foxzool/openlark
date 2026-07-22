## Context

`openlark-security/src/error.rs`（772 行）有两类公开面：

```
error.rs
├── 活面（保留）
│   ├── SecurityError = CoreError（别名，lib.rs re-export）
│   ├── SecurityResult<T>
│   ├── SecurityErrorBuilder（domain 味构造器；另案 #500）
│   └── map_feishu_security_error（飞书码映射；另案 #500）
└── 死装置 344–600（删除）
    ├── SecurityErrorExt trait + impl（谓词 + 事件方法）
    ├── SecurityEvent
    ├── SecurityErrorAnalyzer + analyze_security_risk
    ├── SecurityRiskAssessment / SecurityRiskLevel / SecurityRiskType
    ├── SecurityRiskClassify trait + CoreError impl
    └── SecurityAction / ComplianceImpact
```

全仓 grep 实证：死装置 9 个类型 + trait 的全部符号，排除定义文件与 `target/` 后 **0 命中**。它们不在 `lib.rs` / `prelude` re-export（仅 `SecurityError` 别名 + `SecurityResult` 被 re-export），无文档/示例指向。security API 叶子经 `Transport::request_typed → Response::decode → CoreError` 构造错误，根本不走这套装置。

`SecurityRiskClassify` trait（#472 / #477）把 `CoreError → SecurityRiskLevel` 分类表收口到一处，依赖方向正确；但风险评估功能 0 消费者——它是 hypothetical seam（1 adapter `CoreError`、0 调用者）。

## Goals / Non-Goals

**Goals:**
- 删除零消费者的风险分类装置（9 类型 + trait + 10 装置测试）
- 改写 4 个 builder/mapper 测试，去掉对 `SecurityErrorExt` 谓词的依赖
- 连带清理：移 `uuid` 依赖、删无用 `use serde::Serialize`
- 保持真实 API（ACS / 安全合规叶子、`SecurityClient`、`SecurityError` 别名）路径与行为完全不变

**Non-Goals:**
- 不动 `SecurityErrorBuilder` / `map_feishu_security_error`（同源死代码，另案 #500；本 change 保持手术式范围）
- 不重构 `error.rs` 其余活面
- 不引入风险评估的替代实现（无消费者，YAGNI）

## Decisions

### Decision 1: 删除而非保留/接线

**决策**：删整块装置。

**为什么**：0 消费者 + 无 sink（库天然无）+ trait 解的是无人用之题。保留即维护一条通往虚无的 seam。"留并接真实消费者"需凭空发明一个今天不存在的消费者——投机性造功能，非深化。CLAUDE.md §3 + ADR-0001 理由 5 均拒绝「为未来预留是猜未来」。

**备选**：保留 trait 仅作分类表（variant→level 单一来源）→ 否决（一张没人消费的分类表还是死代码；真有消费者时 10 行即可加回）。

### Decision 2: 整块删 344–600（含 SecurityErrorExt），非部分切

**决策**：从 `SecurityErrorExt` 到 `ComplianceImpact` 整块删（344–600），不保留谓词。

**为什么**：谓词 `is_device_error / is_permission_error / is_compliance_error` 唯一非测试调用者就是 `analyze_security_risk`（随删）；事件方法 `to_security_event / security_operation / affected_resource_id` 只喂死的 `SecurityEvent`。整块是内聚死单元，deletion test 干净（删后复杂度不回归）。

**注意（test 耦合）**：4 个 builder/mapper 测试经 `is_*_error()` 谓词依赖 `SecurityErrorExt`——删 Ext 会编译断裂。处置见 Decision 3。

### Decision 3: 4 个断裂测试改写为直接断 CoreError variant

**决策**：`test_security_error_creation` / `test_permission_error` / `test_compliance_error` / `test_feishu_error_mapping` 的断言从 `err.is_permission_error()` 等改为直接断 variant（如 `matches!(err, CoreError::Authentication { code: ErrorCode::PermissionMissing, .. })`），保留对 `SecurityErrorBuilder` / `map_feishu_security_error` 的覆盖。

**为什么**：与"构造器另案、本次不动"立场一致；改写后测得更实（直接验 variant 而非谓词包装）。

**备选**：连测试带构造器一起删 → 否决（扩大范围成"清空 error.rs 大半"，Kitchen Sink 风险 CLAUDE.md §10；构造器死活留作 #500）。

### Decision 4: 落地走方案 B——breaking、0.19.0 硬删、例外跳过废弃周期

**决策**：判 breaking（policy line 115「删除公开类型」），进 0.19.0 窗口硬删，不挂 `#[deprecated]`；CHANGELOG 迁移表 + 例外理由（policy line 141）。

**为什么**：给零使用量类型挂 `#[deprecated`] 是纯表演（警告不到任何人），且项目刚于 2026-06-29 把全仓 `#[deprecated]` 清零，转头再挂是反讽。例外理由：零消费者 + 不在 prelude/文档/示例 + 从未主推 → 废弃周期无对象可警告。与 ADR-0001 先例一致（v0.18 窗口硬删 pub 导航壳 + CHANGELOG 迁移表）。

**备选 A**（当作非稳定实验面现在硬删，套 policy line 26）→ 否决（钻漏洞；cargo-semver-checks / 严格审计仍判 breaking）。
**备选 C**（先 `#[deprecated]` 再删）→ 否决（见上，表演 + 反讽）。

### Decision 5: 连带清理 uuid + Serialize，chrono 留

**决策**：移 `uuid`（仅 `SecurityEvent` 用）、删 `use serde::Serialize`（装置删除后无用）；`chrono` 保留（`openapi_logs/list_data.rs:53/59` 活叶子使用）。同步 `.github/msrv/Cargo.lock`（删依赖触发 msrv `--locked`）。

## Risks / Trade-offs

- **[BREAKING 公开 API]** 移除 9 个公开类型 + 1 trait → **缓解**：全仓 grep 实证零跨 crate 引用、零外部消费；不在 prelude/re-export/文档主推面；v0.19.0 breaking 窗口；CHANGELOG 迁移表 + 例外理由
- **[4 测试编译断裂]** 删 `SecurityErrorExt` 谓词致 builder 测试断裂 → **缓解**：同 change 内改写为直接断 variant；`cargo test -p openlark-security` 验证
- **[uuid 移除致 msrv --locked 失败]** 删依赖改 Cargo.lock → **缓解**：同步 `.github/msrv/Cargo.lock`（memory: 删/改依赖的 change 须同步 msrv pinned lockfile）；`cargo machete` 验 uuid 已无引用
- **[误删活面]** 装置边界 344–600 紧邻活面 `map_feishu_security_error`（~299–342）→ **缓解**：cut 点在 343/344 空行处，清晰；`cargo build -p openlark-security --all-features` 验活面可达
