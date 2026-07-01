# Brainstorm Summary

- Change: fix-analytics-missing-docs
- Date: 2026-07-01

## 确认的技术方案

**Doc recipe（核心）**：每项 doc = `<文件 //! 标题>` + `<item 角色>`。
- Request struct → `/// <标题>请求。`
- Response struct → `/// <标题>响应。`
- Response field → `/// <字段含义>。`（如 `/// 响应数据。`）
- `new` → `/// 创建新的请求构建器。`
- `execute` → `/// 执行<标题>请求。`（**docPath 只留文件级 `//!`，不在方法里重复**——决策 A）
- `execute_with_options` → `/// 使用指定请求选项执行<标题>请求。`
- builder 方法 → `/// <动作>。`

工作样例（`search/v2/schema/create.rs`，标题="创建数据范式"，6 warning→6 doc）已验证贴合。

**执行策略（决策 B）**：
1. Pilot：先回补 `schema/create.rs` 一个文件，实跑 recipe，审查产出符合规范。
2. 批量：剩余 ~17 文件按域分 3-4 组 subagent 并行（per-domain 粒度：search/data_source 组、search/rest 组、report 组）。
3. 每 subagent 自验：`cargo doc -p openlark-analytics --all-features 2>&1 | grep <它的文件>` 无 warning。
4. 主会话收尾：移除 lib.rs:36 allow + 补 `doc_wiki/search.rs` 空 docPath + CI 接线 + 全局验证。

## 关键取舍与风险

- **docPath 只留文件级**（A）：更简洁无冗余，牺牲与 communication crate 的方法级 docPath 一致性。可接受——文件级 `//!` 已是 rustdoc 导航主体。
- **per-domain 而非 per-file**（B）：3-4 组 vs 18 subagent，调度/审查开销低，域内 coherent。牺牲极限并行度。
- **test #1 进 CI**：接受 `cargo test --workspace --all-features --no-run` 成本（~1-2min），因它是唯一 rustdoc 层 missing_docs 守门（clippy/build 不触发）。CI 复用编译产物。
- **[122 项规模诱发占位符]** → D2 grep 守门（`待补充文档|公开项说明`）+ pilot 质量前置验证。
- **[误报]**：已确认无——trait impl（`data_format`）不被 missing_docs 标记（docs 从 trait 继承），122 全是合法公开 API。

## 测试策略

- 逐文件：`cargo doc -p openlark-analytics --all-features 2>&1 | grep <file>` 无 warning（subagent 自验）。
- 全局：`cargo doc --workspace --all-features` missing_docs=0；`cargo doc -p openlark-analytics --all-features` 0 警告。
- 行为测试：3 个 `test_workspace_missing_docs.py` 测试全绿（含 test#1 workspace 级 + test#2 扫描 + test#3 item 级 allowlist）。
- 占位符守门：`grep -rnE '待补充文档|公开项说明' crates/openlark-analytics/src/` 为空。
- 回归：`cargo test -p openlark-analytics` 现有测试不破；`just lint`（已强制 missing_docs）通过。

## Spec Patch

无。delta spec（`lint-execution-consistency`：MODIFIED 移除 analytics 豁免 + ADDED missing_docs 测试 MUST 在 CI）的验收场景已覆盖本设计。doc recipe/质量属设计层细节，非 spec 层行为，无需回写。
