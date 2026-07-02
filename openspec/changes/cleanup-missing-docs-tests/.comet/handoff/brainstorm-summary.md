# Brainstorm Summary

- Change: cleanup-missing-docs-tests
- Date: 2026-07-02

## 确认的技术方案

**D1 删冗余 has_no_warnings**：
- 删 8 整文件（仅含 has_no_warnings）：ai/analytics/application/auth/cardkit/client/core/webhook。
- 9 文件删 `has_no_warnings` 方法 **+ 同步删 `import subprocess`**（参照 workflow_narrow 无 has_no_warnings 时的 import 形态）：communication/docs/helpdesk/hr/mail/meeting/platform/protocol/workflow。
- 保留 workflow_narrow 不动（本就只有结构变体）。

**D2 接 10 结构变体进 CI**：ci.yml 现有 run block（ci.yml:112-115）加 `python3 -m unittest \` + 10 个 backslash 续行模块（沿用 ci.yml:69-72 api-contracts 多模块模式）。

## 关键取舍与风险

- **删除安全（point 1）**：workspace `--all-features` 编译是最严格超集（统一 features + 全 crate），per-crate 是子集 → workspace 通过必致 per-crate 通过；feature unification 只让 workspace 更宽。删除安全，#1 的 workspace 测试持续守门。
- **import 清理（point 2）**：删 has_no_warnings 后 `import subprocess` 变 unused，须同步删（参照 workflow_narrow：无 subprocess）。9 文件改 = 删方法 + 删 import。
- **ci.yml 模式（point 3）**：用 backslash 续行列 10 模块（api-contracts 既有模式），非单行长行，可读 + 一致。
- **[结构变体硬编码路径过期]** → 现全绿（0.27s），路径当前有效；接 CI 后若文件移动致路径失效会变红提示更新——这正是接 CI 的价值。

## 测试策略

- D2 验证：本地 + CI 跑 10 结构变体模块全绿。
- 删除安全：workspace `cargo doc --workspace --all-features` 仍 0 missing_docs（#1 守门不变）。
- 回归：`cargo fmt --check` + `just lint`（测试文件改动不影响 Rust lint）；yaml 合法。
- 无残留死测试：`ls tools/tests/test_openlark_*_missing_docs.py | wc -l` = 10；`grep has_no_missing_docs_warnings tools/tests/` 空。

## Spec Patch

无。delta spec（lint-execution-consistency MODIFIED：扩展 CI 测试两层 + 场景声明 has_no_warnings 冗余已删）已覆盖。
