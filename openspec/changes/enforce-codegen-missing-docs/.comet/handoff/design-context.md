# Comet Design Handoff

- Change: enforce-codegen-missing-docs
- Phase: design
- Mode: compact
- Context hash: 495c60d2e9dd12dfc6694fb541ab8caa8871e17cae6d30b6fd26cae2e0c959f3

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/enforce-codegen-missing-docs/proposal.md

- Source: openspec/changes/enforce-codegen-missing-docs/proposal.md
- Lines: 1-34
- SHA256: 5564dfc5b26c524761c332d0e1e5a63c14b73b65ce25118221b48572cd716cab

```md
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
```

## openspec/changes/enforce-codegen-missing-docs/design.md

- Source: openspec/changes/enforce-codegen-missing-docs/design.md
- Lines: 1-57
- SHA256: 645b82bfc2f9302bc08dcf55ea5c66db6c9fca6debf119239257a6c0a3954053

```md
## Context

`tools/codegen.py` 的 `run_closed_loop`（生成后 fmt+clippy 验证）带 `-A missing_docs`（codegen.py:185），让生成代码绕过 missing_docs。这与 workspace/CI 的 missing_docs 治理不一致，且兜底掩盖 `_field_lines` 对无 description 字段不生成 doc 的缺口——是 #273 治理的源头风险（只要 `-A` 在，codegen 持续可能产出缺 doc 代码）。

**勘探事实（已确认）**：
- codegen.py 198 行薄编排器；生成逻辑在 `tools/api_contracts/codegen_render.py`（461 行，风格 A）。
- 当前 codegen 已生成大部分 doc：`_module_doc`（`//!`+docPath）、`_render_struct`（struct 级 `///`）、`_field_lines`（字段级 `///`，**仅当 `field.description` 存在**）、`_request_impl`（`new`/`_setter_doc`/`execute` 方法 doc）。
- request struct 字段是私有的（`config: Config` 等），无 missing_docs 风险；pub 字段只在 response/body/nested struct（经 `_field_lines`）。
- analytics 122 缺口是遗留（文件无 codegen 标记），非当前 codegen 产物；#1 已回补。本 change 不动业务 crate。
- 现状 workspace `cargo doc --workspace --all-features` = 0 missing_docs（含 communication 等 codegen 目标 crate），说明已提交的生成代码 doc-complete。

## Goals / Non-Goals

**Goals**：移除 codegen 闭环的 `-A missing_docs`；给 `_field_lines` 加 description 缺失的 fallback doc；验证移除后生成闭环仍通过。

**Non-Goals**：不重新生成/回退任何业务 crate 源码；不改 codegen 风格 B；不改 schema 解析（`codegen_ir`）；不治理 schema description 质量（fallback 兜底即可，源端治理另案）；不动 ci.yml。

## Decisions

### D1: 移除 `-A missing_docs`（codegen.py:185）
`run_closed_loop` 的 clippy 命令从 `... -- -Dwarnings -A missing_docs` 改为 `... -- -Dwarnings`。一行改动。

### D2: `_field_lines` fallback doc（codegen_render.py）
当前：`if field.description: out.append(f"    /// {_oneliner(field.description)}")`。改为：始终生成 doc——有 description 用 `_oneliner(description)`，无则用 `field.rust_name`：
```python
doc = _oneliner(field.description) if field.description else field.rust_name
out.append(f"    /// {doc}")
```
fallback 用字段真实名（`/// user_id`），与 #1「引用真实名、非占位符」原则一致，机械不误导。

**Alternative**：要求 schema 必须填 description（源端治理）——否决，超 codegen 范畴，且现有大量 schema 字段无 description，阻塞面太大。

### D3: 验证策略——不覆盖真实 crate 文件
codegen 生成会覆盖手动改动（codegen marker 警示）。验证移除 `-A` 后生成闭环通过，MUST 用安全方式：
- 优先：codegen 现有单元测试（`tools/api_contracts/` 或 `tools/tests/`）跑 `_field_lines` / `render_api_file` 的断言（更新 golden 输出含 fallback doc）。
- 实跑验证：在一个**测试夹具或临时目标**上跑 codegen 生成 + clippy（无 `-A`），不覆盖 communication 真实文件。若必须实跑，生成后 `git checkout` 还原。
- 确保现有 `api-contracts` CI job 通过。

### D4: build 执行方式
本 change 改动小（2 个 tools/ 文件，~5 行），逻辑清晰，build_mode 可用 executing-plans（直接执行）或 subagent-driven。改动量小、无并行单元，倾向 executing-plans/direct（build 阶段 plan-ready 暂停由用户选定）。

## Risks / Trade-offs

- **[移除 -A 暴露其它无 doc 的 pub 项]** → 当前勘探确认 struct/方法已生成 doc，pub 字段是唯一风险点（D2 兜底）。build 阶段实测：若 codegen 还生成 enum 或其它 pub 项，补对应 doc 生成。范围可控。
- **[fallback doc 质量]** → 字段名兜底是最小有效 doc（非占位符、不误导）；语义 doc 依赖 schema description 质量提升，属另案。可接受。
- **[codegen golden 测试需更新]** → 若 codegen 有 golden-file 断言，fallback 改动会改变输出，需同步更新 golden。build 阶段处理。
- **[实跑 codegen 覆盖手动改动]** → D3 验证策略明确不覆盖真实 crate 文件（用夹具/临时 + git checkout 还原）。

## Migration Plan

纯增量、非破坏性，无 API/数据迁移。回滚 = revert。顺序：先加 fallback doc（D2，使生成代码 doc-complete）→ 移除 -A（D1）→ 验证（D3）。

## Open Questions

- codegen 是否生成 enum 或其它 pub 项（除 struct/字段/方法外）？→ build 阶段实测 `_render_*` 全集 + 实跑生成。
- codegen 测试基础设施位置（`tools/api_contracts/` tests？golden files？）→ build 阶段勘探。
- build 执行方式（executing-plans vs subagent-driven）→ build 阶段 plan-ready 暂停由用户选定（D4 倾向 executing-plans）。
```

## openspec/changes/enforce-codegen-missing-docs/tasks.md

- Source: openspec/changes/enforce-codegen-missing-docs/tasks.md
- Lines: 1-25
- SHA256: 69e121f4276fc2d9321345107b0b216ddaa1d1c74964af2a04d60346f776e71c

```md
## 1. 勘探与确认

- [ ] 1.1 确认 codegen_render 的 pub 项全集：已勘探仅 struct 字段（经 `_field_lines`）需 fallback；无 `pub enum`/`pub type`/`pub const`（endpoint const 在 endpoints/，[MANUAL] 不自动生成）。
- [ ] 1.2 检查 `tools/api_contracts/test_codegen_render.py` 是否有 `_field_lines` / `render_api_file` 的 golden 断言需随 fallback 同步（无 description 字段的预期输出）。

## 2. D2：`_field_lines` fallback doc

- [ ] 2.1 修改 `tools/api_contracts/codegen_render.py` 的 `_field_lines`：`description` 缺失时生成 `/// {field.rust_name}`（字段名兜底），确保每个 pub 字段都有 doc。
- [ ] 2.2 自验：构造一个无 `description` 的 FieldDef，确认 `_field_lines` 输出含 `/// <field_name>`。

## 3. D1：移除 `-A missing_docs`

- [ ] 3.1 修改 `tools/codegen.py:185` `run_closed_loop` 的 clippy 命令：移除 `-A missing_docs`（`-- -Dwarnings` 结尾）。

## 4. 测试同步

- [ ] 4.1 若 1.2 发现 golden 断言，更新 `test_codegen_render.py` 对应预期输出（含 fallback doc）。
- [ ] 4.2 跑 `python3 -m pytest tools/api_contracts/test_codegen_render.py tools/api_contracts/test_codegen_ir.py tools/api_contracts/test_mod_tree.py`（或等价）通过。

## 5. 验证

- [ ] 5.1 **实跑生成闭环**：在测试夹具或临时目标上跑 codegen 生成一个 API（含无 description 字段）+ `cargo clippy -p <crate> --all-targets --all-features -- -Dwarnings`（无 `-A`）exit 0；**不覆盖真实业务 crate 文件**（生成后 `git checkout` 还原或用临时目录）。
- [ ] 5.2 `cargo fmt --check` + `just lint`（codegen 改动属 tools/，确认 workspace lint 不破）通过。
- [ ] 5.3 确认 `api-contracts` CI 相关本地可跑的验证（`python3 tools/validate_api_contracts.py` 等）不破。
- [ ] 5.4 占位符守门：fallback doc 是字段真实名（非 `待补充文档` 等占位符）。
```

## openspec/changes/enforce-codegen-missing-docs/specs/codegen-missing-docs/spec.md

- Source: openspec/changes/enforce-codegen-missing-docs/specs/codegen-missing-docs/spec.md
- Lines: 1-29
- SHA256: 4abf9377957b820a35f9fc428a86089e70510eafc6d27471d8d8c807a5247865

```md
## ADDED Requirements

### Requirement: codegen 生成后验证闭环 MUST NOT 绕过 missing_docs

`tools/codegen.py` 的生成后验证闭环（`run_closed_loop`，对刚生成的代码跑 fmt + clippy）MUST 与 workspace/CI 的 missing_docs 治理一致——其 clippy 调用 MUST NOT 含 `-A missing_docs`（或任何放过 missing_docs 的 `-A` 标志）。这消除 codegen 工具维度「生成时绕过、仓库级强制」的执行不一致，确保 codegen 持续产出符合 workspace missing_docs 标准的代码，不让 `-A` 兜底掩盖生成代码的文档缺口。

#### Scenario: run_closed_loop clippy 不含 -A missing_docs

- **WHEN** 检查 `tools/codegen.py` 的 `run_closed_loop` 函数中 clippy 命令的参数
- **THEN** MUST NOT 出现 `-A missing_docs`（命令以 `-- -Dwarnings` 结尾，不额外放过 missing_docs）

#### Scenario: 生成的代码在无 -A 的 clippy 下通过

- **WHEN** codegen 生成一个 API 文件后，对目标 crate 跑 `cargo clippy -p <crate> --all-targets --all-features -- -Dwarnings`（无 `-A missing_docs`）
- **THEN** MUST exit 0（生成代码 doc-complete，不依赖 `-A` 兜底）

### Requirement: codegen 生成的 pub 字段 MUST 有 doc（含 fallback）

`tools/api_contracts/codegen_render.py` 渲染 struct 字段时，每个 pub 字段 MUST 带 `///` doc。当 schema 字段有 `description` 时用其作为 doc；当 `description` 缺失时 MUST 生成 fallback doc（`/// {field.rust_name}`，引用字段真实名），不得留空。此约束保证移除 `-A missing_docs` 后生成代码仍 doc-complete，从源头杜绝 #273 治理项 #1 那样的累积缺口。

#### Scenario: 有 description 的字段生成语义 doc

- **WHEN** codegen 渲染一个 schema 中带 `description` 的 pub 字段
- **THEN** 生成的 `.rs` 中该字段 MUST 有 `/// {_oneliner(description)}` 形式的 doc

#### Scenario: 无 description 的字段生成 fallback doc

- **WHEN** codegen 渲染一个 schema 中 `description` 为空的 pub 字段（`field.rust_name` = `user_id`）
- **THEN** 生成的 `.rs` 中该字段 MUST 有 `/// user_id` 形式的 fallback doc（引用字段真实名），MUST NOT 无 doc
```

