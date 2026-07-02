---
change: cleanup-missing-docs-tests
design-doc: docs/superpowers/specs/2026-07-02-cleanup-missing-docs-tests-design.md
base-ref: d29c87fafe2131d6805aaed0ff353bec8cf0eb7f
---

# cleanup-missing-docs-tests Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 清理 `tools/tests/` 下 18 个不在 CI 的 per-crate missing_docs 死测试——删 8 整文件 + 改 9 文件（删冗余 `has_no_warnings` 方法及 `subprocess` import）+ 接 10 结构变体进 CI，激活 crate 特定 `#![allow]` 回归守卫。

**Architecture:** 纯机械改动，无业务代码、无测试断言逻辑变更。三类操作：(D1a) `git rm` 删除仅含 `has_no_warnings` 的 8 个文件；(D1b) 9 个"混合"文件精简到只剩结构变体（参照 `test_openlark_workflow_narrow_missing_docs.py` 形态——只 `import unittest` + `from pathlib import Path`）；(D2) ci.yml 现有 run block 内插入一行 `python3 -m unittest` + 10 个 backslash 续行模块。冗余论证见 Design Doc §3 D1：per-crate `cargo test -p <crate> --all-features --no-run` 是 workspace `--workspace --all-features` 的子集（feature unification 让 workspace 更宽），workspace 测试（#1 PR #293 已接 CI）subsume 全部 per-crate，删除安全。

**Tech Stack:** Python 3.12（unittest）、GitHub Actions YAML、Rust（`cargo doc` 守门，本 change 不改 Rust 代码）。

## Global Constraints

- **不改业务 crate Rust 源码**（本 change 只动 `tools/tests/*.py` + `.github/workflows/ci.yml`）。
- **不改测试断言逻辑**：结构变体方法（`do_not_suppress`/`mod_roots`/`cleaned_slices`/`v1_root` 等）的硬编码路径列表、断言文本、msg 一律保留原样。
- **不动 `test_openlark_workflow_narrow_missing_docs.py`**（参照形态，本就只有结构变体）。
- **不动 #1 的 `test_workspace_missing_docs`**（ci.yml:114，PR #293 已接 CI 的 workspace 级守门，本 change 的删除安全性依赖它）。
- **不动 `test_check_mod_reachability` 和 `check_mod_reachability.py`**。
- **import 清理规则**：9 文件删 `has_no_warnings` 方法后，`import subprocess` 变 unused，必须同步删；保留 `import unittest` + `from pathlib import Path`（结构变体所需）。
- **ci.yml 改动范围**：仅 ci.yml:112-115 的 "Check mod reachability" step 的 run block；缩进、step 名、其他 step 一律不动。
- **base-ref**：`d29c87fafe2131d6805aaed0ff353bec8cf0eb7f`（main HEAD，工作树 clean）。
- **顺序约束**：D1（Task 1-2）必须先于 D2（Task 3）——ci.yml 接的 10 模块在 D1 完成后才只剩结构变体（跑模块 = 跑结构变体）；D1 删/改期间若某文件还残留 `has_no_warnings`，被 ci.yml 调用会触发慢编译（18×）违背初衷。

---

## File Structure

| 文件 | 操作 | 责任 |
|------|------|------|
| `tools/tests/test_openlark_{ai,analytics,application,auth,cardkit,client,core,webhook}_missing_docs.py` (8) | **删除整文件** | 仅含 `has_no_warnings`，冗余于 #1 workspace 测试 |
| `tools/tests/test_openlark_{communication,docs,helpdesk,hr,mail,meeting,platform,protocol,workflow}_missing_docs.py` (9) | **改**：删 `test_*_has_no_missing_docs_warnings` 方法 + 删 `import subprocess` | 混合文件，精简到只剩结构变体 |
| `tools/tests/test_openlark_workflow_narrow_missing_docs.py` (1) | **不动** | 参照形态（仅 `import unittest` + `from pathlib import Path` + 结构变体） |
| `.github/workflows/ci.yml:112-115` | **改**：run block 插入 `python3 -m unittest` + 10 续行模块 | 接 10 结构变体进 CI |

**改后终态**：`ls tools/tests/test_openlark_*_missing_docs.py | wc -l` = 10（9 改 + workflow_narrow）；`grep has_no_missing_docs_warnings tools/tests/` 空；ci.yml "Check mod reachability" step 跑 12 个 python 命令（test_check_mod_reachability + test_workspace_missing_docs + 10 模块 + check_mod_reachability.py）。

---

## Task 1: 删 8 个仅含 has_no_warnings 的整文件 (D1a)

**Files:**
- Delete: `tools/tests/test_openlark_ai_missing_docs.py`
- Delete: `tools/tests/test_openlark_analytics_missing_docs.py`
- Delete: `tools/tests/test_openlark_application_missing_docs.py`
- Delete: `tools/tests/test_openlark_auth_missing_docs.py`
- Delete: `tools/tests/test_openlark_cardkit_missing_docs.py`
- Delete: `tools/tests/test_openlark_client_missing_docs.py`
- Delete: `tools/tests/test_openlark_core_missing_docs.py`
- Delete: `tools/tests/test_openlark_webhook_missing_docs.py`

**Interfaces:**
- Consumes: 无（独立删除操作）。
- Produces: 仓库少 8 个文件；git 暂存区记录 8 个 deletion。Task 3 的 ci.yml 接线**不**包含这 8 个模块名（它们已不存在）。

- [x] **Step 1: 确认 8 文件均仅含 has_no_warnings（删除前置校验）**

逐个检查 8 文件无结构变体方法（`do_not_suppress`/`mod_roots`/`cleaned_slices`/`v1_root`），确认整删安全：

Run: `for m in ai analytics application auth cardkit client core webhook; do echo "=== $m ==="; grep -E 'def test_|do_not_suppress|mod_roots|cleaned_slices|v1_root' tools/tests/test_openlark_${m}_missing_docs.py; done`
Expected: 每个文件只有 1 行 `def test_*_has_no_missing_docs_warnings`，**无**任何 `do_not_suppress`/`mod_roots`/`cleaned_slices`/`v1_root` 命中。若某文件出现结构变体方法，**停止**——它属于 Task 2 的 9 文件之列，整删会丢回归守卫，需回查 Design Doc §3 D1 表格。

- [x] **Step 2: git rm 8 文件**

Run:
```bash
git rm tools/tests/test_openlark_ai_missing_docs.py \
       tools/tests/test_openlark_analytics_missing_docs.py \
       tools/tests/test_openlark_application_missing_docs.py \
       tools/tests/test_openlark_auth_missing_docs.py \
       tools/tests/test_openlark_cardkit_missing_docs.py \
       tools/tests/test_openlark_client_missing_docs.py \
       tools/tests/test_openlark_core_missing_docs.py \
       tools/tests/test_openlark_webhook_missing_docs.py
```
Expected: 8 行 `rm tools/tests/test_openlark_*_missing_docs.py`，git 暂存 8 个 deletion。

- [x] **Step 3: 验证文件数降到 10**

Run: `ls tools/tests/test_openlark_*_missing_docs.py | wc -l`
Expected: `10`（18 − 8）。若不是 10，停止排查。

Run: `ls tools/tests/test_openlark_*_missing_docs.py | sort`
Expected: 输出恰好这 10 行（9 待改 + workflow_narrow）：
```
tools/tests/test_openlark_communication_missing_docs.py
tools/tests/test_openlark_docs_missing_docs.py
tools/tests/test_openlark_helpdesk_missing_docs.py
tools/tests/test_openlark_hr_missing_docs.py
tools/tests/test_openlark_mail_missing_docs.py
tools/tests/test_openlark_meeting_missing_docs.py
tools/tests/test_openlark_platform_missing_docs.py
tools/tests/test_openlark_protocol_missing_docs.py
tools/tests/test_openlark_workflow_missing_docs.py
tools/tests/test_openlark_workflow_narrow_missing_docs.py
```

- [x] **Step 4: 提交**

Run:
```bash
git commit -m "test: 删 8 个仅含 has_no_warnings 的冗余 missing_docs 测试 (#273 Part #2 D1a)

per-crate cargo test -p <crate> --no-run 是 workspace --workspace --all-features
的子集（feature unification 让 workspace 更宽），#1 的 workspace 级测试
(PR #293 已接 CI) subsume 全部 per-crate，删除安全。

删: test_openlark_{ai,analytics,application,auth,cardkit,client,core,webhook}_missing_docs.py"
```
Expected: commit 成功，`git status` clean。

---

## Task 2: 改 9 个混合文件——删 has_no_warnings 方法 + 删 import subprocess (D1b)

**Files:**
- Modify: `tools/tests/test_openlark_communication_missing_docs.py`
- Modify: `tools/tests/test_openlark_docs_missing_docs.py`
- Modify: `tools/tests/test_openlark_helpdesk_missing_docs.py`
- Modify: `tools/tests/test_openlark_hr_missing_docs.py`
- Modify: `tools/tests/test_openlark_mail_missing_docs.py`
- Modify: `tools/tests/test_openlark_meeting_missing_docs.py`
- Modify: `tools/tests/test_openlark_platform_missing_docs.py`
- Modify: `tools/tests/test_openlark_protocol_missing_docs.py`
- Modify: `tools/tests/test_openlark_workflow_missing_docs.py`

**Interfaces:**
- Consumes: Task 1 已删 8 文件（本 task 的 9 文件不受影响，但仓库已无 8 整删文件）。
- Produces: 9 文件精简到"参照 workflow_narrow 形态"——只剩 `import unittest` + `from pathlib import Path` + 结构变体方法 + `if __name__ == "__main__": unittest.main()`。Task 3 ci.yml 接线后，跑这 9 模块 = 跑结构变体（0.27s 文件扫描，无 cargo 编译）。

**改造模板（9 文件统一形态）**——以 `test_openlark_workflow_missing_docs.py` 为范例（其余 8 文件结构变体方法体各异，但 import 区和方法删除规则相同）：

改造前（workflow 现状，见 base-ref）：
```python
import subprocess                                              # ← 删
import unittest
from pathlib import Path


class OpenlarkWorkflowMissingDocsTests(unittest.TestCase):
    def test_openlark_workflow_has_no_missing_docs_warnings(self):   # ← 删整个方法（含方法体）
        result = subprocess.run(
            ["cargo", "test", "-p", "openlark-workflow", "--all-features", "--no-run"],
            capture_output=True,
            text=True,
            check=False,
        )

        output = result.stdout + result.stderr
        self.assertEqual(result.returncode, 0, msg=output)
        self.assertNotIn("warning: missing document for ", output, msg=output)

    def test_openlark_workflow_cleaned_slices_do_not_suppress_missing_docs(self):
        # ... 结构变体方法，保留不动 ...
        ...


if __name__ == "__main__":
    unittest.main()
```

改造后（参照 `test_openlark_workflow_narrow_missing_docs.py` 形态）：
```python
import unittest
from pathlib import Path


class OpenlarkWorkflowMissingDocsTests(unittest.TestCase):
    def test_openlark_workflow_cleaned_slices_do_not_suppress_missing_docs(self):
        # ... 结构变体方法，保留不动 ...
        ...


if __name__ == "__main__":
    unittest.main()
```

**两处必删（每文件）**：
1. `import subprocess` 行（9 文件均有，workflow_narrow 无——已验证 baseline `grep 'import subprocess'` 命中 17，刨掉非 missing_docs 文件后 9 文件各 1）。
2. `def test_<crate>_has_no_missing_docs_warnings(self):` 方法整体（从 `def` 行到下一个 `def` 前的空行，含方法体所有行）。方法名按 crate 命名（如 `test_openlark_communication_has_no_missing_docs_warnings`）。

- [x] **Step 1: 确认 9 文件均有 import subprocess + has_no_warnings 方法（改前校验）**

Run: `for m in communication docs helpdesk hr mail meeting platform protocol workflow; do echo "=== $m ==="; grep -cE '^import subprocess|has_no_missing_docs_warnings' tools/tests/test_openlark_${m}_missing_docs.py; done`
Expected: 每文件至少输出 `2`（1 行 `import subprocess` + ≥1 行含 `has_no_missing_docs_warnings` 的方法 def/调用）。若某文件输出 `<2`，停止——可能已被改过或归类错误，回查 Design Doc §3 D1 表。

- [x] **Step 2: 逐文件删除 import subprocess 行**

对 9 文件每个，删除文件首行 `import subprocess`（保留其后的 `import unittest` 和 `from pathlib import Path`，以及两 import 间的空行规则——参照 workflow_narrow：`import unittest` 在前，空行，`from pathlib import Path`，两空行，`class`）。

每文件改后首部应形如：
```python
import unittest
from pathlib import Path


class Openlark<Crate>MissingDocsTests(unittest.TestCase):
```

> 注：部分文件 import 顺序可能为 `import subprocess` / `import unittest` / `from pathlib import Path`，删 subprocess 后剩两行；若原顺序为 `import unittest` / `import subprocess` / `from pathlib import Path`，删中间 subprocess 行后注意不要留双空行。以"参照 workflow_narrow 首部"为准。

- [x] **Step 3: 逐文件删除 has_no_missing_docs_warnings 方法**

对 9 文件每个，删除 `def test_<crate>_has_no_missing_docs_warnings(self):` 整个方法块——从该 `def` 行起，到下一个 `def test_` 行（结构变体方法）前的所有行（含方法间的空行分隔）。删除后第一个结构变体方法应紧跟 class 声明后的空行。

范例（workflow 文件）删除区间：当前文件第 7-18 行（`def test_openlark_workflow_has_no_missing_docs_warnings` 到 `self.assertNotIn(...)` 方法体结束的空行），删后第 19 行的 `def test_openlark_workflow_cleaned_slices_do_not_suppress_missing_docs` 上移。

**保留不动**：所有结构变体方法（`do_not_suppress`/`mod_roots`/`cleaned_slices`/`v1_root` 等命名变体）的 def 签名、硬编码路径列表、断言文本、msg 字符串、循环体。

- [x] **Step 4: 校验无残留 has_no_warnings + 无残留 import subprocess**

Run: `grep -rn 'has_no_missing_docs_warnings' tools/tests/`
Expected: 空（无输出）。若有输出，漏删某文件的方法，回 Step 3 补。

Run: `grep -rn '^import subprocess' tools/tests/test_openlark_*_missing_docs.py`
Expected: 空。若有输出，漏删某文件的 import，回 Step 2 补。

- [x] **Step 5: 校验 9 文件 Python 语法合法 + 可被 unittest 加载**

Run: `python3 -m py_compile tools/tests/test_openlark_communication_missing_docs.py tools/tests/test_openlark_docs_missing_docs.py tools/tests/test_openlark_helpdesk_missing_docs.py tools/tests/test_openlark_hr_missing_docs.py tools/tests/test_openlark_mail_missing_docs.py tools/tests/test_openlark_meeting_missing_docs.py tools/tests/test_openlark_platform_missing_docs.py tools/tests/test_openlark_protocol_missing_docs.py tools/tests/test_openlark_workflow_missing_docs.py`
Expected: 无输出、exit 0（py_compile 成功）。若有 SyntaxError，删除时多删/少删了行，回 Step 2/3 修正。

- [x] **Step 6: 实跑 9 文件结构变体全绿（确认删方法未误伤结构变体）**

Run（在仓库根，9 模块一起跑）:
```bash
python3 -m unittest \
  tools.tests.test_openlark_communication_missing_docs \
  tools.tests.test_openlark_docs_missing_docs \
  tools.tests.test_openlark_helpdesk_missing_docs \
  tools.tests.test_openlark_hr_missing_docs \
  tools.tests.test_openlark_mail_missing_docs \
  tools.tests.test_openlark_meeting_missing_docs \
  tools.tests.test_openlark_platform_missing_docs \
  tools.tests.test_openlark_protocol_missing_docs \
  tools.tests.test_openlark_workflow_missing_docs
```
Expected: `OK` + exit 0，9 个 test case 全 pass（每文件 1 个结构变体方法 = 1 test，共 9）。耗时约 0.3s（纯文件扫描，无 cargo 编译——证明 has_no_warnings 已删干净）。若某 test FAIL，结构变体方法体被误改，回 Step 3 核对方法体与 base-ref diff。

- [x] **Step 7: 提交**

Run:
```bash
git add tools/tests/test_openlark_communication_missing_docs.py \
        tools/tests/test_openlark_docs_missing_docs.py \
        tools/tests/test_openlark_helpdesk_missing_docs.py \
        tools/tests/test_openlark_hr_missing_docs.py \
        tools/tests/test_openlark_mail_missing_docs.py \
        tools/tests/test_openlark_meeting_missing_docs.py \
        tools/tests/test_openlark_platform_missing_docs.py \
        tools/tests/test_openlark_protocol_missing_docs.py \
        tools/tests/test_openlark_workflow_missing_docs.py
git commit -m "test: 删 9 文件的冗余 has_no_warnings 方法 + import subprocess (#273 Part #2 D1b)

每文件删 test_*_has_no_missing_docs_warnings 方法（冗余于 #1 workspace 测试）
+ 同步删 unused 的 import subprocess。保留结构变体方法（do_not_suppress/
mod_roots/cleaned_slices/v1_root）及其 import unittest/pathlib.Path 不动。

参照 test_openlark_workflow_narrow_missing_docs.py 形态。"
```
Expected: commit 成功，`git status` clean。

---

## Task 3: ci.yml 接 10 结构变体进 CI (D2)

**Files:**
- Modify: `.github/workflows/ci.yml:112-115`（"Check mod reachability" step 的 run block）

**Interfaces:**
- Consumes: Task 1 + Task 2 完成（10 个 `test_openlark_*_missing_docs.py` 模块均只剩结构变体，可被 `python3 -m unittest` 加载且 0.3s 内跑完）。
- Produces: CI "Check mod reachability" step 在 `test_workspace_missing_docs` 之后、`check_mod_reachability.py` 之前，额外跑 10 个结构变体模块。后续 PR 的 CI 会激活 crate 特定 `#![allow]` 回归守卫。

**改动锚点**（base-ref ci.yml:111-115）：
```yaml
      - name: Check mod reachability (no new orphan src files)
        run: |
          python3 -m unittest tools.tests.test_check_mod_reachability
          python3 -m unittest tools.tests.test_workspace_missing_docs
          python3 tools/check_mod_reachability.py
```

改后（在 `test_workspace_missing_docs` 行后、`check_mod_reachability.py` 行前，插入 `python3 -m unittest \` + 10 backslash 续行模块；沿用 ci.yml:69-72 多模块 backslash 续行格式）：
```yaml
      - name: Check mod reachability (no new orphan src files)
        run: |
          python3 -m unittest tools.tests.test_check_mod_reachability
          python3 -m unittest tools.tests.test_workspace_missing_docs
          python3 -m unittest \
            tools.tests.test_openlark_communication_missing_docs \
            tools.tests.test_openlark_docs_missing_docs \
            tools.tests.test_openlark_helpdesk_missing_docs \
            tools.tests.test_openlark_hr_missing_docs \
            tools.tests.test_openlark_mail_missing_docs \
            tools.tests.test_openlark_meeting_missing_docs \
            tools.tests.test_openlark_platform_missing_docs \
            tools.tests.test_openlark_protocol_missing_docs \
            tools.tests.test_openlark_workflow_missing_docs \
            tools.tests.test_openlark_workflow_narrow_missing_docs
          python3 tools/check_mod_reachability.py
```

**约束**：
- 缩进严格照抄相邻行（run block 内命令 10 空格缩进，续行 12 空格）。
- step 名 `Check mod reachability (no new orphan src files)` 不改。
- 不动 `test_check_mod_reachability` 行、`test_workspace_missing_docs` 行、`check_mod_reachability.py` 行。
- 模块顺序按 Design Doc §3 D2（communication → docs → helpdesk → hr → mail → meeting → platform → protocol → workflow → workflow_narrow）。

- [x] **Step 1: 确认 ci.yml 当前 run block 与 base-ref 一致（改前锚点校验）**

Run: `sed -n '111,115p' .github/workflows/ci.yml`
Expected: 输出恰为上面"改动锚点"的 5 行（name + run + 3 命令）。若行号偏移或内容不同，停止——main 上有新提交动了 ci.yml，需 rebase 到最新 main 再改，否则 diff 错位。

- [x] **Step 2: 在 run block 插入 10 模块调用**

用 Edit 工具，old_string = 改动锚点的 3 命令行（`python3 -m unittest tools.tests.test_workspace_missing_docs\n          python3 tools/check_mod_reachability.py`），new_string = 在两者间插入 `python3 -m unittest \` + 10 续行模块（如上"改后"块）。注意保留行首缩进（命令行 10 空格、续行 12 空格）和每个续行末尾的 backslash（最后一行 `workflow_narrow_missing_docs` 无 backslash）。

- [x] **Step 3: 校验 yaml 语法合法**

Run: `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml')); print('yaml OK')"`
Expected: `yaml OK`。若抛 `yaml.YAMLError`，缩进/backslash 错，回 Step 2 核对。

- [x] **Step 4: 校验插入的 run block 与目标一致**

Run: `sed -n '111,126p' .github/workflows/ci.yml`
Expected: 输出恰为"改后"块 16 行（name + run + 14 命令/续行）。逐行核对：`test_check_mod_reachability` → `test_workspace_missing_docs` → `python3 -m unittest \` → 10 模块续行 → `check_mod_reachability.py`。

- [x] **Step 5: 本地实跑 ci.yml 那条新命令（模拟 CI）**

Run（在仓库根，照抄 ci.yml 新增的那条命令）:
```bash
python3 -m unittest \
  tools.tests.test_openlark_communication_missing_docs \
  tools.tests.test_openlark_docs_missing_docs \
  tools.tests.test_openlark_helpdesk_missing_docs \
  tools.tests.test_openlark_hr_missing_docs \
  tools.tests.test_openlark_mail_missing_docs \
  tools.tests.test_openlark_meeting_missing_docs \
  tools.tests.test_openlark_platform_missing_docs \
  tools.tests.test_openlark_protocol_missing_docs \
  tools.tests.test_openlark_workflow_missing_docs \
  tools.tests.test_openlark_workflow_narrow_missing_docs
```
Expected: `OK` + exit 0，10 个 test case 全 pass（Task 2 的 9 + workflow_narrow 的 1）。耗时约 0.3s。若 FAIL，对照 Task 2 Step 6 排查（workflow_narrow 本就 pass，问题应在 9 文件之一）。

- [x] **Step 6: 提交**

Run:
```bash
git add .github/workflows/ci.yml
git commit -m "ci: 接 10 个 missing_docs 结构变体测试进 CI (#273 Part #2 D2)

在 'Check mod reachability' step 的 run block，test_workspace_missing_docs 之后、
check_mod_reachability.py 之前，加 python3 -m unittest 跑 10 模块的结构变体
(crate 特定 #![allow] 回归守卫)。沿用 ci.yml 多模块 backslash 续行格式。

跑模块 = 跑结构变体（D1 已删冗余 has_no_warnings），0.3s 文件扫描无 cargo 编译。"
```
Expected: commit 成功，`git status` clean。

---

## Task 4: 全量验证 (D3)

**Files:**
- 无文件改动（纯验证 task）。

**Interfaces:**
- Consumes: Task 1-3 全部提交完成（`git log` 有 3 个新 commit，`git status` clean）。
- Produces: 验证证据，确认本 change 达到 Design Doc §5 测试策略全部层级，可进入 comet verify 阶段。

- [x] **Step 1: 无残留 has_no_warnings（D1 完整性）**

Run: `grep -rn 'has_no_missing_docs_warnings' tools/tests/`
Expected: 空（无输出）。确认 18 处（baseline）全删。

- [x] **Step 2: 文件数 = 10（D1 完整性）**

Run: `ls tools/tests/test_openlark_*_missing_docs.py | wc -l`
Expected: `10`。

Run: `ls tools/tests/test_openlark_*_missing_docs.py | sort`
Expected: 恰好 10 行（Task 1 Step 3 列出的 10 文件），无 8 整删文件残留。

- [x] **Step 3: 无残留 import subprocess（D1b 清理完整性）**

Run: `grep -rn '^import subprocess' tools/tests/test_openlark_*_missing_docs.py`
Expected: 空。

- [x] **Step 4: workspace missing_docs 仍 0（#1 守门不变——删除安全性核心证据）**

Run: `cargo doc --workspace --all-features 2>&1 | grep -c 'missing documentation for'`
Expected: `0`。这是"删 18 冗余 has_no_warnings 安全"的核心证据——workspace 级测试持续覆盖"无 missing_docs"。若非 0，停止——本 change 不应引入 missing_docs（未改 Rust 代码），可能是 base-ref 已有问题或环境异常，需排查（非本 change 范围）。

> 注：此步耗时较长（cargo doc 全量）。若已确认本 change 未触任何 .rs 文件（`git diff --name-only d29c87fafe2131d6805aaed0ff353bec8cf0eb7f HEAD -- '*.rs'` 应为空），可酌情跳过，但 Design Doc §5 将其列为回归守门，建议实跑留证。

- [x] **Step 5: cargo fmt --check + just lint 通过（CI lint job 模拟）**

Run: `cargo fmt --all -- --check`
Expected: exit 0（本 change 未改 .rs，fmt 必通过；CI lint job 第一步即此，参见 memory `run-cargo-fmt-check-before-push.md`）。

Run: `just lint`
Expected: exit 0（clippy --all-features + --no-default-features 全过；本 change 未改 Rust 代码，lint 不受影响）。

- [x] **Step 6: ci.yml yaml 合法（D2 完整性复校）**

Run: `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml')); print('yaml OK')"`
Expected: `yaml OK`（Task 3 Step 3 已验，此处复验确认提交后状态）。

- [x] **Step 7: CI run block 命令顺序正确（D2 终态校验）**

Run: `sed -n '111,126p' .github/workflows/ci.yml`
Expected: 与 Task 3 Step 4 输出一致——`test_check_mod_reachability` → `test_workspace_missing_docs` → `python3 -m unittest \` + 10 续行 → `check_mod_reachability.py`。

- [x] **Step 8: git log 确认 3 个 commit（本 change 改动完整性）**

Run: `git log --oneline d29c87fafe2131d6805aaed0ff353bec8cf0eb7f..HEAD`
Expected: 恰好 3 行（D1a 删 8 文件、D1b 改 9 文件、D2 ci.yml 接线）。若多于 3，有意外提交需 squash 或排查；若少于 3，某 task 漏提交。

- [x] **Step 9: 推送前最终状态确认**

Run: `git status`
Expected: `clean working tree`（nothing to commit）。

Run: `git rev-parse HEAD`
Expected: 记录此 SHA，供 comet verify 阶段引用。

**验证全部通过后**：本 change 实施完成，可进入 comet verify 阶段（`comet-state next cleanup-missing-docs-tests` 决定下一步 skill）。

---

## Self-Review 记录

**1. Spec coverage**（对照 Design Doc §3 + tasks.md）：
- D1a 删 8 整文件 → Task 1（Step 1 前置校验 + Step 2 git rm + Step 3 文件数 + Step 4 提交）。✓
- D1b 改 9 文件 → Task 2（Step 1 校验 + Step 2 删 import + Step 3 删方法 + Step 4 无残留 + Step 5 py_compile + Step 6 实跑 + Step 7 提交）。✓
- 保留 workflow_narrow 不动 → Global Constraints 明示 + Task 1/2 均不涉该文件。✓
- D2 ci.yml 接 10 模块 → Task 3（Step 1 锚点校验 + Step 2 Edit + Step 3 yaml + Step 4 run block + Step 5 实跑 + Step 6 提交）。✓
- D3 验证 → Task 4（9 步覆盖 has_no_warnings 空 / 文件数 10 / subprocess 空 / workspace missing_docs 0 / fmt+lint / yaml / run block 顺序 / commit 数 / clean tree）。✓ tasks.md 3.1-3.4 全覆盖。

**2. Placeholder scan**：无 TBD/TODO；每步均含具体命令、预期输出、失败回退路径。代码块（改造前后范例、yaml 改后块、bash 命令）均为完整可执行内容，非占位。

**3. Type/name consistency**：模块名 10 处（Task 3 Step 2/5、Task 4 Step 7）全程一致——communication/docs/helpdesk/hr/mail/meeting/platform/protocol/workflow/workflow_narrow；方法名 `has_no_missing_docs_warnings` 全程拼写一致；文件路径 `tools/tests/test_openlark_*_missing_docs.py` 全程一致；ci.yml 行号锚点 112-115 / 111-115 与 base-ref 一致。
