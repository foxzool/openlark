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
