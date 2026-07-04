# Comet Design Handoff

- Change: okr-v2-endpoint-enum
- Phase: design
- Mode: compact
- Context hash: 5f4474c97300f22267f0682db0ad32be06755324c955b8f2474d748a703c61d0

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/okr-v2-endpoint-enum/proposal.md

- Source: openspec/changes/okr-v2-endpoint-enum/proposal.md
- Lines: 1-38
- SHA256: 15da8e0c0f2d73a550f70e29d0b3387e511a37cb53fbf9cdfa03d18d995be8fd

```md
## Why

`openlark-hr` okr/v2 的 25 个叶子端点 URL 构造方式**不一致**，`OkrApiV2` enum（`crates/openlark-hr/src/common/api_endpoints.rs:1941`）25 variant 齐备却**在生产代码中零调用**：

- **23 叶** inline `format!("/open-apis/okr/v2/...")` 硬编码 ❌
- **2 叶** inline 字符串字面量：`category/list` 的 `ApiRequest::get("/open-apis/okr/v2/categories")` 与 `cycle/list` 生产代码的 `ApiRequest::get("/open-apis/okr/v2/cycles")` ❌（`cycle/list` 仅其**测试**用了 enum，生产未用）

这违 `openlark-api` 核心契约 5（端点用 enum）+ AGENTS.md ANTI-PATTERNS「不要硬编码 URL」。`OkrApiV2` enum 25 个 variant **已齐备且 `to_url()` 实现正确**，但生产代码零调用——enum 形同 Potemkin，single source of truth 失效。

这是 **#328（type-okr-v2-responses）刻意 carve-out 的遗留**：该 change 的 D4 决策保留 URL 构造不动以避免混入 typed 化主线，其 spec 显式声明「端点 enum 一致性不在本 requirement 范围」。本 change 填补这块 carve-out，是 #328 code-review follow-up 的第三支（#336 消重、#337 CHANGELOG 之外的 URL 收敛）。

issue #338 原称「12 叶 inline format!」是**低估**：仅匹配单行 `format!("/open-apis/okr/v2`，漏数 12 个多行 `format!(` 与 2 个字符串字面量（`category/list`、`cycle/list` 生产）。实测 **25 叶全部待迁移**（enum 生产零调用）。

## What Changes

- 把 okr/v2 **25 个叶子**的端点 URL 构造从 inline `format!`（23 叶）/ inline 字符串字面量（`category/list`、`cycle/list` 2 叶）统一改为 `OkrApiV2::*().to_url()`
- `OkrApiV2` enum 已含全部 25 variant，**无需新增 variant**——纯调用点迁移
- URL 路径字符串**零变化**（enum 的 `to_url()` 已产生与现有 inline 串完全相同的路径）
- **补 `OkrApiV2` enum `to_url()` 测试**：`api_endpoints.rs::test_okr_api_urls` 当前只测 `OkrApiV1`，补全 25 variant 断言，锁定 variant→URL 映射（迁移的安全网，抓潜伏休眠 bug）
- 不动 typed Response（#328 已定）、不动导航链、不动请求/响应模型、不动叶子现有测试（含 Potemkin `test_url_path`，另案）

## Capabilities

### New Capabilities
（无）

### Modified Capabilities
- `v1-sub-api-accessors`：新增 okr/v2 端点 URL 构造一致性 requirement。现有 requirement 覆盖「版本节点 SHALL 暴露链式 accessor」「HR 零 accessor 死节点 SHALL 删除」（#327）与「okr/v2 navigable 链叶子 SHALL 返回 typed Response」（#328，显式把端点 enum 排除在外）；本 change 补第四条——**okr/v2 叶子端点 SHALL 经 `OkrApiV2` enum 的 `.to_url()` 构造**，不得 inline `format!` 或字符串字面量硬编码。属非破坏性内部一致性收敛（URL 串不变，纯构造方式迁移）。

## Impact

- **crates/openlark-hr**（全部改动集中于此）：
  - `src/okr/okr/v2/<resource>/...` 25 个叶子的 `execute_with_options()`：URL 构造行 `format!(...)` / 字符串字面量 → `OkrApiV2::Variant(self.id).to_url()`
  - 资源批次：alignment(2) / category(1) / cycle 含 objective 子树(6，含 cycle/list) / indicator(1) / key_result 含 indicator/progress(5) / objective 含 alignment/indicator/key_result/progress(10)
  - `src/common/api_endpoints.rs` `OkrApiV2`：enum 本身不动（variant 齐、`to_url()` 正确）；**仅补 `test_okr_api_urls` 的 25 variant to_url 断言**
- **公开 API**：无变化（URL 串不变，`execute()` 签名不变）——非 breaking
- **依赖**：无新增
- **测试**：叶子现有 `test_url_path` 是 Potemkin（不调生产），**不动**（修需架构改动，另案）；迁移安全网靠新增的 enum `to_url()` 25 variant 测试 + 全量 build/clippy/fmt/test + 验收 grep
```

## openspec/changes/okr-v2-endpoint-enum/design.md

- Source: openspec/changes/okr-v2-endpoint-enum/design.md
- Lines: 1-91
- SHA256: 766b2eea3ef0e92fced3eec7456f955a3bb89eee93e68460373e3bda38af44c2

[TRUNCATED]

```md
## Context

`OkrApiV2` enum（`crates/openlark-hr/src/common/api_endpoints.rs:1941`）已定义全部 25 个 variant 并实现正确的 `to_url()`，但**生产代码零调用**（`cycle/list.rs:177` 的 enum 用法在其**测试**里，非生产）。25 叶全部用 inline `format!`（23）或字符串字面量（`category/list`、`cycle/list` 生产 2）硬编码相同路径，enum 形同摆设。本设计说明如何把 25 叶统一到 enum 调用，且保证 URL 串零变化。

前置事实（已代码核实）：
- enum variant 集**完整**：25 叶每一片都对应一个已存在 variant（见下表）
- `execute(self)` 消费 self，`self.xxx_id` 为 owned，可 move 进 variant，无需 `.clone()`
- `cycle/list.rs` 生产代码（`execute_with_options` 行 72）**也是 inline** `"/open-apis/okr/v2/cycles"`，仅其测试用了 enum——故 cycle/list 亦在迁移列
- `OkrApiV2` enum `to_url()` **零测试覆盖**（`test_okr_api_urls` 只测 `OkrApiV1::PeriodList`）；叶子 `test_url_path` 是 Potemkin（建 `_req` 丢弃、assert 只测 `format!` 宏），不调生产

## Goals / Non-Goals

**Goals:**
- okr/v2 **25 叶**端点 URL 全部经 `OkrApiV2::*().to_url()` 构造（含 cycle/list 生产代码）
- URL 路径字符串前后逐字节相同（纯构造方式迁移）
- `OkrApiV2` 成为 okr/v2 端点的真实 single source of truth
- 补 `OkrApiV2` enum `to_url()` 的 25 variant 测试，锁定映射（迁移安全网）

**Non-Goals:**
- 不改 typed Response / 请求响应模型（#328 范畴）
- 不动导航链与资源 accessor
- 不动其他 crate 的同类 inline URL（另案；本 change 范围限 okr/v2）
- 不重构 `OkrApiV2` enum 本身（variant 齐、`to_url()` 正确，无需改）
- 不改测试内部的 URL 期望值断言

## Decisions

### D1: 全量迁移 25 叶（非 issue 字面的 12 叶）
issue #338 称「12 叶」源于窄 grep（只匹配单行 `format!("/open-apis/okr/v2`），漏 12 个多行 `format!(` + 2 个字符串字面量（`category/list`、`cycle/list` 生产）。brainstorming 进一步发现 cycle/list **生产代码也 inline**（仅测试用了 enum），故全量 = **25 叶**。用户已确认全量执行。半量迁移留漏网叶，违「不硬编码 URL」契约且使 enum 仍是 Potemkin。

**替代方案**：仅迁单行 12 叶——拒绝，半成品且违背契约初衷。

### D2: 复用现有 enum，不增 variant
25 叶 → variant 映射已全覆盖（含 `CycleObjectiveCreate`/`CycleObjectiveList` 同 URL 不同 variant、`ObjectiveAlignmentCreate`/`ObjectiveAlignmentList`、`ObjectiveKeyResultCreate`/`ObjectiveKeyResultList` 同 URL 不同 variant——这是 enum 既有设计，语义分层而非 URL 分层）。

| 资源（叶数） | 叶 → variant |
|---|---|
| alignment (2) | delete→AlignmentDelete, get→AlignmentGet |
| category (1) | list→CategoryList（当前为字符串字面量） |
| cycle (1) | list→CycleList（生产为字符串字面量，仅测试用了 enum） |
| cycle/objective (2) | create→CycleObjectiveCreate, list→CycleObjectiveList |
| cycle (2) | objectives_position→CycleObjectivesPosition, objectives_weight→CycleObjectivesWeight |
| indicator (1) | patch→IndicatorPatch |
| key_result (3) | delete→KeyResultDelete, get→KeyResultGet, patch→KeyResultPatch |
| key_result/indicator (1) | list→KeyResultIndicatorList |
| key_result/progress (1) | list→KeyResultProgressList |
| objective (3) | delete→ObjectiveDelete, get→ObjectiveGet, patch→ObjectivePatch |
| objective/alignment (2) | create→ObjectiveAlignmentCreate, list→ObjectiveAlignmentList |
| objective/indicator (1) | list→ObjectiveIndicatorList |
| objective/key_result (2) | create→ObjectiveKeyResultCreate, list→ObjectiveKeyResultList |
| objective (key_results_*) (2) | position→ObjectiveKeyResultsPosition, weight→ObjectiveKeyResultsWeight |
| objective/progress (1) | list→ObjectiveProgressList |

### D3: 统一迁移形态 `let path = OkrApiV2::Variant(self.id).to_url();`
统一用 `path` 变量名以最小化 diff（多数叶子原有变量名就是 `path`）。
- 单 id 叶：`format!("/open-apis/okr/v2/objectives/{}", self.objective_id)` → `OkrApiV2::ObjectiveGet(self.objective_id).to_url()`
- 多行 format! 叶（objectives_position 等）：同形，`self.cycle_id` / `self.objective_id` / `self.key_result_id` move 进 variant
- 字符串字面量叶（`category/list`、`cycle/list`）：当前 URL 内联在 `ApiRequest::get("/open-apis/okr/v2/...")`，改为先 `let path = OkrApiV2::Variant.to_url();` 再 `ApiRequest::get(&path)`（`cycle/list` 还需兼顾其后续 `.query(...)` 链式调用——`ApiRequest::get(&path).query(...)`）

### D4: 不动 Potemkin 叶子测试
叶子的 `test_url_path` 是 Potemkin 测试（建 `_req` 丢弃、assert 只比 `format!` 宏与硬编码串，**不调生产 execute()**），迁移前后都测不出生产 URL 变化。**本 change 不动它们**——修需架构改动（提取 URL accessor 供测试观测，或引入 mock transport），属另案。Design Doc 标为已知局限。

### D5: 补 `OkrApiV2` enum `to_url()` 25 variant 测试（迁移安全网）
`api_endpoints.rs::test_okr_api_urls` 当前只测 `OkrApiV1::PeriodList`，`OkrApiV2` 25 variant **零覆盖**。enum 现不被调用，`to_url()` 若潜伏笔误是**休眠 bug**，迁移后变活 bug。补全 25 variant 断言（alignment 2 / category 1 / cycle 6 / indicator 1 / key_result 5 / objective 11，含带 id 的 variant 用 `"123"` 之类的占位 id），锁定映射。**迁移前先补**，使其作为各批次迁移的回归基线。

**替代方案**：不加测试、靠手工核对 enum 与 inline 串——拒绝，休眠 bug 无自动兜底，违 CLAUDE.md #5（真验证）。

## Risks / Trade-offs

- **[Risk] enum `to_url()` 串与 inline 串存在不可见差异 / 休眠笔误**（如 `key_result_position` 少 s、尾斜杠、占位符拼写——enum 现不被调用，潜伏笔误是休眠 bug，迁移后变活 bug）→ Mitigation：D5 补的 25 variant `to_url()` 测试迁移前先落地，逐字节断言每个 variant 输出；迁移后该测试 + `cargo test -p openlark-hr` 全过即证明 enum 串正确。已抽样核对 ObjectiveKeyResultsPosition / KeyResultProgressList / CycleObjectivesWeight 与 inline 串一致，当前无休眠 bug。
- **[Risk] 漏迁某叶**（25 叶分散在 6 资源子树）→ Mitigation：验收用 `grep -rn 'open-apis/okr/v2' crates/openlark-hr/src/okr/okr/v2/`，仅应命中 enum `to_url()` 实现（在 `common/api_endpoints.rs`，不在叶目录）+ Potemkin 测试期望值；叶目录内生产构造零命中。
- **[Risk] Potemkin 叶子测试不动，给人「URL 已测」错觉**→ Mitigation：Design Doc 显式标注局限；未来另案提取 URL accessor 或引入 mock transport 才能真正测生产 URL。本 change 不扩大范围。
- **[Trade-off] enum 当前 25 variant 中有同 URL 不同 variant 对**（CycleObjectiveCreate/List、ObjectiveAlignmentCreate/List、ObjectiveKeyResultCreate/List）——语义冗余但属既有设计，本 change 不收敛（另案），仅复用。

## Migration Plan

**第 0 步（前置）**：补 `api_endpoints.rs::test_okr_api_urls` 的 25 variant `to_url()` 断言，build + test 通过，commit。此为后续各批次的回归基线。

按 6 资源批次推进（每批 build + test 一次，commit 一次）：
1. alignment (2) + category (1) + cycle/list (1) —— 含两类字符串字面量形态（category/list、cycle/list），先做以验证 D3 全部形态（cycle/list 兼顾 `.query(...)` 链式）
```

Full source: openspec/changes/okr-v2-endpoint-enum/design.md

## openspec/changes/okr-v2-endpoint-enum/tasks.md

- Source: openspec/changes/okr-v2-endpoint-enum/tasks.md
- Lines: 1-53
- SHA256: 1f198b009bba72783d8a96de015b55a6be053f0174de9d4e69b4d7ec179b66d0

```md
# Tasks — okr-v2-endpoint-enum

按 design.md 推进：先补 enum 测试（回归基线），再 6 资源批次迁移。每批：迁移叶子 → `cargo build + test -p openlark-hr --all-features` → commit。`OkrApiV2` enum 本体无需改动（variant 齐、`to_url()` 正确）。

## 0. 前置：补 enum to_url 测试（迁移安全网）

- [ ] 0.1 在 `crates/openlark-hr/src/common/api_endpoints.rs` 的 `test_okr_api_urls` 补全 `OkrApiV2` 25 variant `to_url()` 断言（带 id 的 variant 用占位 id 如 `"123"`）
- [ ] 0.2 `cargo test -p openlark-hr --all-features` 通过；commit（回归基线）

## 1. 批次 A：alignment + category + cycle/list（验证全部迁移形态）

- [ ] 1.1 迁移 `alignment/get.rs`、`alignment/delete.rs`：`format!(...)` → `OkrApiV2::AlignmentGet/Delete(self.alignment_id).to_url()`
- [ ] 1.2 迁移 `category/list.rs`、`cycle/list.rs`：`ApiRequest::get("/open-apis/okr/v2/...")` → `let path = OkrApiV2::CategoryList/CycleList.to_url();` 后 `ApiRequest::get(&path)`（cycle/list 兼顾后续 `.query(...)` 链式）
- [ ] 1.3 `cargo build + test -p openlark-hr --all-features` 通过；commit

## 2. 批次 B：cycle/objective + cycle 多行（4 叶）

- [ ] 2.1 迁移 `cycle/objective/create.rs`、`cycle/objective/list.rs` → `OkrApiV2::CycleObjectiveCreate/List(self.cycle_id)`
- [ ] 2.2 迁移 `cycle/objectives_position.rs`、`cycle/objectives_weight.rs`（多行 format!）→ `OkrApiV2::CycleObjectivesPosition/Weight(self.cycle_id)`
- [ ] 2.3 build + test 通过；commit

## 3. 批次 C：indicator + key_result 主（4 叶）

- [ ] 3.1 迁移 `indicator/patch.rs` → `OkrApiV2::IndicatorPatch(self.indicator_id)`
- [ ] 3.2 迁移 `key_result/{delete,get,patch}.rs` → `OkrApiV2::KeyResultDelete/Get/Patch(self.key_result_id)`
- [ ] 3.3 build + test 通过；commit

## 4. 批次 D：key_result 子树（2 叶）

- [ ] 4.1 迁移 `key_result/indicator/list.rs` → `OkrApiV2::KeyResultIndicatorList(self.key_result_id)`
- [ ] 4.2 迁移 `key_result/progress/list.rs` → `OkrApiV2::KeyResultProgressList(self.key_result_id)`
- [ ] 4.3 build + test 通过；commit

## 5. 批次 E：objective 主 + key_results_*（5 叶）

- [ ] 5.1 迁移 `objective/{delete,get,patch}.rs` → `OkrApiV2::ObjectiveDelete/Get/Patch(self.objective_id)`
- [ ] 5.2 迁移 `objective/key_results_position.rs`、`objective/key_results_weight.rs` → `OkrApiV2::ObjectiveKeyResultsPosition/Weight(self.objective_id)`
- [ ] 5.3 build + test 通过；commit

## 6. 批次 F：objective 子树（6 叶）

- [ ] 6.1 迁移 `objective/alignment/{create,list}.rs` → `OkrApiV2::ObjectiveAlignmentCreate/List(self.objective_id)`
- [ ] 6.2 迁移 `objective/indicator/list.rs` → `OkrApiV2::ObjectiveIndicatorList(self.objective_id)`
- [ ] 6.3 迁移 `objective/key_result/{create,list}.rs` → `OkrApiV2::ObjectiveKeyResultCreate/List(self.objective_id)`
- [ ] 6.4 迁移 `objective/progress/list.rs` → `OkrApiV2::ObjectiveProgressList(self.objective_id)`
- [ ] 6.5 build + test 通过；commit

## 7. 全量验收

- [ ] 7.1 `grep -rn 'open-apis/okr/v2' crates/openlark-hr/src/okr/okr/v2/` 仅命中测试期望值，叶目录生产构造零命中
- [ ] 7.2 `cargo clippy -p openlark-hr --all-features --all-targets` 零警告
- [ ] 7.3 `cargo fmt --check` 通过
- [ ] 7.4 `cargo build + test -p openlark-hr --all-features` 全过；`OkrApiV2` enum 与 `to_url()` 实现 diff 为零
```

## openspec/changes/okr-v2-endpoint-enum/specs/v1-sub-api-accessors/spec.md

- Source: openspec/changes/okr-v2-endpoint-enum/specs/v1-sub-api-accessors/spec.md
- Lines: 1-32
- SHA256: 25692616f0350c0788f8a27013a75f28a74252eae80b8c1a951c4d20ac6a1d96

```md
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
```

