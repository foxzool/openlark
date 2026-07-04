# Brainstorm Summary

- Change: okr-v2-endpoint-enum
- Date: 2026-07-04

## 确认的技术方案

okr/v2 **25 叶**端点 URL 从 inline `format!`(23)/字符串字面量(category/list 1)/inline 字面量(cycle/list 生产行 1) 统一到 `OkrApiV2::Variant(self.id).to_url()`。`OkrApiV2` enum 25 variant 已齐备、`to_url()` 正确，**无需新增 variant**。

迁移形态：
- id 叶：`format!("/open-apis/okr/v2/objectives/{}", self.objective_id)` → `let path = OkrApiV2::ObjectiveGet(self.objective_id).to_url();`
- category/list / cycle/list：URL 内联在 `ApiRequest::get(...)`，改为 `let path = OkrApiV2::Variant.to_url();` 再 `ApiRequest::get(&path)`
- `execute(self)` 消费 self，`self.xxx_id` 直接 move 进 variant，无需 `.clone()`

## brainstorming 三轮深挖发现（对 open 产物的修正）

1. **scope 修正 #1（open 阶段已确认）**：issue 称 12 叶是低估，实测由 inline format!/字面量构成。
2. **scope 修正 #2（brainstorming 发现）**：cycle/list **生产代码**（`execute_with_options` 行 72）也是 inline `"/open-apis/okr/v2/cycles"`，仅其**测试**用了 enum。故待迁移 = **25 叶**（非 24）。
3. **测试覆盖真相**：
   - `OkrApiV2` enum `to_url()` **零测试**（`test_okr_api_urls` 只覆盖 `OkrApiV1::PeriodList`）
   - 叶子 `test_url_path` 是 **Potemkin**：`let _req = ...; ` 建完丢弃，assert 只比 `format!(...)` 与硬编码串，**不调生产 execute()**
   - 即生产 URL 构造的实际测试覆盖 ≈ 0
4. **核心风险**：enum 现不被调用，`to_url()` 潜伏笔误是**休眠 bug**，迁移后变活 bug，现有测试抓不到。已抽样核对 enum 与 inline 串一致（KeyResultsPosition/KeyResultProgressList/CycleObjectivesWeight 等），当前无休眠 bug。

## 关键取舍与风险

- **补 enum to_url 测试（已确认采纳）**：在 `api_endpoints.rs::test_okr_api_urls` 补全 25 variant to_url 断言（~25 行），作迁移唯一可靠安全网。小幅扩 scope 但 CLAUDE.md #5 要求真验证。
- **不动 Potemkin 叶子测试**：修它需架构改动（提取 URL accessor 供测试观测），越界，属另案。Design Doc 标为已知局限。
- **enum variant 同 URL 不同名对**（CycleObjectiveCreate/List、ObjectiveAlignmentCreate/List、ObjectiveKeyResultCreate/List）属既有语义分层，本 change 不收敛。
- [Risk] enum `to_url()` 休眠笔误 → Mitigation：补的 25 variant 测试 + 抽样核对
- [Risk] 漏迁某叶 → Mitigation：`grep -rn 'open-apis/okr/v2' crates/openlark-hr/src/okr/okr/v2/` 仅命中 enum 实现（在 common/）+ Potemkin 测试期望值，叶目录生产构造零命中
- [Trade-off] URL 路径串逐字节不变 → 非breaking，无发布窗口

## 测试策略

1. **新增**：`api_endpoints.rs::test_okr_api_urls` 补 25 variant `to_url()` 断言（迁移前先加，锁定映射）
2. **沿用**：`cargo build/test -p openlark-hr --all-features` + clippy + `cargo fmt --check`
3. **验收 grep**：叶目录生产代码零 inline URL
4. **不动**：Potemkin `test_url_path`（既有，另案）

## Spec Patch（回写 OpenSpec）

- proposal.md / design.md / tasks.md：「24 叶」「cycle/list 已迁」→ **25 叶**；cycle/list 加入批次 A
- design.md Risk：「cycle_123 断言兜底」→「enum to_url 测试 + grep」；新增 D5 测试策略决策
- specs/v1-sub-api-accessors/spec.md：scenario「24 叶全部经 enum」→「25 叶」；新增 scenario「OkrApiV2 to_url 测试覆盖全 25 variant」
