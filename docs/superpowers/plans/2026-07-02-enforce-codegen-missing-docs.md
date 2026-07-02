---
change: enforce-codegen-missing-docs
design-doc: docs/superpowers/specs/2026-07-02-enforce-codegen-missing-docs-design.md
base-ref: 066a475a360ebde703f9fd196ed323ce1d519926
archived-with: 2026-07-02-enforce-codegen-missing-docs
---

# enforce-codegen-missing-docs 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: 用 superpowers:subagent-driven-development（推荐）或 superpowers:executing-plans 按任务逐个实施。步骤用 checkbox（`- [ ]`）追踪。

**Goal:** 收口 codegen 工具链的 missing_docs 闭环——移除生成后 clippy 验证的 `-A missing_docs` 兜底，并给无 description 的字段生成字段名兜底 doc。

**Architecture:** 两处最小改动 + 单测 + 安全验证。D2（fallback doc）先行确保生成的代码 doc-complete，D1（移除 -A）随后让闭环真正 enforce missing_docs，D3 用「不覆盖真实 crate、只跑 clippy」的策略验证。改动局限在 `tools/`，不碰业务 crate 源码。

**Tech Stack:** Python（codegen 工具链）、Rust/clippy（闭环验证）、unittest（Python 单测）。

## Global Constraints

- 改动局限 `tools/codegen.py` 与 `tools/api_contracts/codegen_render.py` 两个文件 + `tools/api_contracts/test_codegen_render.py` 一个测试文件。
- **不覆盖真实业务 crate 源码**（D3 验证只跑 clippy，不跑 codegen 生成）。
- **不动** ci.yml、schema 解析（codegen_ir）、风格 B、proposal/spec 范围。
- fallback doc 必须是**字段真实名**（如 `/// user_id`），禁止占位符（`待补充文档` 等）。
- **不给 trait impl（`_render_api_response_trait`）和 `_tests`（`#[cfg(test)]` 私有 mod）加 doc** —— 这两项不需要 doc（trait impl 继承 doc，cfg(test) mod 非 pub）。
- 中文输出，提交信息用中文/约定式。

archived-with: 2026-07-02-enforce-codegen-missing-docs
---

## File Structure

| 文件 | 职责 | 改动 |
|------|------|------|
| `tools/api_contracts/codegen_render.py` | 风格 A 渲染器 | `_field_lines`（行 134-143）：description 缺失时生成字段名兜底 doc |
| `tools/api_contracts/test_codegen_render.py` | 渲染器单测（inline `assertIn`，非 golden file） | 新增一个测试方法，断言空 description 字段产出 `/// {rust_name}` |
| `tools/codegen.py` | codegen 编排器（含 `run_closed_loop` 生成后验证） | 行 183-186：`run_closed_loop` clippy 命令移除 `-A missing_docs` |

无新文件创建。

archived-with: 2026-07-02-enforce-codegen-missing-docs
---

## Task 1: 勘探确认（pub 项全集 + 测试现状）

**Files:**
- Read: `tools/api_contracts/codegen_render.py`（确认 pub 项渲染入口）
- Read: `tools/api_contracts/test_codegen_render.py`（确认测试用 inline 断言，非 golden）

**Interfaces:**
- Consumes: Design Doc 第 1 节勘探事实
- Produces: 确认结论，指导 Task 2/4 是否需要同步既有测试预期

**勘探事实（已在 Design Doc 确认，本任务复核）：**
- codegen 生成的 pub 项入口：`render_api_file`（codegen_render.py:31）→ `_render_struct`（struct 级 doc + 字段级 `_field_lines`）、`_request_struct`/`_request_impl`（已 doc）。
- **唯一 doc 缺口 = `_field_lines` 中 description 缺失的字段**（行 137 `if field.description:` 才输出 doc）。无 `pub enum`/`pub type`/`pub const` 渲染（endpoint const 在 endpoints/ 属 [MANUAL]）。
- `test_codegen_render.py` 用 **inline `assertIn`/`assertNotIn`**，无 golden file 全文比对；fixture `SCHEMA_POST`（行 36-55）的 body 字段 `msg_type`/`uuid` 本就无 description → 当前不产 doc，D2 后会产 `/// msg_type`/`/// uuid`。
- 既有 `test_body_struct`（行 132-138）只断言字段行存在（`assertIn("pub msg_type: String,", ...)`），不断言缺失 doc 行 → **D2 不会破坏既有断言**，无需同步。

- [x] **Step 1: 复核 codegen_render.py 的 pub 项渲染入口**

Run: `grep -n "pub \|_field_lines\|_render_struct\|fn _" tools/api_contracts/codegen_render.py | head -40`
Expected: 确认 `_field_lines` 是唯一字段级 doc 出口；struct/方法已有 doc；无 enum/const 渲染。

- [x] **Step 2: 复核测试现状（确认无需同步 golden）**

Run: `grep -n "assertIn.*/// \|golden\|msg_type\|uuid" tools/api_contracts/test_codegen_render.py`
Expected: 看到 `SCHEMA_POST` 中 `msg_type`/`uuid` 无 description，但既有断言只查 `pub msg_type: String,` 这类字段行，不查 doc 行的缺失/存在 → Task 4.1 答案：**无需同步既有预期**。

- [x] **Step 3: 记录结论**

本任务无代码改动。结论：pub 项 doc 缺口仅在无 description 字段；既有测试用 inline 断言、D2 不破坏。进入 Task 2。

archived-with: 2026-07-02-enforce-codegen-missing-docs
---

## Task 2: D2 — `_field_lines` fallback doc

**Files:**
- Modify: `tools/api_contracts/codegen_render.py:134-143`（`_field_lines` 函数）

**Interfaces:**
- Consumes: `FieldDef`（`codegen_ir.py:80-87`，含 `rust_name: str` 与 `description: str = ""`）
- Produces: `_field_lines` 对空 description 字段输出 `    /// {field.rust_name}`（始终生成 doc 行）

- [x] **Step 1: 修改 `_field_lines`，description 缺失时用字段名兜底**

替换 `tools/api_contracts/codegen_render.py` 的 `_field_lines` 函数体（当前行 134-143）：

```python
def _field_lines(field: FieldDef, *, force_optional: bool = False) -> list[str]:
    optional = (not field.required) or force_optional
    out: list[str] = []
    doc = _oneliner(field.description) if field.description else field.rust_name
    out.append(f"    /// {doc}")
    if optional:
        out.append('    #[serde(skip_serializing_if = "Option::is_none")]')
    rust_t = _rust_type(field.type_expr, field.required and not force_optional)
    out.append(f"    pub {field.rust_name}: {rust_t},")
    return out
```

关键变化：原 `if field.description:` 条件分支 → 始终生成 doc 行，文案在 description 缺失时退化为 `field.rust_name`。`_oneliner("")` 会返回空串（见行 364-370），故必须保留 `if field.description else` 守卫，避免输出空 doc 行 `/// `。

- [x] **Step 2: 静态自检——确认改动语义**

Run: `python3 -c "import ast; ast.parse(open('tools/api_contracts/codegen_render.py').read()); print('OK')"`
Expected: 输出 `OK`（语法合法）。

- [x] **Step 3: 提交 D2**

```bash
git add tools/api_contracts/codegen_render.py
git commit -m "feat(codegen): 字段无 description 时用字段名兜底生成 doc（D2）"
```

archived-with: 2026-07-02-enforce-codegen-missing-docs
---

## Task 3: D2 单测——空 description 字段产出字段名 doc

**Files:**
- Modify: `tools/api_contracts/test_codegen_render.py`（新增测试方法/类）

**Interfaces:**
- Consumes: `_field_lines`（codegen_render.py）、`FieldDef`/`TypePrimitive`（codegen_ir.py）
- Produces: 一个新测试，断言空 description → `/// {rust_name}`；有 description → 原 `_oneliner` 文案

- [x] **Step 1: 写失败测试——新增测试类覆盖 fallback 路径**

在 `tools/api_contracts/test_codegen_render.py` 末尾追加测试类。需新增 import `_field_lines`、`FieldDef`、`TypePrimitive`：

调整文件顶部 import 块（行 9-15 区域），把 `_field_lines` 加入 codegen_render 的 import，并在 codegen_ir 的 import 行追加 `FieldDef`、`TypePrimitive`：

```python
from tools.api_contracts.codegen_ir import FieldDef, TypePrimitive, parse_api_schema_to_ir
from tools.api_contracts.codegen_render import (
    _emit_token_decl,
    _field_lines,
    render_api_file,
    render_endpoint_const_snippet,
)
```

文件末尾追加测试类：

```python
class FieldLinesFallbackDocTest(unittest.TestCase):
    """D2：无 description 字段用字段名兜底生成 doc。"""

    def test_empty_description_falls_back_to_rust_name(self):
        field = FieldDef(
            name="user_id",
            rust_name="user_id",
            type_expr=TypePrimitive(rust="String"),
            required=True,
            description="",
        )
        lines = _field_lines(field)
        self.assertIn("    /// user_id", lines)
        # 兜底文案必须是字段真实名，非空 doc 行、非占位符
        for line in lines:
            self.assertNotIn("///  ", line)  # 不是空 doc 行（"/// " 后空白）
            self.assertNotIn("待补充", line)
            self.assertNotIn("TODO", line)

    def test_present_description_uses_oneliner(self):
        field = FieldDef(
            name="user_id",
            rust_name="user_id",
            type_expr=TypePrimitive(rust="String"),
            required=True,
            description="用户标识",
        )
        lines = _field_lines(field)
        self.assertIn("    /// 用户标识", lines)

    def test_every_field_gets_doc_with_scHEMA_post(self):
        # 集成层：SCHEMA_POST 的 msg_type/uuid 无 description → 兜底出 doc
        ir = parse_api_schema_to_ir(_api(), SCHEMA_POST)
        code = render_api_file(ir)
        self.assertIn("    /// msg_type", code)
        self.assertIn("    /// uuid", code)
        # 有 description 的字段文案不变
        self.assertIn("    /// 接收者", code)
```

- [x] **Step 2: 运行新测试，确认通过（fallback 已在 Task 2 实现，应直接绿）**

Run: `python3 -m pytest tools/api_contracts/test_codegen_render.py::FieldLinesFallbackDocTest -v`
Expected: 3 个测试全 PASS。若 `test_empty_description_falls_back_to_rust_name` FAIL，说明 Task 2 改动未生效或 `_field_lines` 未正确导入。

- [x] **Step 3: 提交测试**

```bash
git add tools/api_contracts/test_codegen_render.py
git commit -m "test(codegen): 新增空 description 字段兜底 doc 单测（D2）"
```

archived-with: 2026-07-02-enforce-codegen-missing-docs
---

## Task 4: D2 回归——全套渲染器/IR/mod_tree 单测通过

**Files:**
- Run: `tools/api_contracts/test_codegen_render.py`、`test_codegen_ir.py`、`test_mod_tree.py`（只跑，不改）

**Interfaces:**
- Consumes: Task 2/3 的产物
- Produces: 全绿回归证明（D2 不破坏既有渲染）

- [x] **Step 1: 跑三套单测**

Run: `python3 -m pytest tools/api_contracts/test_codegen_render.py tools/api_contracts/test_codegen_ir.py tools/api_contracts/test_mod_tree.py -v`
Expected: 全部 PASS，0 fail。

> 若无 pytest：`python3 -m unittest tools.api_contracts.test_codegen_render tools.api_contracts.test_codegen_ir tools.api_contracts.test_mod_tree -v`（在仓库根，注意包路径）。

- [x] **Step 2: 占位符守门（人工抽样）**

Run: `grep -rn "待补充\|TODO.*doc\|placeholder" tools/api_contracts/codegen_render.py`
Expected: 无命中（fallback 用的是 `field.rust_name`，非占位符）。若命中既有无关代码，记录但不处理。

- [x] **Step 3: 无 commit（本任务只验证，无代码改动）**

回归通过即放行进入 Task 5。

archived-with: 2026-07-02-enforce-codegen-missing-docs
---

## Task 5: D1 — 移除 `run_closed_loop` 的 `-A missing_docs`

**Files:**
- Modify: `tools/codegen.py:183-186`（`run_closed_loop` 的 clippy 命令列表）

**Interfaces:**
- Consumes: Task 2 的 D2（生成已 doc-complete，移除 -A 后闭环可过）
- Produces: codegen 闭环真正 enforce missing_docs

- [x] **Step 1: 移除 clippy 命令的 `-A missing_docs`**

替换 `tools/codegen.py` 的 `run_closed_loop` 中 commands 列表（当前行 181-187）：

```python
    commands = [
        ["cargo", "fmt", "--all"],
        [
            "cargo", "clippy", "-p", crate_name, "--all-targets", "--all-features",
            "--", "-Dwarnings",
        ],
    ]
```

关键变化：clippy 命令行尾从 `"--", "-Dwarnings", "-A", "missing_docs"` → `"--", "-Dwarnings"`。`cargo fmt --all` 行不变。

- [x] **Step 2: 静态自检**

Run: `python3 -c "import ast; ast.parse(open('tools/codegen.py').read()); print('OK')"`
Expected: 输出 `OK`。

- [x] **Step 3: 提交 D1**

```bash
git add tools/codegen.py
git commit -m "feat(codegen): 移除生成后 clippy 的 -A missing_docs 兜底（D1）"
```

archived-with: 2026-07-02-enforce-codegen-missing-docs
---

## Task 6: D3 验证——闭环安全 + workspace 回归

**Files:**
- Run: `cargo clippy -p openlark-communication`（不生成，只验证已提交产出 doc-complete）
- Run: `cargo fmt --check`、`just lint`（workspace 回归）

**Interfaces:**
- Consumes: Task 2-5 全部产物
- Produces: 证明移除 -A 后真实 codegen 目标 crate 仍 clippy 通过

> **不跑 codegen 生成**（避免覆盖 `openlark-communication` 等真实 crate 的手工改动）。只对已提交的 crate 跑无 -A 的 clippy。

- [x] **Step 1: D1 安全验证——openlark-communication 无 -A clippy 通过**

Run: `cargo clippy -p openlark-communication --all-targets --all-features -- -Dwarnings`
Expected: exit 0，无 missing_docs 警告。

> 证明已提交的 codegen 产出是 doc-complete 的、不依赖 -A 兜底。若 FAIL 且报 missing_docs → 说明真实 crate 有遗留缺 doc pub 项（应是 #273 子项 #1 的 analytics 122 范围之外的新发现），停止并按 Design Doc §6 风险条上报用户（不在本 change 内修业务 crate）。

- [x] **Step 2: workspace fmt + lint 回归**

Run: `cargo fmt --check`
Expected: exit 0（codegen 改动在 `tools/`，不影响 Rust 格式；但 CI lint 第一步是 fmt --check，必须显式确认）。

Run: `just lint`
Expected: 通过（clippy workspace 无新 warning）。若 `just` 不可用，等价 `cargo clippy --workspace --all-targets --all-features -- -Dwarnings`。

- [x] **Step 3: （可选）api-contracts 验证脚本**

Run: `python3 tools/validate_api_contracts.py 2>&1 | tail -20` 或仓库既有等价命令
Expected: 不破（若脚本依赖网络/缓存不可跑，记录跳过原因，非阻塞）。

- [x] **Step 4: 最终确认 + 不再 commit（验证任务无代码改动）**

确认所有验证通过。若过程中发现需修补（如 fmt 失败需 `cargo fmt`），回到对应 Task 修后单独 commit。

archived-with: 2026-07-02-enforce-codegen-missing-docs
---

## Self-Review

**1. Spec coverage（对 Design Doc 各节）：**
- D1（§3 D1，codegen.py:185 移除 -A）→ Task 5 ✓
- D2（§3 D2，`_field_lines` fallback）→ Task 2 ✓
- D2 单测（§5 D2 单测）→ Task 3 ✓
- D3（§3 D3，不覆盖真实 crate）→ Task 6 Step 1 明确「不跑生成、只跑 clippy」✓
- 回归（§5 回归：三套单测 + fmt + workspace lint）→ Task 4 + Task 6 ✓
- 占位符守门（§5）→ Task 3 Step 1 断言 + Task 4 Step 2 grep ✓
- tasks.md 1.1/1.2（勘探）→ Task 1 ✓
- tasks.md 2.1/2.2（D2 改动 + 自验）→ Task 2 + Task 3 ✓
- tasks.md 3.1（D1）→ Task 5 ✓
- tasks.md 4.1（golden 同步）→ Task 1 Step 2 结论：**无 golden，无需同步**（既有用 inline assertIn）✓
- tasks.md 4.2（单测通过）→ Task 4 ✓
- tasks.md 5.1（实跑闭环）→ Task 6 Step 1（按 Design Doc D3 改为「不生成只验证」）✓
- tasks.md 5.2（fmt + lint）→ Task 6 Step 2 ✓
- tasks.md 5.3（api-contracts CI）→ Task 6 Step 3 ✓
- tasks.md 5.4（占位符守门）→ Task 3/4 ✓

**2. Placeholder scan：** 无 TBD/TODO/「类似 Task N」；每个 code 步骤含完整代码块；每个验证步骤含具体命令与期望输出。✓

**3. Type consistency：**
- `FieldDef(name, rust_name, type_expr, required, description)` —— 与 `codegen_ir.py:80-87` 一致（`description: str = ""` 默认空）✓
- `TypePrimitive(rust="String")` —— 与 `codegen_ir.py` 中 `TypePrimitive` 字段名一致 ✓
- `_field_lines(field, *, force_optional=False)` —— 签名未变 ✓
- `_oneliner(field.description)` —— `_oneliner(text: str, limit=80) -> str`，空 description 不可直接传（返回 ""），故保留 `if field.description else` 守卫 ✓
- import 块新增 `_field_lines`、`FieldDef`、`TypePrimitive` 名称与源模块导出一致 ✓

无遗留 gap。
