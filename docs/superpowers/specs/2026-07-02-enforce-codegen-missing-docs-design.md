---
comet_change: enforce-codegen-missing-docs
role: technical-design
canonical_spec: openspec
---

# Design: enforce-codegen-missing-docs

> #273 missing_docs 深度治理子项 #4。承接子项 #1（PR #293，analytics 122 项 doc 回补——遗留代码，非当前 codegen 产物）。本 change 收口 codegen 工具链的 missing_docs 闭环：移除生成后验证的 `-A missing_docs` 安全网 + 给无 description 字段加 fallback doc。
>
> Canonical spec：`openspec/changes/enforce-codegen-missing-docs/specs/codegen-missing-docs/spec.md`（delta，新 capability）。

## 1. Context

`tools/codegen.py` 的 `run_closed_loop`（生成后 fmt+clippy 验证）带 `-A missing_docs`（codegen.py:185），让生成代码绕过 missing_docs。这与 workspace/CI 治理不一致，且兜底掩盖 `_field_lines` 对无 description 字段不生成 doc 的缺口——只要 `-A` 在，codegen 持续可能产出缺 doc 代码。

**勘探事实（已确认）**：
- codegen.py 198 行薄编排器；生成逻辑在 `tools/api_contracts/codegen_render.py`（461 行，风格 A）。
- 当前 codegen 已生成大部分 doc：`_module_doc`（`//!`+docPath）、`_render_struct`（struct 级 `///`）、`_field_lines`（字段级 `///`，**仅当 `field.description` 存在**）、`_request_impl`（`new`/`_setter_doc`/`execute` 方法 doc）。
- **pub 项全集**：struct 字段（`_field_lines`，response/body/nested）、request struct + 方法（已 doc）。`_tests` 是 `#[cfg(test)]` 私有 mod（无 pub）；`_render_api_response_trait` 是 trait impl（doc 继承）；无 enum 渲染；endpoint const 在 endpoints/（[MANUAL]，不自动进 .rs）。→ **唯一 doc 缺口 = 无 description 字段**。
- analytics 122 缺口是遗留（文件无 codegen 标记），非当前 codegen 产物，#1 已回补。workspace `cargo doc --workspace --all-features` 现 0 missing_docs（含 communication 等 codegen 目标）。
- `test_codegen_render.py`（327 行）用 **inline `assertIn`/`assertNotIn`**（非 golden file）；fixture 字段都带 description（如 `"description": "接收者"`）。

## 2. 目标 / 非目标

**目标**：移除 codegen 闭环 `-A missing_docs`；给 `_field_lines` 加 fallback doc；验证移除后闭环仍通过。

**非目标**：不重新生成/回退业务 crate 源码；不改 codegen 风格 B；不改 schema 解析（`codegen_ir`）；不治理 schema description 质量；不动 ci.yml。

## 3. 方案

### D1: 移除 `-A missing_docs`（codegen.py:185）

`run_closed_loop` 的 clippy 命令：
```python
# 前
["cargo", "clippy", "-p", crate_name, "--all-targets", "--all-features",
 "--", "-Dwarnings", "-A", "missing_docs"],
# 后
["cargo", "clippy", "-p", crate_name, "--all-targets", "--all-features",
 "--", "-Dwarnings"],
```

### D2: `_field_lines` fallback doc（codegen_render.py）

```python
# 前
if field.description:
    out.append(f"    /// {_oneliner(field.description)}")

# 后（始终生成 doc）
doc = _oneliner(field.description) if field.description else field.rust_name
out.append(f"    /// {doc}")
```

fallback 用字段真实名（`/// user_id`），与 #1「引用真实名、非占位符」原则一致。

### D3: 验证策略（不覆盖真实 crate 文件）

codegen 实跑会覆盖手动改动（marker 警示）。采用**不生成、只验证**策略：
- **D1 安全验证**：移除 -A 后，对 codegen 目标 crate `openlark-communication` 跑 `run_closed_loop` 的 clippy 命令（无 -A）→ exit 0。证明已提交的 codegen 产出 doc-complete、不依赖 -A 兜底。**不跑 codegen 生成**（避免覆盖 communication 手工改动）。
- **D2 单测**：`test_codegen_render.py` 新增——构造空 description 的 FieldDef，断言 `_field_lines` 输出 `/// {field_name}`。
- **回归**：现有 `test_codegen_render`/`test_codegen_ir`/`test_mod_tree` inline 断言不破坏（fixture 字段有 description，D2 不改其输出）；`api-contracts` CI 验证通过。

## 4. 决策与替代

| 决策 | 选择 | 否决的替代 |
|------|------|-----------|
| D1 验证方式 | 对 communication 跑无 -A clippy（不生成） | 实跑 codegen 生成（覆盖手工改动）/ 实跑后 git checkout（多余风险） |
| D2 fallback 文案 | 字段名 `/// {field.rust_name}` | 要求 schema 填 description（超范围、阻塞面大） |
| D2 测试 | 新增空 description 单测 | 仅靠现有 fixture（不覆盖 fallback 路径） |

## 5. 测试策略

| 层级 | 验证 |
|------|------|
| D2 单测 | `test_codegen_render.py`：空 description FieldDef → `_field_lines` 输出含 `/// {field_name}` |
| D1 安全 | `cargo clippy -p openlark-communication --all-targets --all-features -- -Dwarnings`（无 -A）exit 0 |
| 回归 | `pytest tools/api_contracts/test_codegen_render.py test_codegen_ir.py test_mod_tree.py` 全过；`cargo fmt --check`；workspace lint 不破 |
| 占位符守门 | fallback 是字段真实名（非 `待补充文档`） |

## 6. 风险与缓解

- **[移除 -A 暴露其它无 doc pub 项]** → 勘探已确认 pub 项全集（仅字段 + 已 doc 的方法）；D1 安全验证在 communication 上实证。范围可控。
- **[fallback doc 质量]** → 字段名兜底是最小有效 doc；语义 doc 依赖 schema 质量，另案。可接受。
- **[codegen 实跑覆盖手工改动]** → D3 明确不跑生成，只跑 clippy 验证。

## 7. 迁移与回滚

纯增量、非破坏性。回滚 = revert。顺序：先 D2 fallback（使生成 doc-complete）→ D1 移除 -A → D3 验证。

## 8. Open Questions / Build 阶段决策

- build_mode：改动极小（2 文件 ~5 行），倾向 executing-plans/direct → build 阶段 plan-ready 暂停由用户选定。
- isolation：branch（默认）。
- tdd_mode：direct（工具脚本改动，单测验证而非 TDD Red-Green）。
