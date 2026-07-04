## ADDED Requirements

### Requirement: okr/v2 叶子端点 SHALL 经 OkrApiV2 enum 构造

openlark-hr 中 okr/v2 的全部叶子端点 SHALL 经 `OkrApiV2` enum（`crates/openlark-hr/src/common/api_endpoints.rs`）的 `.to_url()` 构造 URL，不得在叶子 `execute()` 内 inline `format!("/open-apis/okr/v2/...")` 或写字符串字面量。`OkrApiV2` enum 是 okr/v2 端点路径的 single source of truth。

本 requirement 填补 `v1-sub-api-accessors` 中 okr/v2 typed Response requirement 显式 carve-out 的空缺——该 requirement 原文声明「端点 enum 一致性（inline `format!` vs `OkrApiV2`）不在本 requirement 范围」，本 requirement 接管该范围。属非破坏性内部一致性收敛：URL 路径串逐字节不变，仅构造方式从硬编码迁移到 enum 调用；对齐 `openlark-api` 核心契约 5（端点用 enum）与 AGENTS.md ANTI-PATTERNS「不要硬编码 URL」。

#### Scenario: 25 叶全部经 enum 构造

- **WHEN** 变更后检查 `crates/openlark-hr/src/okr/okr/v2/` 下全部 25 个叶子的 `execute_with_options()` URL 构造
- **THEN** 25 叶（含 `cycle/list` 生产代码——其原本仅测试用了 enum）均经 `OkrApiV2::*().to_url()` 构造，叶目录内零处 inline `format!("/open-apis/okr/v2/...")` 生产构造、零处 `ApiRequest::get("/open-apis/okr/v2/...")` 字符串字面量生产构造

#### Scenario: URL 路径串逐字节不变

- **WHEN** 对比迁移前后各叶端点实际请求 URL
- **THEN** 全部 25 叶 URL 路径与迁移前完全相同（如 `objective/get` 仍为 `/open-apis/okr/v2/objectives/{objective_id}`、`cycle/objectives_weight` 仍为 `/open-apis/okr/v2/cycles/{cycle_id}/objectives_weight`），仅构造方式由 inline 改为 enum；由新增的 `OkrApiV2` 25 variant `to_url()` 测试逐字节锁定（叶子既有 `test_url_path` 为 Potemkin、不测生产，不作验证依据）

#### Scenario: category/list 与 cycle/list 字符串字面量形态收敛

- **WHEN** 变更后检查 `category/list.rs` 与 `cycle/list.rs` 的 `execute_with_options()`
- **THEN** 不再内联 `ApiRequest::get("/open-apis/okr/v2/categories")` / `ApiRequest::get("/open-apis/okr/v2/cycles")`，均改为经 `OkrApiV2::CategoryList/CycleList.to_url()` 构造后传入 `ApiRequest::get`（`cycle/list` 保留后续 `.query(...)` 链式）

#### Scenario: OkrApiV2 enum 本身不变

- **WHEN** 变更后检查 `crates/openlark-hr/src/common/api_endpoints.rs` 的 `OkrApiV2` 定义与 `to_url()` 实现
- **THEN** 25 个 variant 与各 `to_url()` 分支与本变更前逐行一致（本 change 纯调用点迁移，不增删 variant、不改 URL 实现）

#### Scenario: OkrApiV2 to_url 测试覆盖全 25 variant

- **WHEN** 变更后检查 `api_endpoints.rs::test_okr_api_urls`
- **THEN** 覆盖 `OkrApiV2` 全部 25 variant 的 `to_url()` 输出断言（带 id variant 用占位 id），逐字节锁定期望 URL；该测试在本 change 迁移前置落地，作为各批次迁移的回归基线
