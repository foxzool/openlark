---
comet_change: okr-v2-endpoint-enum
role: technical-design
canonical_spec: openspec
---

# Technical Design — okr-v2-endpoint-enum

> Canonical spec: `openspec/changes/okr-v2-endpoint-enum/`（proposal/design/specs/tasks）。本 Doc 记录 brainstorming 深挖出的技术决策与测试策略，不重复需求 spec。

## 1. 背景与目标

把 okr/v2 **25 个叶子**端点 URL 构造统一到 `OkrApiV2` enum 的 `.to_url()`，消除 inline `format!`（23 叶）与字符串字面量（`category/list`、`cycle/list` 2 叶）。`OkrApiV2` enum 25 variant 已齐备、`to_url()` 正确，纯调用点迁移，URL 路径串逐字节不变（非 breaking）。

## 2. brainstorming 三轮深挖（对 open 产物的事实修正）

### 2.1 scope 修正：25 叶（非 issue 字面的 12，亦非 open 草稿的 24）
- issue #338 窄 grep 漏数多行 `format!(` → 实测 inline 叶远不止 12
- open 草稿算出 24 叶（误把 `cycle/list` 当已迁）→ brainstorming 发现 `cycle/list` **生产代码**（`execute_with_options` 行 72）也是 inline `"/open-apis/okr/v2/cycles"`，仅其**测试**用了 enum → 待迁移 = **25 叶**

### 2.2 测试覆盖真相（关键）
| 层 | 现状 | 问题 |
|---|---|---|
| `OkrApiV2` enum `to_url()` | **零测试**（`test_okr_api_urls` 只覆盖 `OkrApiV1::PeriodList`） | variant→URL 映射未锁定 |
| 叶子 `test_url_path` | **Potemkin**：`let _req = Request::new(...).cycle_id("cycle_123");` 建完丢弃，`assert_eq!(format!(...), "硬编码串")` 只测 `format!` 宏 | **不调生产 execute()**，测不出生产 URL |
| 生产 enum 调用 | **0 处** | enum 形同 Potemkin |

**结论：生产 URL 构造的实际测试覆盖 ≈ 0。** open 草稿 Risk 段「cycle_123 断言兜底」前提错误，已回写改正。

### 2.3 核心风险：休眠 bug
enum 现不被调用，`to_url()` 若潜伏笔误（如 `key_result_position` 少 s）是**休眠 bug**——迁移后变活 bug，而现有测试（enum 无测试 + 叶子 Potemkin）都抓不到。已抽样核对 ObjectiveKeyResultsPosition / KeyResultProgressList / CycleObjectivesWeight 与 inline 串一致，**当前无休眠 bug**，但迁移必须锁住映射。

## 3. 决策

- **D1 全量 25 叶**：含 cycle/list 生产。半量留漏网叶，违契约。
- **D2 复用现有 enum**：25 叶→variant 映射全覆盖（见 openspec design.md 表），含 3 对同 URL 不同 variant（Create/List 语义分层）属既有设计，不收敛。
- **D3 统一形态** `let path = OkrApiV2::Variant(self.id).to_url();`；`execute(self)` 消费 self，`self.xxx_id` move 进 variant 无需 clone；category/list、cycle/list 先 `let path` 再 `ApiRequest::get(&path)`，cycle/list 保留 `.query(...)` 链式。
- **D4 不动 Potemkin 叶子测试**：修需架构改动（提取 URL accessor / mock transport），越界另案。
- **D5 补 enum `to_url()` 25 variant 测试**：迁移**前置**落地，作回归基线与休眠 bug 安全网。小幅扩 scope 但 CLAUDE.md #5 要求真验证。

## 4. 测试策略

1. **新增（D5）**：`api_endpoints.rs::test_okr_api_urls` 补 25 variant `to_url()` 断言，迁移前先 commit。
2. **沿用**：每批 `cargo build + test -p openlark-hr --all-features` + 末尾 `cargo clippy --all-targets` + `cargo fmt --check`。
3. **验收 grep**：`grep -rn 'open-apis/okr/v2' crates/openlark-hr/src/okr/okr/v2/` 仅命中 Potemkin 测试期望值，叶目录生产构造零命中。
4. **不动**：Potemkin `test_url_path`（既有，另案）。

## 5. 迁移顺序

第 0 步：补 enum 测试（基线）。批次 A~F：A=alignment+category+cycle/list（验证全部形态）→ B=cycle/objective+cycle 多行 → C=indicator+key_result 主 → D=key_result 子树 → E=objective 主+key_results_* → F=objective 子树。每批独立 commit，失败即 revert 该批。

## 6. 已知局限（另案）

- 叶子 `test_url_path` 是 Potemkin，给人「URL 已测」错觉。真正测生产 URL 需提取 URL accessor（`pub(crate) fn url(&self) -> String`）或引入 mock transport，属独立 change，不在本范围。
- enum 3 对同 URL 不同 variant（Create/List）语义冗余，收敛另案。

## 7. Spec Patch（已回写 OpenSpec）

- proposal/design/tasks：「24 叶 / cycle/list 已迁」→ **25 叶 / cycle/list 待迁并进批次 A**
- design Risk：删「cycle_123 断言兜底」→ 改「D5 enum 测试 + grep」；新增 D5
- spec delta：scenario 24→25；新增「OkrApiV2 to_url 测试覆盖全 25 variant」scenario；category/list scenario 扩成 category+cycle/list
