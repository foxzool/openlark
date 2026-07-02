## Why

`tools/codegen.py` 的 `run_closed_loop`（生成后 fmt+clippy 验证）在 clippy 调用里带 `-A missing_docs`（codegen.py:185）。这让 codegen 产出的代码绕过 missing_docs 强制——与 workspace/CI 的 missing_docs 治理（#273 系列：workspace 无 `-A`、crate 级 allow outlier 已清）形成**执行不一致**。更关键的是它是**源头风险**：`codegen_render._field_lines` 仅在 schema 字段有 `description` 时生成 `///` doc，无 description 的 pub 字段就不带 doc，靠 `-A` 兜底。只要 `-A` 在，codegen 就持续可能产出缺 doc 的代码，稀释 #273 已完成的治理（如 #1 analytics 回补的 122 项）。现在收口这个生成闭环。

**勘探事实**：
- codegen.py 仅 198 行薄编排器；生成逻辑在 `tools/api_contracts/codegen_render.py`（461 行，风格 A）。
- 当前 codegen **已生成大部分 doc**：`//!` 模块 + docPath、struct 级 `///`、字段级 `///`（当 `field.description` 存在）、`new`/setter/`execute` 方法 doc。
- analytics 的 122 缺口是**遗留/老生成器**产物（文件无 codegen 标记），非当前 codegen 造成，#1 已手工回补——本 change 不动业务 crate。
- 因此本 change 范围聚焦：移除 vestigial `-A` 安全网 + 给 description 缺失字段加 fallback doc，属小中改。

## What Changes

- **移除** `tools/codegen.py:185` `run_closed_loop` clippy 命令的 `-A missing_docs`（生成后验证真强制 missing_docs，对齐 workspace 标准）。
- **加 fallback doc**：`tools/api_contracts/codegen_render.py` 的 `_field_lines`，当 `field.description` 为空时生成 `/// {field.rust_name}`（字段名兜底，与 #1「引用真实名」原则一致），确保每个 pub 字段都有 doc。
- **验证**：codegen 对测试 API 的生成 + clippy（不带 `-A`）通过；现有 codegen 测试不破；现有 communication（codegen 目标 crate）的生成输出仍 0 missing_docs。

**非破坏性**：仅 codegen 工具链（2 个 `tools/` 文件），不动任何业务 crate 源码、不改公开 API。

## Capabilities

### New Capabilities

- `codegen-missing-docs`: codegen 工具链 missing_docs 闭环——生成代码 MUST doc-complete（含 description 缺失字段的兜底 doc），生成后验证闭环 MUST 不用 `-A missing_docs` 绕过强制。

### Modified Capabilities

（无——本 change 是 codegen 工具维度，独立于 `lint-execution-consistency` 的 workspace lint 命令维度。）

## Impact

- **工具代码**：`tools/codegen.py`（-1 flag）、`tools/api_contracts/codegen_render.py`（`_field_lines` fallback，+几行）。
- **CI**：codegen 相关验证（`api-contracts` job）会跑改后的 codegen；不改 ci.yml。
- **Spec**：新建 `openspec/specs/codegen-missing-docs/spec.md`（delta，归档时进主 spec）。
- **业务 crate / 依赖 / 公开 API**：无变更。
