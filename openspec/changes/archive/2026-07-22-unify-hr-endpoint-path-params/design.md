## Context

`api_endpoints.rs`（3605 行，11 enum，~532 match arm）是 HR 的类型安全端点表。叶子消费：`XxxApiV1::Variant(...).to_url()` → 喂 `ApiRequest::get/put/...`。path-param 有两套约定并存（见 proposal Why）。

## Goals / Non-Goals

**Goals:**
- 把 6 个 Convention B 端点统一到 Convention A（variant 带参、编译期查）
- 行为逐字保持（URL 同串）→ 6 个 e2e 测试不变即过
- 防"忘记 `.replace`"的静默 `{}` URL 运行时 bug 类（Convention A 让缺参成编译期错误）

**Non-Goals:**
- 不做全量 macro/derive 深化（187 端点 / 532 arm）—— deletion test 下 enum 挣得位置，payoff 部分已实现（URL 已集中 1 文件）；grilling Q1 否决 A
- 不采纳 `API_PATH_PREFIX` 常量（532 arm 机械改，换"单一来源永不改变的串"，不抵成本）—— grilling Q2 收窄
- 不动 Convention A 端点、不重构 endpoint 表结构（更大议题，ADR-0001 硬约束 3 已 park）

## Decisions

### Decision 1: 只做 B→A 统一，不做全量深化 / 不采纳前缀常量

grilling 三问：(Q1) 范围 → B 有界清理；(Q2) 收窄 → 只做 `{}` 统一（6 端点），丢前缀（532 arm）；(Q3) 落地 → 方案 B（breaking、0.19.0、例外跳废弃周期）。详见 proposal Why。

### Decision 2: clone vs move

self 字段用 `.clone()`（保留字段供后续 body 用——patch/update 的 record_id/view_id 后续入 body；与原 `.replace` 的 borrow 意图一致）。本地 `record_key`（leave_employ_expire_record/get 计算得出、后续不用）用 move。

### Decision 3: 落地方案 B（同候选 1 先例）

pub variant 形态 unit→tuple 是 SemVer-breaking（cargo-semver-checks 会标）。但：行为逐字不变 + variant 仅被本 crate 6 叶子构造（0 外部消费者，外部用 leaf builder）→ 废弃周期无对象可警告。0.19.0 硬删、引 policy line 141 + ADR-0001 先例。比候选 1（删除）更低风险（行为保持）。

## Risks / Trade-offs

- **[BREAKING 公开 API]** 6 个 pub variant 形态变更 → **缓解**：行为逐字保持；0 外部消费者；v0.19.0 窗口；CHANGELOG 迁移表 + 例外理由
- **[clone 微开销]** patch/update 叶子 clone record_id/view_id → **缓解**：原 `.replace` 也是 borrow（非 move），clone 等价语义；一次 String clone 可忽略
- **[record_key 含 `-` 入 path]** `leave_employ_expire_records/{start}-{end}` → **缓解**：原 `.replace` 已如此（行为不变），是否应改 query-param 是另一议题
