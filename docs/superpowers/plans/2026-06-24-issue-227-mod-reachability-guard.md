# Issue #227 mod 可达性 CI 守卫 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 新增 `tools/check_mod_reachability.py` 守卫，以 rustc dep-info 为基准检测「写了但没在 mod 链挂载」的孤儿 `.rs` 文件，接入 CI lint job，关闭 #227。

**Architecture:** cargo-driven——用 `cargo rustc -p <crate> --lib --all-features -- --emit=dep-info` 生成编译器实际编译的文件清单（`.d`），与 `src/` 目录树 diff，差集即孤儿。用编译器自身解析，零误报（完美处理 `#[path]`/inline mod/cfg gate）。

**Tech Stack:** Python 3.12（标准库），unittest，GitHub Actions。

**参考文件（只读）:**
- `tools/compare_api_catalogs.py` — 既有工具风格（argparse + dataclass + main()）
- `tools/tests/test_compare_api_catalogs.py` — 既有测试风格（importlib 加载 + unittest）
- `.github/workflows/ci.yml:87-109` — `lint` job，集成点（clippy --all-features 生成 .d）
- 设计：`docs/superpowers/specs/2026-06-24-issue-227-mod-reachability-guard-design.md`

**已验证事实（实现时直接用）:**
- dep 文件路径：`target/debug/libopenlark_<suffix>.d`（`openlark-communication` → `libopenlark_communication.d`；crate 名连字符→下划线）
- `.d` 第 1 行格式：`<rlib>: <src1.rs> <src2.rs> ...`（空格分隔，可能换行续行 `\`）
- 全部 20 个 `openlark-*` crate 都有 `src/lib.rs`（`--lib` 安全）
- main 分支上 `event/event/v1/outbound_ip/list.rs` 是已知孤儿（#225 未合并）——回归验证基线

---

## 文件结构

| 动作 | 路径 | 职责 |
|---|---|---|
| 新建 | `tools/check_mod_reachability.py` | 守卫脚本（纯函数 + IO 分离） |
| 新建 | `tools/tests/test_check_mod_reachability.py` | 4 个单测 |
| 修改 | `.github/workflows/ci.yml` | lint job 追加守卫步骤 |

---

## Task 1: check_mod_reachability.py 核心纯函数 + 单测（TDD）

**Files:**
- Create: `tools/check_mod_reachability.py`
- Create: `tools/tests/test_check_mod_reachability.py`

- [ ] **Step 1: 写脚本骨架 + 纯函数（parse_dep_info, diff_orphans）**

Create `tools/check_mod_reachability.py` with content:

```python
#!/usr/bin/env python3
"""检测「写了但没在 mod 链挂载」的孤儿 .rs 文件。

以 rustc dep-info（cargo rustc --emit=dep-info）为基准，与 src/ 目录树 diff，
找出从未被任何 mod 链引用的孤儿文件。用编译器自身解析，零误报。

用法：
    python3 tools/check_mod_reachability.py            # 全 workspace
    python3 tools/check_mod_reachability.py --crate openlark-communication
"""

from __future__ import annotations

import argparse
import glob
import subprocess
import sys
from pathlib import Path
from typing import Iterable, Set

REPO_ROOT = Path(__file__).resolve().parents[1]
CRATES_DIR = REPO_ROOT / "crates"
TARGET_DEBUG = REPO_ROOT / "target" / "debug"


def parse_dep_info(dep_content: str, crate_src_dir: Path) -> Set[Path]:
    """从 .d 文件内容提取属于本 crate src/ 的 .rs 文件集合。

    .d 第 1 行格式：`<rlib>: <src1.rs> <src2.rs> ...`（空格分隔，可能 `\` 换行续行）。
    只保留路径落在 crate_src_dir 下的 .rs 文件。
    """
    src_prefix = str(crate_src_dir).replace("\\", "/").rstrip("/") + "/"
    # 合并续行（以 \ 结尾的行与下一行拼接）
    joined = dep_content.replace("\\\n", " ")
    files: Set[Path] = set()
    for line in joined.splitlines():
        if ":" not in line:
            continue
        # 取冒号后的目标列表（冒号前是产物 rlib 路径）
        targets = line.split(":", 1)[1]
        for tok in targets.split():
            tok = tok.strip().replace("\\", "/")
            if not tok.endswith(".rs"):
                continue
            # 用字符串前缀匹配（不要求路径存在），便于单测用虚拟路径
            if tok.startswith(src_prefix) or tok.startswith("/" + src_prefix.lstrip("/")):
                files.add(Path(tok))
    return files


def diff_orphans(on_disk: Iterable[Path], compiled: Iterable[Path]) -> Set[Path]:
    """返回 on_disk 有、compiled 没有的孤儿文件集合。"""
    return set(on_disk) - set(compiled)


def list_src_files(crate_src_dir: Path) -> Set[Path]:
    """列出 crate src/ 下所有 .rs 文件（排除 tests/examples/benches，但 src/ 下通常无）。"""
    return {p for p in crate_src_dir.rglob("*.rs")}


def crate_dep_file(crate_name: str) -> Path:
    """crate 名 -> dep 文件路径。openlark-communication -> libopenlark_communication.d"""
    suffix = crate_name.replace("-", "_")
    matches = sorted(TARGET_DEBUG.glob(f"lib{suffix}*.d"), key=lambda p: p.stat().st_mtime)
    return matches[-1] if matches else TARGET_DEBUG / f"lib{suffix}.d"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--crate", help="仅检查单个 crate（默认全 workspace）")
    args = parser.parse_args()

    crate_dirs = sorted(d for d in CRATES_DIR.iterdir() if (d / "src" / "lib.rs").is_file())
    if args.crate:
        crate_dirs = [d for d in crate_dirs if d.name == args.crate]
        if not crate_dirs:
            print(f"❌ 未找到 crate：{args.crate}", file=sys.stderr)
            return 2

    # 生成 dep-info（全 workspace 一次构建足够；单 crate 则只构建该 crate）
    target = args.crate or "--workspace"
    cmd = ["cargo", "build", "--lib", "--all-features"]
    if args.crate:
        cmd += ["-p", args.crate]
    else:
        # workspace 级 build 生成所有 .d；用 --workspace
        cmd = ["cargo", "build", "--workspace", "--all-features"]
    proc = subprocess.run(cmd, cwd=REPO_ROOT, capture_output=True, text=True)
    if proc.returncode != 0:
        print(f"❌ 编译失败（先修编译错误）：\n{proc.stderr}", file=sys.stderr)
        return 1

    all_orphans: list[tuple[str, Path]] = []
    for crate_dir in crate_dirs:
        crate_src = crate_dir / "src"
        dep = crate_dep_file(crate_dir.name)
        if not dep.is_file():
            print(f"⚠️  跳过 {crate_dir.name}：未找到 dep 文件 {dep.name}", file=sys.stderr)
            continue
        compiled = parse_dep_info(dep.read_text(encoding="utf-8"), crate_src)
        on_disk = list_src_files(crate_src)
        orphans = diff_orphans(on_disk, compiled)
        for o in sorted(orphans):
            all_orphans.append((crate_dir.name, o))

    if all_orphans:
        print(f"❌ 发现 {len(all_orphans)} 个孤儿文件（写了但未在 mod 链挂载）：\n")
        for crate, path in all_orphans:
            rel = path.relative_to(REPO_ROOT)
            print(f"  {crate}: {rel}")
        print("\n修复：在对应 crate 的 lib.rs / 上级 mod.rs 声明该模块，或删除该文件。")
        return 1

    print(f"✅ 全部 {len(crate_dirs)} 个 crate 无孤儿文件。")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
```

- [ ] **Step 2: 写 4 个单测**

Create `tools/tests/test_check_mod_reachability.py` with content:

```python
import importlib.util
import sys
import tempfile
import unittest
from pathlib import Path

MODULE_PATH = Path(__file__).resolve().parents[1] / "check_mod_reachability.py"
SPEC = importlib.util.spec_from_file_location("check_mod_reachability", MODULE_PATH)
mod = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
sys.modules[SPEC.name] = mod
SPEC.loader.exec_module(mod)


class TestParseDepInfo(unittest.TestCase):
    def test_parses_src_files_on_first_line(self):
        src_dir = Path("/repo/crates/openlark-fake/src")
        content = (
            "/repo/target/debug/libopenlark_fake.rlib: "
            "/repo/crates/openlark-fake/src/lib.rs "
            "/repo/crates/openlark-fake/src/a.rs "
            "/repo/crates/openlark-core/src/lib.rs\n"
        )
        result = mod.parse_dep_info(content, src_dir)
        names = {p.name for p in result}
        self.assertEqual(names, {"lib.rs", "a.rs"})

    def test_excludes_other_crates_and_non_rs(self):
        src_dir = Path("/repo/crates/openlark-fake/src")
        content = (
            "/repo/target/debug/libopenlark_fake.rlib: "
            "/repo/crates/openlark-fake/src/lib.rs "
            "/repo/crates/openlark-core/src/lib.rs "
            "/repo/crates/openlark-fake/Cargo.toml\n"
        )
        result = mod.parse_dep_info(content, src_dir)
        self.assertEqual({p.name for p in result}, {"lib.rs"})

    def test_handles_line_continuation(self):
        src_dir = Path("/repo/crates/openlark-fake/src")
        content = (
            "lib.rlib: /repo/crates/openlark-fake/src/lib.rs \\\n"
            "  /repo/crates/openlark-fake/src/b.rs\n"
        )
        result = mod.parse_dep_info(content, src_dir)
        self.assertEqual({p.name for p in result}, {"lib.rs", "b.rs"})


class TestDiffOrphans(unittest.TestCase):
    def test_finds_orphan(self):
        a = Path("/x/a.rs")
        b = Path("/x/b.rs")
        c = Path("/x/c.rs")
        result = mod.diff_orphans([a, b, c], [a, b])
        self.assertEqual(result, {c})

    def test_no_orphan_when_equal(self):
        f = Path("/x/a.rs")
        self.assertEqual(mod.diff_orphans([f], [f]), set())


if __name__ == "__main__":
    unittest.main()
```

- [ ] **Step 3: 跑单测**

Run: `python3 -m unittest tools.tests.test_check_mod_reachability -v`
Expected: OK — 5 tests passed（2 个 test class，5 个 test method）

- [ ] **Step 4: Commit**

```bash
git add tools/check_mod_reachability.py tools/tests/test_check_mod_reachability.py
git commit -m "feat(tools): 新增 mod 可达性守卫脚本 + 单测 (#227)"
```

---

## Task 2: 全 workspace 跑守卫，回归验证 event/ 孤儿

**Files:** 无改动（验证步）

- [ ] **Step 1: 单 crate 验证（communication，main 上 event/ 应为孤儿）**

Run: `python3 tools/check_mod_reachability.py --crate openlark-communication`
Expected: 退出码 1，报告含 `event/event/v1/outbound_ip/list.rs`（及同目录其他文件）。
这确认守卫能抓到 #225 修复前的真实 bug。

- [ ] **Step 2: 全 workspace 跑（看其他 crate 是否也有孤儿）**

Run: `python3 tools/check_mod_reachability.py`
Expected: 列出全仓所有孤儿（communication 的 event/ 在内）。**记录输出**——这是存量债务清单，
守卫上线后这些需由各自 PR 修复（本 issue 不修存量，见 spec 风险章节）。

- [ ] **Step 3: 反向验证（PR #225 分支上 event/ 孤儿消失）**

```bash
git stash  # 暂存本分支改动（若有）
git checkout fix/issue-216-connection-endpoint
python3 tools/check_mod_reachability.py --crate openlark-communication
git checkout fix/issue-227-mod-reachability-guard
git stash pop 2>/dev/null || true
```
Expected: 在 #225 分支上 communication 的 event/ 孤儿**消失**（守卫转绿），证明守卫随修复正确响应。

- [ ] **Step 4: 暂不提交**（Task 2 是验证，无代码改动）

---

## Task 3: 接入 CI lint job

**Files:**
- Modify: `.github/workflows/ci.yml`（lint job，第 107 行 `Run clippy (all features)` 之后）

- [ ] **Step 1: 在 lint job 追加 Python setup + 守卫两步**

在 `.github/workflows/ci.yml` 的 `lint:` job 内，找到：

```yaml
      - name: Run clippy (no default features)
        run: cargo clippy --workspace --lib --no-default-features -- -D warnings
```

在其**之前**（即 `Run clippy (all features)` 之后、`Run clippy (no default features)` 之前）插入：

```yaml
      - uses: actions/setup-python@v5
        with:
          python-version: "3.12"
      - name: Check mod reachability (no orphan src files)
        run: |
          python3 -m unittest tools.tests.test_check_mod_reachability
          python3 tools/check_mod_reachability.py
```

> 位置说明：放在 `clippy (all features)`（生成 .d）之后、`clippy (no default features)` 之前。
> 守卫用 `--all-features` 矩阵，与 `clippy (no default features)` 的 `--no-default-features` 无关，
> 顺序不影响正确性。

- [ ] **Step 2: 本地模拟 CI 验证守卫可运行**

Run:
```bash
python3 -m unittest tools.tests.test_check_mod_reachability && echo "TESTS OK"
python3 tools/check_mod_reachability.py --crate openlark-core 2>&1 | tail -3
```
Expected: 单测 OK；openlark-core（无已知孤儿）退出码 0。

- [ ] **Step 3: 校验 YAML 合法**

Run: `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))" && echo "YAML OK"`
Expected: `YAML OK`

- [ ] **Step 4: Commit**

```bash
git add .github/workflows/ci.yml
git commit -m "ci: lint job 接入 mod 可达性守卫 (#227)"
```

---

## Task 4: 开 PR 关闭 #227

**Files:** 无代码改动

- [ ] **Step 1: 推送 + 开 PR**

```bash
git push -u origin fix/issue-227-mod-reachability-guard

gh pr create --repo foxzool/openlark \
  --base main \
  --head fix/issue-227-mod-reachability-guard \
  --title "ci: mod 可达性守卫，防「写了但没接通」的死代码" \
  --label "priority:p1,type:ci,scope:quality" \
  --body "..."
```

- [ ] **Step 2: 确认 CI 守卫在 PR 上运行**

观察 PR 的 lint job 是否执行 `Check mod reachability` 步骤。
**注意**：因 main 上 communication/event/ 是已知孤儿（#225 未合并），守卫会让该 PR 的 lint job **失败**。
处理策略见 spec「风险与边界」：等 #225 合并后再合并本 PR，或本 PR 加临时 allowlist。

---

## 自检（Self-Review）

- **Spec 覆盖**：✅ 脚本 = Task 1；单测 = Task 1 Step 2；回归验证 = Task 2；CI 接入 = Task 3；DoD 每条映射 Task。
- **占位符扫描**：✅ 无 TBD；每个代码 step 含完整代码块；命令含 expected。
- **类型/接口一致**：✅ `parse_dep_info(content, src_dir) -> Set[Path]`、`diff_orphans(on_disk, compiled) -> Set[Path]`、`list_src_files(dir) -> Set[Path]`、`crate_dep_file(name) -> Path` 贯穿 Task 1/2/3。
- **时序依赖**：Task 2 依赖 Task 1（脚本存在）；Task 3 依赖 Task 1（单测存在）；顺序正确。
