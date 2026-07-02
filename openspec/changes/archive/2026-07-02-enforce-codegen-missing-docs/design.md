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
