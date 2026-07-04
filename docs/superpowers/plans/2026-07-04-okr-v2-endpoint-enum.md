---
change: okr-v2-endpoint-enum
design-doc: docs/superpowers/specs/2026-07-04-okr-v2-endpoint-enum-design.md
base-ref: c1c32a5dc
---

# okr-v2-endpoint-enum 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 把 `crates/openlark-hr/src/okr/okr/v2/` 下 25 个叶子的端点 URL 构造从 inline `format!`（23 叶）/ 字符串字面量（`category/list`、`cycle/list` 生产 2 叶）统一改为 `OkrApiV2::Variant(self.id).to_url()`，URL 路径串逐字节不变。

**Architecture:** 纯调用点迁移。`OkrApiV2` enum（`crates/openlark-hr/src/common/api_endpoints.rs:1941`）25 variant 已齐备、`to_url()` 已正确实现，**无需新增 variant、无需改 enum 本体**。`execute(self)` 消费 self，`self.xxx_id` 为 owned `String`，直接 move 进 variant，无需 `.clone()`。先补 enum `to_url()` 25 variant 测试作为回归基线（D5），再按 6 资源批次迁移，每批独立 commit。

**Tech Stack:** Rust, openlark-core `ApiRequest`（`get(url: impl Into<String>)`），openlark-hr `common::api_endpoints::OkrApiV2`。

## Global Constraints

- **URL 串逐字节不变**：仅改构造方式，路径前后完全相同（非 breaking）。
- **统一形态（D3）**：单 id 叶 `let path = OkrApiV2::Variant(self.id).to_url();`；字符串字面量叶先 `let path = OkrApiV2::Variant.to_url();` 再 `ApiRequest::get(&path)`（cycle/list 保留 `.query(...)` 链式）。
- **不动 Potemkin 叶子测试（D4）**：叶子内 `test_url_path` 不调生产 execute()，本 change 不改。
- **不动 typed Response、不动 enum 本体、不动导航链。**
- **enum import 路径**：`use crate::common::api_endpoints::OkrApiV2;`（每个迁移的生产叶子文件需在 import 区新增此行；与既有 `use crate::okr::okr::v2::common::models::...` 并列，alphabetically `common::api_endpoints` 在前）。
- **每批完成后必跑**：`cargo build + cargo test -p openlark-hr --all-features`，全过后 commit。
- **回滚**：任一批 test 失败即 `git revert` 该批 commit，不影响他批。

## File Structure

**Modify:**
- `crates/openlark-hr/src/common/api_endpoints.rs` —— 仅 Task 0 改 `test_okr_api_urls`（补 25 variant 断言）；enum 本体零改动。
- 25 个叶子文件（均位于 `crates/openlark-hr/src/okr/okr/v2/` 下，详见各 Task）—— 每个叶子改 2 处：① import 区加 `use crate::common::api_endpoints::OkrApiV2;` ② `execute_with_options` 内 URL 构造由 inline 改为 enum `.to_url()`。

**No create / no delete.** 纯编辑现有文件。

## 关键事实速查（已代码核实）

- `ApiRequest::get/post/put/delete(url: impl Into<String>)`（`crates/openlark-core/src/api/mod.rs:117`）—— `path`（move）与 `&path` 均编译通过。
- `api_endpoints.rs` 的 `#[cfg(test)] mod tests { use super::*; ... }`（行 3425）—— `OkrApiV2` 已通过 `super::*` 进入测试作用域，**无需在测试内额外 use**。
- 25 variant → URL 映射见 `crates/openlark-hr/src/common/api_endpoints.rs:2005-2092`（`impl OkrApiV2 { pub fn to_url(&self) -> String }`）。
- 现有单 id 叶模板（以 `alignment/get.rs:51-52` 为准）：
  ```rust
  let path = format!("/open-apis/okr/v2/alignments/{}", self.alignment_id);
  let req: ApiRequest<GetAlignmentResponse> = ApiRequest::get(path);
  ```
- 现有多行 format! 叶模板（以 `cycle/objectives_position.rs:62-70` 为准）：
  ```rust
  let path = format!(
      "/open-apis/okr/v2/cycles/{}/objectives_position",
      self.cycle_id
  );
  let req: ApiRequest<...> = ApiRequest::put(path).body(body_val);
  ```
- 现有字符串字面量叶模板（`category/list.rs:37`、`cycle/list.rs:71-72`）：
  ```rust
  // category/list.rs
  let req: ApiRequest<ListCategoryResponse> = ApiRequest::get("/open-apis/okr/v2/categories");
  // cycle/list.rs
  let mut req: ApiRequest<ListCycleResponse> =
      ApiRequest::get("/open-apis/okr/v2/cycles").query("user_id", &self.user_id);
  ```

---

## Task 0: 补 OkrApiV2 enum 25 variant to_url() 测试（迁移安全网 / D5）

**Files:**
- Modify: `crates/openlark-hr/src/common/api_endpoints.rs:3464-3468`（`test_okr_api_urls` 函数体）

**Why:** enum 当前零测试覆盖，`to_url()` 若潜伏笔误是休眠 bug，迁移后变活 bug。先补全 25 variant 断言锁定映射，作各批次回归基线。**迁移前置**。

- [x] **Step 1: 在 `test_okr_api_urls` 内补 25 variant `to_url()` 断言**

把 `test_okr_api_urls` 函数体（行 3465-3468）从：

```rust
    fn test_okr_api_urls() {
        let url = OkrApiV1::PeriodList.to_url();
        assert_eq!(url, "/open-apis/okr/v1/periods");
    }
```

改为（保留既有 `OkrApiV1::PeriodList` 断言，新增 25 个 `OkrApiV2` variant 断言，带 id 的 variant 用占位 id `"123"`）：

```rust
    fn test_okr_api_urls() {
        let url = OkrApiV1::PeriodList.to_url();
        assert_eq!(url, "/open-apis/okr/v1/periods");

        // OkrApiV2 — alignment (2)
        assert_eq!(
            OkrApiV2::AlignmentDelete("123".to_string()).to_url(),
            "/open-apis/okr/v2/alignments/123"
        );
        assert_eq!(
            OkrApiV2::AlignmentGet("123".to_string()).to_url(),
            "/open-apis/okr/v2/alignments/123"
        );

        // category (1)
        assert_eq!(
            OkrApiV2::CategoryList.to_url(),
            "/open-apis/okr/v2/categories"
        );

        // cycle (5)
        assert_eq!(OkrApiV2::CycleList.to_url(), "/open-apis/okr/v2/cycles");
        assert_eq!(
            OkrApiV2::CycleObjectivesPosition("123".to_string()).to_url(),
            "/open-apis/okr/v2/cycles/123/objectives_position"
        );
        assert_eq!(
            OkrApiV2::CycleObjectivesWeight("123".to_string()).to_url(),
            "/open-apis/okr/v2/cycles/123/objectives_weight"
        );
        assert_eq!(
            OkrApiV2::CycleObjectiveCreate("123".to_string()).to_url(),
            "/open-apis/okr/v2/cycles/123/objectives"
        );
        assert_eq!(
            OkrApiV2::CycleObjectiveList("123".to_string()).to_url(),
            "/open-apis/okr/v2/cycles/123/objectives"
        );

        // indicator (1)
        assert_eq!(
            OkrApiV2::IndicatorPatch("123".to_string()).to_url(),
            "/open-apis/okr/v2/indicators/123"
        );

        // key_result (5)
        assert_eq!(
            OkrApiV2::KeyResultDelete("123".to_string()).to_url(),
            "/open-apis/okr/v2/key_results/123"
        );
        assert_eq!(
            OkrApiV2::KeyResultGet("123".to_string()).to_url(),
            "/open-apis/okr/v2/key_results/123"
        );
        assert_eq!(
            OkrApiV2::KeyResultPatch("123".to_string()).to_url(),
            "/open-apis/okr/v2/key_results/123"
        );
        assert_eq!(
            OkrApiV2::KeyResultIndicatorList("123".to_string()).to_url(),
            "/open-apis/okr/v2/key_results/123/indicators"
        );
        assert_eq!(
            OkrApiV2::KeyResultProgressList("123".to_string()).to_url(),
            "/open-apis/okr/v2/key_results/123/progresses"
        );

        // objective (11)
        assert_eq!(
            OkrApiV2::ObjectiveDelete("123".to_string()).to_url(),
            "/open-apis/okr/v2/objectives/123"
        );
        assert_eq!(
            OkrApiV2::ObjectiveGet("123".to_string()).to_url(),
            "/open-apis/okr/v2/objectives/123"
        );
        assert_eq!(
            OkrApiV2::ObjectivePatch("123".to_string()).to_url(),
            "/open-apis/okr/v2/objectives/123"
        );
        assert_eq!(
            OkrApiV2::ObjectiveKeyResultsPosition("123".to_string()).to_url(),
            "/open-apis/okr/v2/objectives/123/key_results_position"
        );
        assert_eq!(
            OkrApiV2::ObjectiveKeyResultsWeight("123".to_string()).to_url(),
            "/open-apis/okr/v2/objectives/123/key_results_weight"
        );
        assert_eq!(
            OkrApiV2::ObjectiveAlignmentCreate("123".to_string()).to_url(),
            "/open-apis/okr/v2/objectives/123/alignments"
        );
        assert_eq!(
            OkrApiV2::ObjectiveAlignmentList("123".to_string()).to_url(),
            "/open-apis/okr/v2/objectives/123/alignments"
        );
        assert_eq!(
            OkrApiV2::ObjectiveIndicatorList("123".to_string()).to_url(),
            "/open-apis/okr/v2/objectives/123/indicators"
        );
        assert_eq!(
            OkrApiV2::ObjectiveKeyResultCreate("123".to_string()).to_url(),
            "/open-apis/okr/v2/objectives/123/key_results"
        );
        assert_eq!(
            OkrApiV2::ObjectiveKeyResultList("123".to_string()).to_url(),
            "/open-apis/okr/v2/objectives/123/key_results"
        );
        assert_eq!(
            OkrApiV2::ObjectiveProgressList("123".to_string()).to_url(),
            "/open-apis/okr/v2/objectives/123/progresses"
        );
    }
```

- [x] **Step 2: 运行测试验证 25 variant 断言全过（回归基线）**

Run: `cargo test -p openlark-hr --all-features test_okr_api_urls`
Expected: PASS（25 个 `OkrApiV2` 断言 + 1 个 `OkrApiV1` 断言全过）。若任一 variant 断言失败，说明 enum `to_url()` 有休眠 bug——停止迁移，先修 enum（但已抽样核对无休眠 bug，应一次通过）。

- [x] **Step 3: 提交基线**

```bash
git add crates/openlark-hr/src/common/api_endpoints.rs
git commit -m "test(hr/okr): 补 OkrApiV2 25 variant to_url() 断言（okr-v2-endpoint-enum 迁移安全网 / D5）"
```

---

## Task 1: 批次 A — alignment(2) + category/list + cycle/list（验证全部迁移形态）

**Files:**
- Modify: `crates/openlark-hr/src/okr/okr/v2/alignment/get.rs`
- Modify: `crates/openlark-hr/src/okr/okr/v2/alignment/delete.rs`
- Modify: `crates/openlark-hr/src/okr/okr/v2/category/list.rs`
- Modify: `crates/openlark-hr/src/okr/okr/v2/cycle/list.rs`

**Why:** 先验证 D3 全部三种迁移形态（单 id `format!` / 字符串字面量 / 字符串字面量 + `.query()` 链式）。

- [x] **Step 1: 迁移 alignment/get.rs（单 id format! 形态）**

1. import 区（既有 `use crate::okr::okr::v2::common::models::Alignment;` 上方）新增：
   ```rust
   use crate::common::api_endpoints::OkrApiV2;
   ```
2. `execute_with_options` 内行 51 由：
   ```rust
   let path = format!("/open-apis/okr/v2/alignments/{}", self.alignment_id);
   ```
   改为：
   ```rust
   let path = OkrApiV2::AlignmentGet(self.alignment_id).to_url();
   ```
   （下一行 `let req: ApiRequest<GetAlignmentResponse> = ApiRequest::get(path);` 不动。）

- [x] **Step 2: 迁移 alignment/delete.rs（单 id format! 形态）**

1. import 区新增 `use crate::common::api_endpoints::OkrApiV2;`（位置同上）。
2. `execute_with_options` 内（行 49）由：
   ```rust
   let path = format!("/open-apis/okr/v2/alignments/{}", self.alignment_id);
   ```
   改为：
   ```rust
   let path = OkrApiV2::AlignmentDelete(self.alignment_id).to_url();
   ```

- [x] **Step 3: 迁移 category/list.rs（字符串字面量形态）**

1. import 区新增（该文件当前无 `use crate::...` 组，紧随 `use std::sync::Arc;` 后加一组空行 + 导入）：
   ```rust

   use crate::common::api_endpoints::OkrApiV2;
   ```
2. `execute_with_options` 内行 37 由：
   ```rust
   let req: ApiRequest<ListCategoryResponse> = ApiRequest::get("/open-apis/okr/v2/categories");
   ```
   改为（先 `let path` 再 `ApiRequest::get(&path)`，遵循 D3）：
   ```rust
   let path = OkrApiV2::CategoryList.to_url();
   let req: ApiRequest<ListCategoryResponse> = ApiRequest::get(&path);
   ```

- [x] **Step 4: 迁移 cycle/list.rs（字符串字面量 + .query() 链式形态）**

1. import 区新增（该文件当前无 `use crate::...` 组，紧随 `use std::sync::Arc;` 后加一组空行 + 导入）：
   ```rust

   use crate::common::api_endpoints::OkrApiV2;
   ```
2. `execute_with_options` 内行 71-72 由：
   ```rust
   let mut req: ApiRequest<ListCycleResponse> =
       ApiRequest::get("/open-apis/okr/v2/cycles").query("user_id", &self.user_id);
   ```
   改为（保留 `.query("user_id", ...)` 链式）：
   ```rust
   let path = OkrApiV2::CycleList.to_url();
   let mut req: ApiRequest<ListCycleResponse> =
       ApiRequest::get(&path).query("user_id", &self.user_id);
   ```
   注：该文件 `#[cfg(test)]` 内 `test_url_construction`（行 174-179）已用 enum——**不动测试**。

- [x] **Step 5: build + test 验证批次 A**

Run: `cargo build -p openlark-hr --all-features && cargo test -p openlark-hr --all-features`
Expected: BUILD SUCCESS + 全部测试 PASS（含 Task 0 的 25 variant 断言 + 4 叶子各自的反序列化测试）。

- [x] **Step 6: 提交批次 A**

```bash
git add crates/openlark-hr/src/okr/okr/v2/alignment/get.rs \
        crates/openlark-hr/src/okr/okr/v2/alignment/delete.rs \
        crates/openlark-hr/src/okr/okr/v2/category/list.rs \
        crates/openlark-hr/src/okr/okr/v2/cycle/list.rs
git commit -m "refactor(hr/okr): okr/v2 批次A alignment+category/list+cycle/list 迁移到 OkrApiV2 enum（okr-v2-endpoint-enum）"
```

---

## Task 2: 批次 B — cycle/objective(create+list) + cycle/objectives_position + cycle/objectives_weight（4 叶，多行 format!）

**Files:**
- Modify: `crates/openlark-hr/src/okr/okr/v2/cycle/objective/create.rs`
- Modify: `crates/openlark-hr/src/okr/okr/v2/cycle/objective/list.rs`
- Modify: `crates/openlark-hr/src/okr/okr/v2/cycle/objectives_position.rs`
- Modify: `crates/openlark-hr/src/okr/okr/v2/cycle/objectives_weight.rs`

- [x] **Step 1: 迁移 cycle/objective/create.rs（单行 format! / cycle_id）**

1. import 区新增 `use crate::common::api_endpoints::OkrApiV2;`。
2. 行 57 由：
   ```rust
   let path = format!("/open-apis/okr/v2/cycles/{}/objectives", self.cycle_id);
   ```
   改为：
   ```rust
   let path = OkrApiV2::CycleObjectiveCreate(self.cycle_id).to_url();
   ```
   （测试内行 96-98 的 `format!(...)` 期望值断言**不动**——它是 Potemkin 测试期望值。）

- [x] **Step 2: 迁移 cycle/objective/list.rs（单行 format! / cycle_id）**

1. import 区新增 `use crate::common::api_endpoints::OkrApiV2;`。
2. 行 51 由：
   ```rust
   let path = format!("/open-apis/okr/v2/cycles/{}/objectives", self.cycle_id);
   ```
   改为：
   ```rust
   let path = OkrApiV2::CycleObjectiveList(self.cycle_id).to_url();
   ```

- [x] **Step 3: 迁移 cycle/objectives_position.rs（多行 format! / cycle_id）**

1. import 区新增 `use crate::common::api_endpoints::OkrApiV2;`（紧随既有 `use crate::okr::okr::v2::common::models::Objective;` 上方）。
2. 行 62-65 由：
   ```rust
   let path = format!(
       "/open-apis/okr/v2/cycles/{}/objectives_position",
       self.cycle_id
   );
   ```
   改为：
   ```rust
   let path = OkrApiV2::CycleObjectivesPosition(self.cycle_id).to_url();
   ```
   （测试内行 108-111 的 `format!(...)` 期望值断言**不动**。）

- [x] **Step 4: 迁移 cycle/objectives_weight.rs（多行 format! / cycle_id）**

1. import 区新增 `use crate::common::api_endpoints::OkrApiV2;`。
2. 行 63-66 由：
   ```rust
   let path = format!(
       "/open-apis/okr/v2/cycles/{}/objectives_weight",
       self.cycle_id
   );
   ```
   改为：
   ```rust
   let path = OkrApiV2::CycleObjectivesWeight(self.cycle_id).to_url();
   ```

- [x] **Step 5: build + test 验证批次 B**

Run: `cargo build -p openlark-hr --all-features && cargo test -p openlark-hr --all-features`
Expected: BUILD SUCCESS + 全部测试 PASS。

- [x] **Step 6: 提交批次 B**

```bash
git add crates/openlark-hr/src/okr/okr/v2/cycle/objective/create.rs \
        crates/openlark-hr/src/okr/okr/v2/cycle/objective/list.rs \
        crates/openlark-hr/src/okr/okr/v2/cycle/objectives_position.rs \
        crates/openlark-hr/src/okr/okr/v2/cycle/objectives_weight.rs
git commit -m "refactor(hr/okr): okr/v2 批次B cycle/objective + cycle/objectives_position|weight 迁移到 OkrApiV2 enum（okr-v2-endpoint-enum）"
```

---

## Task 3: 批次 C — indicator/patch + key_result(delete+get+patch)（4 叶）

**Files:**
- Modify: `crates/openlark-hr/src/okr/okr/v2/indicator/patch.rs`
- Modify: `crates/openlark-hr/src/okr/okr/v2/key_result/delete.rs`
- Modify: `crates/openlark-hr/src/okr/okr/v2/key_result/get.rs`
- Modify: `crates/openlark-hr/src/okr/okr/v2/key_result/patch.rs`

- [x] **Step 1: 迁移 indicator/patch.rs（单行 format! / indicator_id）**

1. import 区新增 `use crate::common::api_endpoints::OkrApiV2;`。
2. 行 59 由：
   ```rust
   let path = format!("/open-apis/okr/v2/indicators/{}", self.indicator_id);
   ```
   改为：
   ```rust
   let path = OkrApiV2::IndicatorPatch(self.indicator_id).to_url();
   ```

- [x] **Step 2: 迁移 key_result/delete.rs（单行 format! / key_result_id）**

1. import 区新增 `use crate::common::api_endpoints::OkrApiV2;`。
2. 行 49 由：
   ```rust
   let path = format!("/open-apis/okr/v2/key_results/{}", self.key_result_id);
   ```
   改为：
   ```rust
   let path = OkrApiV2::KeyResultDelete(self.key_result_id).to_url();
   ```

- [x] **Step 3: 迁移 key_result/get.rs（单行 format! / key_result_id）**

1. import 区新增 `use crate::common::api_endpoints::OkrApiV2;`。
2. 行 51 由：
   ```rust
   let path = format!("/open-apis/okr/v2/key_results/{}", self.key_result_id);
   ```
   改为：
   ```rust
   let path = OkrApiV2::KeyResultGet(self.key_result_id).to_url();
   ```

- [x] **Step 4: 迁移 key_result/patch.rs（单行 format! / key_result_id）**

1. import 区新增 `use crate::common::api_endpoints::OkrApiV2;`。
2. 行 59 由：
   ```rust
   let path = format!("/open-apis/okr/v2/key_results/{}", self.key_result_id);
   ```
   改为：
   ```rust
   let path = OkrApiV2::KeyResultPatch(self.key_result_id).to_url();
   ```

- [x] **Step 5: build + test 验证批次 C**

Run: `cargo build -p openlark-hr --all-features && cargo test -p openlark-hr --all-features`
Expected: BUILD SUCCESS + 全部测试 PASS。

- [x] **Step 6: 提交批次 C**

```bash
git add crates/openlark-hr/src/okr/okr/v2/indicator/patch.rs \
        crates/openlark-hr/src/okr/okr/v2/key_result/delete.rs \
        crates/openlark-hr/src/okr/okr/v2/key_result/get.rs \
        crates/openlark-hr/src/okr/okr/v2/key_result/patch.rs
git commit -m "refactor(hr/okr): okr/v2 批次C indicator/patch + key_result delete/get/patch 迁移到 OkrApiV2 enum（okr-v2-endpoint-enum）"
```

---

## Task 4: 批次 D — key_result/indicator/list + key_result/progress/list（2 叶，多行 format!）

**Files:**
- Modify: `crates/openlark-hr/src/okr/okr/v2/key_result/indicator/list.rs`
- Modify: `crates/openlark-hr/src/okr/okr/v2/key_result/progress/list.rs`

- [x] **Step 1: 迁移 key_result/indicator/list.rs（多行 format! / key_result_id）**

1. import 区新增 `use crate::common::api_endpoints::OkrApiV2;`。
2. 行 52-55 由：
   ```rust
   let path = format!(
       "/open-apis/okr/v2/key_results/{}/indicators",
       self.key_result_id
   );
   ```
   改为：
   ```rust
   let path = OkrApiV2::KeyResultIndicatorList(self.key_result_id).to_url();
   ```

- [x] **Step 2: 迁移 key_result/progress/list.rs（多行 format! / key_result_id）**

1. import 区新增 `use crate::common::api_endpoints::OkrApiV2;`。
2. 行 50-53 由：
   ```rust
   let path = format!(
       "/open-apis/okr/v2/key_results/{}/progresses",
       self.key_result_id
   );
   ```
   改为：
   ```rust
   let path = OkrApiV2::KeyResultProgressList(self.key_result_id).to_url();
   ```

- [x] **Step 3: build + test 验证批次 D**

Run: `cargo build -p openlark-hr --all-features && cargo test -p openlark-hr --all-features`
Expected: BUILD SUCCESS + 全部测试 PASS。

- [x] **Step 4: 提交批次 D**

```bash
git add crates/openlark-hr/src/okr/okr/v2/key_result/indicator/list.rs \
        crates/openlark-hr/src/okr/okr/v2/key_result/progress/list.rs
git commit -m "refactor(hr/okr): okr/v2 批次D key_result/indicator/list + key_result/progress/list 迁移到 OkrApiV2 enum（okr-v2-endpoint-enum）"
```

---

## Task 5: 批次 E — objective(delete+get+patch) + objective/key_results_position + objective/key_results_weight（5 叶）

**Files:**
- Modify: `crates/openlark-hr/src/okr/okr/v2/objective/delete.rs`
- Modify: `crates/openlark-hr/src/okr/okr/v2/objective/get.rs`
- Modify: `crates/openlark-hr/src/okr/okr/v2/objective/patch.rs`
- Modify: `crates/openlark-hr/src/okr/okr/v2/objective/key_results_position.rs`
- Modify: `crates/openlark-hr/src/okr/okr/v2/objective/key_results_weight.rs`

- [x] **Step 1: 迁移 objective/delete.rs（单行 format! / objective_id）**

1. import 区新增 `use crate::common::api_endpoints::OkrApiV2;`。
2. 行 49 由：
   ```rust
   let path = format!("/open-apis/okr/v2/objectives/{}", self.objective_id);
   ```
   改为：
   ```rust
   let path = OkrApiV2::ObjectiveDelete(self.objective_id).to_url();
   ```

- [x] **Step 2: 迁移 objective/get.rs（单行 format! / objective_id）**

1. import 区新增 `use crate::common::api_endpoints::OkrApiV2;`。
2. 行 51 由：
   ```rust
   let path = format!("/open-apis/okr/v2/objectives/{}", self.objective_id);
   ```
   改为：
   ```rust
   let path = OkrApiV2::ObjectiveGet(self.objective_id).to_url();
   ```

- [x] **Step 3: 迁移 objective/patch.rs（单行 format! / objective_id）**

1. import 区新增 `use crate::common::api_endpoints::OkrApiV2;`。
2. 行 59 由：
   ```rust
   let path = format!("/open-apis/okr/v2/objectives/{}", self.objective_id);
   ```
   改为：
   ```rust
   let path = OkrApiV2::ObjectivePatch(self.objective_id).to_url();
   ```

- [x] **Step 4: 迁移 objective/key_results_position.rs（多行 format! / objective_id）**

1. import 区新增 `use crate::common::api_endpoints::OkrApiV2;`。
2. 行 62-65 由：
   ```rust
   let path = format!(
       "/open-apis/okr/v2/objectives/{}/key_results_position",
       self.objective_id
   );
   ```
   改为：
   ```rust
   let path = OkrApiV2::ObjectiveKeyResultsPosition(self.objective_id).to_url();
   ```

- [x] **Step 5: 迁移 objective/key_results_weight.rs（多行 format! / objective_id）**

1. import 区新增 `use crate::common::api_endpoints::OkrApiV2;`。
2. 行 63-66 由：
   ```rust
   let path = format!(
       "/open-apis/okr/v2/objectives/{}/key_results_weight",
       self.objective_id
   );
   ```
   改为：
   ```rust
   let path = OkrApiV2::ObjectiveKeyResultsWeight(self.objective_id).to_url();
   ```

- [x] **Step 6: build + test 验证批次 E**

Run: `cargo build -p openlark-hr --all-features && cargo test -p openlark-hr --all-features`
Expected: BUILD SUCCESS + 全部测试 PASS。

- [x] **Step 7: 提交批次 E**

```bash
git add crates/openlark-hr/src/okr/okr/v2/objective/delete.rs \
        crates/openlark-hr/src/okr/okr/v2/objective/get.rs \
        crates/openlark-hr/src/okr/okr/v2/objective/patch.rs \
        crates/openlark-hr/src/okr/okr/v2/objective/key_results_position.rs \
        crates/openlark-hr/src/okr/okr/v2/objective/key_results_weight.rs
git commit -m "refactor(hr/okr): okr/v2 批次E objective delete/get/patch + key_results_position/weight 迁移到 OkrApiV2 enum（okr-v2-endpoint-enum）"
```

---

## Task 6: 批次 F — objective/alignment(create+list) + objective/indicator/list + objective/key_result(create+list) + objective/progress/list（6 叶）

**Files:**
- Modify: `crates/openlark-hr/src/okr/okr/v2/objective/alignment/create.rs`
- Modify: `crates/openlark-hr/src/okr/okr/v2/objective/alignment/list.rs`
- Modify: `crates/openlark-hr/src/okr/okr/v2/objective/indicator/list.rs`
- Modify: `crates/openlark-hr/src/okr/okr/v2/objective/key_result/create.rs`
- Modify: `crates/openlark-hr/src/okr/okr/v2/objective/key_result/list.rs`
- Modify: `crates/openlark-hr/src/okr/okr/v2/objective/progress/list.rs`

- [x] **Step 1: 迁移 objective/alignment/create.rs（多行 format! / objective_id）**

1. import 区新增 `use crate::common::api_endpoints::OkrApiV2;`。
2. 行 60-63 由：
   ```rust
   let path = format!(
       "/open-apis/okr/v2/objectives/{}/alignments",
       self.objective_id
   );
   ```
   改为：
   ```rust
   let path = OkrApiV2::ObjectiveAlignmentCreate(self.objective_id).to_url();
   ```

- [x] **Step 2: 迁移 objective/alignment/list.rs（多行 format! / objective_id）**

1. import 区新增 `use crate::common::api_endpoints::OkrApiV2;`。
2. 行 52-55 由：
   ```rust
   let path = format!(
       "/open-apis/okr/v2/objectives/{}/alignments",
       self.objective_id
   );
   ```
   改为：
   ```rust
   let path = OkrApiV2::ObjectiveAlignmentList(self.objective_id).to_url();
   ```

- [x] **Step 3: 迁移 objective/indicator/list.rs（多行 format! / objective_id）**

1. import 区新增 `use crate::common::api_endpoints::OkrApiV2;`。
2. 行 52-55 由：
   ```rust
   let path = format!(
       "/open-apis/okr/v2/objectives/{}/indicators",
       self.objective_id
   );
   ```
   改为：
   ```rust
   let path = OkrApiV2::ObjectiveIndicatorList(self.objective_id).to_url();
   ```

- [x] **Step 4: 迁移 objective/key_result/create.rs（多行 format! / objective_id）**

1. import 区新增 `use crate::common::api_endpoints::OkrApiV2;`。
2. 行 61-64 由：
   ```rust
   let path = format!(
       "/open-apis/okr/v2/objectives/{}/key_results",
       self.objective_id
   );
   ```
   改为：
   ```rust
   let path = OkrApiV2::ObjectiveKeyResultCreate(self.objective_id).to_url();
   ```

- [x] **Step 5: 迁移 objective/key_result/list.rs（多行 format! / objective_id）**

1. import 区新增 `use crate::common::api_endpoints::OkrApiV2;`。
2. 行 52-55 由：
   ```rust
   let path = format!(
       "/open-apis/okr/v2/objectives/{}/key_results",
       self.objective_id
   );
   ```
   改为：
   ```rust
   let path = OkrApiV2::ObjectiveKeyResultList(self.objective_id).to_url();
   ```

- [x] **Step 6: 迁移 objective/progress/list.rs（多行 format! / objective_id）**

1. import 区新增 `use crate::common::api_endpoints::OkrApiV2;`。
2. 行 50-53 由：
   ```rust
   let path = format!(
       "/open-apis/okr/v2/objectives/{}/progresses",
       self.objective_id
   );
   ```
   改为：
   ```rust
   let path = OkrApiV2::ObjectiveProgressList(self.objective_id).to_url();
   ```

- [x] **Step 7: build + test 验证批次 F**

Run: `cargo build -p openlark-hr --all-features && cargo test -p openlark-hr --all-features`
Expected: BUILD SUCCESS + 全部测试 PASS。至此 25 叶全部迁移完成。

- [x] **Step 8: 提交批次 F**

```bash
git add crates/openlark-hr/src/okr/okr/v2/objective/alignment/create.rs \
        crates/openlark-hr/src/okr/okr/v2/objective/alignment/list.rs \
        crates/openlark-hr/src/okr/okr/v2/objective/indicator/list.rs \
        crates/openlark-hr/src/okr/okr/v2/objective/key_result/create.rs \
        crates/openlark-hr/src/okr/okr/v2/objective/key_result/list.rs \
        crates/openlark-hr/src/okr/okr/v2/objective/progress/list.rs
git commit -m "refactor(hr/okr): okr/v2 批次F objective 子树 alignment/indicator/key_result/progress 迁移到 OkrApiV2 enum（okr-v2-endpoint-enum）"
```

---

## Task 7: 全量验收

- [x] **Step 1: grep 验证叶目录生产构造零命中**

Run: `grep -rn 'open-apis/okr/v2' crates/openlark-hr/src/okr/okr/v2/`
Expected: 仅命中（a）各叶子 `#[cfg(test)]` 内的 Potemkin `format!(...)` / 硬编码期望值断言，以及（b）`cycle/list.rs:177` 已有的 enum `to_url()` 测试断言；**叶目录 `execute_with_options` 生产构造零命中**（无 `format!("/open-apis/okr/v2`，无 `ApiRequest::get("/open-apis/okr/v2`）。

- [x] **Step 2: 验证 enum 本体 diff 为零**

Run: `git diff c1c32a5dc -- crates/openlark-hr/src/common/api_endpoints.rs | grep -E '^\+|^-' | grep -v '^[+-]\s*//\|^[+-]\s*$' | grep -vE '^\+\s*assert_eq|^\+\s*OkrApiV2::|^\+\s*/open-apis|^\+\s*\}|^\+\s*\)|^\+\s*,|^\+\s*let url|^-$'`
Expected: 除 Task 0 新增的 `test_okr_api_urls` 测试断言行外，**enum 定义（行 1941-2003）与 `to_url()` 实现（行 2005-2093）零 diff**。

- [x] **Step 3: clippy 零警告**

Run: `cargo clippy -p openlark-hr --all-features --all-targets`
Expected: 无 warning、无 error。

- [x] **Step 4: fmt --check 通过**

Run: `cargo fmt --check`
Expected: 无输出（退出码 0）。若不通过，跑 `cargo fmt` 后重新检查并 amend 最后一个 commit。

- [x] **Step 5: build + test 全量通过**

Run: `cargo build -p openlark-hr --all-features && cargo test -p openlark-hr --all-features`
Expected: BUILD SUCCESS + 全部测试 PASS（含 Task 0 的 25 variant `to_url()` 断言）。

- [x] **Step 6: 全量回归（workspace 级，保险）**

Run: `cargo build --workspace --all-features && cargo test --workspace --all-features`
Expected: 全部 PASS（确认无跨 crate 回归）。

---

## Self-Review 备注

- **Spec 覆盖**：delta spec 5 个 scenario 全覆盖——①25 叶经 enum（Task 1-6）②URL 串逐字节不变（Task 0 enum 测试 + 各批 build/test）③category/list 与 cycle/list 字符串字面量收敛（Task 1 Step 3-4）④enum 本体不变（Task 7 Step 2）⑤to_url 测试覆盖 25 variant（Task 0）。
- **休眠 bug 安全网**：Task 0 的 25 variant 断言在 Task 1 之前落地，任一 variant `to_url()` 笔误会在 Task 0 Step 2 即暴露，不会带到迁移。
- **回滚粒度**：6 个批次各自独立 commit，失败即 `git revert <commit>`，互不影响。
