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
2. cycle/objective 子树 (2：create+list) + cycle 多行 (2：objectives_position+weight)
3. indicator (1) + key_result 主 (3：delete/get/patch)
4. key_result/indicator+progress (2)
5. objective 主 (3：delete/get/patch) + key_results_position+weight (2)
6. objective 子树 (alignment 2 + indicator 1 + key_result 2 + progress 1)

**回滚**：每批独立 commit，任一批 test 失败即 `git revert` 该批，不影响他批。整体非 breaking，无需特别发布窗口。

## Open Questions

无。enum variant 已齐、URL 串由 D5 新测试保证、迁移形态明确。可在 build 阶段直接落地。
