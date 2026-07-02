## MODIFIED Requirements

### Requirement: missing_docs 验证测试 MUST 在 CI 运行

missing_docs 验证测试 MUST 在 CI（`.github/workflows/ci.yml`）执行，不得作为只在本地存在、CI 不跑的“死测试”。CI 须覆盖两层：

1. **workspace 级**（`tools/tests/test_workspace_missing_docs.py`）：workspace 无 missing_docs 警告、无 crate 级 missing_docs 抑制、item 级抑制仅限 protocol 生成模块。
2. **crate 特定结构变体**（`tools/tests/test_openlark_*_missing_docs.py` 的 `do_not_suppress` / `mod_roots` / `cleaned_slices` / `v1_root` 等方法）：各 crate 硬编码的“已清理文件/模块根”回归守卫，断言这些位置无 `#![allow(missing_docs)]` 回归。

此约束消除虚假强制感，确保 missing_docs 治理（含 crate 级 `allow` outlier 与各 crate 历史清理契约的回归）被 CI 持续守门。

> **冗余测试已删**：per-crate `has_no_missing_docs_warnings`（跑 `cargo test -p <crate> --no-run`）与 workspace 级测试完全冗余（workspace `--all-features` 编译已覆盖全部 crate 的 missing_docs），故删除、不得再加回。workspace 级测试是“无 missing_docs 警告”断言的单一来源。

#### Scenario: workspace 级 missing_docs 测试在 CI 执行

- **WHEN** 检查 `.github/workflows/ci.yml`
- **THEN** MUST 包含执行 `tools/tests/test_workspace_missing_docs.py`（覆盖其全部测试方法）的步骤，与已有的 `test_check_mod_reachability` 同级运行

#### Scenario: crate 级 allow 回归被 CI 捕获

- **WHEN** 有人重新向任一 crate 引入 `#![allow(missing_docs)]`
- **THEN** CI 执行的 `test_workspace_source_files_do_not_use_crate_level_missing_docs_suppressions` MUST 失败，阻断合入

#### Scenario: crate 特定结构变体在 CI 执行

- **WHEN** 检查 `.github/workflows/ci.yml`
- **THEN** MUST 包含执行 10 个 crate 特定结构变体测试（`test_openlark_{communication,docs,helpdesk,hr,mail,meeting,platform,protocol,workflow}_missing_docs` + `test_openlark_workflow_narrow_missing_docs`）的步骤

#### Scenario: 无冗余 per-crate 编译测试

- **WHEN** 检查 `tools/tests/test_openlark_*_missing_docs.py`
- **THEN** MUST NOT 存在 `has_no_missing_docs_warnings` 方法（per-crate cargo 编译断言，已被 workspace 级 `test_workspace_has_no_missing_docs_warnings` 覆盖，冗余故删除）
