---
change: enforce-bare-urls
design-doc: docs/superpowers/specs/2026-07-01-enforce-bare-urls-design.md
base-ref: 9221ca0fdda47ff61076a9d2c03eca943f859390
---

# enforce-bare-urls 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**目标：** 清零 workspace 全部 1578 条 `rustdoc::bare_urls` warning，并用 workspace `[.rustdoc] bare_urls = "deny"` + CI 解压锁死，杜绝复发。

**架构：** 五位一体——(1) codegen 渲染器改发射 `<URL>`（防新增）；(2) Python 批量脚本包裹现存 1578 裸 URL（清存量，`cargo doc` 闭环验证零警告）；(3) 根 `Cargo.toml` 新增 `[workspace.lints.rustdoc] bare_urls = "deny"`（锁死）；(4) CI doc job 移除 `-A rustdoc::bare_urls`（让 deny 生效）；(5) capability spec（open 阶段已起草，无需改）。改动 1-4 相互依存，缺一即 CI 红。

**技术栈：** Rust workspace（rustdoc lint）、Python 3（regex 批量脚本）、YAML（CI）、TOML（Cargo workspace lints）。

## Global Constraints

- **范围锁定**：仅治理 `rustdoc::bare_urls`（OpenSpec issue #273 Part A 现象 B / A2）。`missing_docs` 一致性（A1）、codegen `-A missing_docs` 闭环另案，**不得混入**。
- **裸 URL 全部包裹为 `<URL>`**（rustdoc 规范），格式：`//! docPath: <https://...>`。
- **代码改动必须最小化**：本 change 不改任何 doc comment 的文案文本，仅包裹 URL；不顺手清理无关代码。
- **lint 等级用 `deny`**（非 `warn`）：靠存量清零 + codegen 防新增 + CI 解压三重兜底。
- **MSRV 锁定**：CI msrv job 用 `.github/msrv/Cargo.lock`（pinned），本 change 未改依赖无需同步；但仍需跑 `--locked` 验证无回归。
- **本地验证须跑 `cargo fmt --check`**：CI lint job 第一步是 fmt --check，clippy 通过 ≠ fmt 通过。
- **clippy 须跑双模式**：`--all-features`（just lint）+ `--no-default-features`（CI 同款）。

---

## 文件结构

| 文件 | 角色 | 动作 |
|------|------|------|
| `tools/api_contracts/codegen_render.py:69-76` | codegen `_module_doc()` 发射 `docPath:` URL（D2 改造点） | 修改 |
| `tools/restructure_hr.py:46` | 一次性 HR 重构脚本，同形态 `docPath:` | 修改（一致性） |
| `tools/wrap_bare_urls.py` | 一次性批量包裹脚本，复发复用 | 新建 |
| `crates/**/*.rs` | 1578 处裸 URL（94% `docPath:` codegen + 4.8% 手写 `文档:`） | 脚本批量修改 |
| `Cargo.toml:62-71` | workspace lints，当前有 `[.rust]`/`[.clippy]`，**无 `[.rustdoc]`** | 新增段 |
| `.github/workflows/ci.yml:44` | doc job `RUSTDOCFLAGS="-D warnings -A rustdoc::bare_urls"` | 修改 |
| `openspec/changes/enforce-bare-urls/specs/rustdoc-bare-urls/spec.md` | capability（open 阶段已起草） | 无需改 |

---

## Task 0: 捕获 1578 baseline（迁移前快照）

**Files:**
- 无（只读验证）

**目的：** 锁定迁移前 baseline，迁移后 `cargo doc` 闭环对比用。Design Doc §1 实测 1578。

- [x] **Step 1: 跑 cargo doc 捕获 baseline warning 数**

```bash
cargo doc --workspace --all-features --no-deps 2>&1 | grep -c 'rustdoc::bare_urls'
```

Expected: `1578`（与 Design Doc §1 一致；若数值偏差 >5，停止并回 Design Doc 核对）

- [x] **Step 2: 落盘 baseline warning 列表供迁移后 diff**

```bash
cargo doc --workspace --all-features --no-deps 2>&1 | grep 'rustdoc::bare_urls' > /tmp/bare-urls-before.txt
wc -l /tmp/bare-urls-before.txt
```

Expected: 行数 = Step 1 数值；列表留作 Task 4 抽样审计参考。

- [x] **Step 3: 确认当前 CI 仍以 `-A` 压制（背景核对）**

Run: `grep 'RUSTDOCFLAGS' /Users/zool/workspace/openlark/.github/workflows/ci.yml`
Expected: 看到 `RUSTDOCFLAGS: "-D warnings -A rustdoc::bare_urls"`——确认 CI 现状是压制态，Task 6 将解压。

> Task 0 不产生 commit，仅为后续任务建立证据基线。

---

## Task 1: codegen 渲染器发射 `<URL>`（D2，防复发前置）

**Files:**
- Modify: `tools/api_contracts/codegen_render.py:69-76`（`_module_doc` 函数）
- Modify: `tools/restructure_hr.py:46`

**Interfaces:**
- Produces: codegen 后续产物（含 `--write` 落盘或离线渲染）的 `//! docPath:` 行 URL 全部为 `<URL>` 形态，Task 5.5 验证。

**为什么在批量修复之前做：** deny 锁死后任何 codegen `--write` 会立刻 CI 红——codegen 改造是 deny 的前置必要条件（Design Doc §3 改动 1）。

- [x] **Step 1: 读 `_module_doc` 确认发射点**

Run: `sed -n '69,76p' /Users/zool/workspace/openlark/tools/api_contracts/codegen_render.py`
Expected 看到：
```python
def _module_doc(ir: ApiIR) -> str:
    return (
        f"//! {ir.name}\n"
        f"//!\n"
        f"//! docPath: {ir.doc_path}\n"
        f"//!\n"
        f"//! {CODEGEN_MARKER}（风格 A）。手工修改将在下次生成时覆盖。"
    )
```

确认 `ir.doc_path` 是唯一 URL 发射点（`_module_doc` 只发射 `docPath:` 一行 URL，不发射 `文档:`）。

- [x] **Step 2: 修改 codegen_render.py 包裹 `<URL>`**

将 `tools/api_contracts/codegen_render.py` 第 73 行：

```python
        f"//! docPath: {ir.doc_path}\n"
```

改为：

```python
        f"//! docPath: <{ir.doc_path}>\n"
```

- [x] **Step 3: 修改 restructure_hr.py 同形态包裹**

将 `tools/restructure_hr.py` 第 46 行：

```python
//!
//! docPath: {doc_path}
```

中的 `//! docPath: {doc_path}` 改为 `//! docPath: <{doc_path}>`（即 URL 两端加 `<` `>`）。

- [x] **Step 4: 单元验证 codegen 渲染输出含 `<URL>`**

```bash
cd /Users/zool/workspace/openlark && python3 -c "
import sys; sys.path.insert(0, 'tools/api_contracts')
from codegen_render import _module_doc
# 构造最小 stub ir，只验证 docPath 行
class IR: 
    name='test'; doc_path='https://open.feishu.cn/x/y'
out = _module_doc(IR())
assert '//! docPath: <https://open.feishu.cn/x/y>' in out, out
print('OK:', [l for l in out.splitlines() if 'docPath' in l])
"
```

Expected: 打印 `OK: ['//! docPath: <https://open.feishu.cn/x/y>']`（注意 `<>` 包裹）。

> 若 `codegen_render` import 失败（缺依赖/路径差异），改为静态 grep 验证：`grep 'docPath:' tools/api_contracts/codegen_render.py` 应见 `f"//! docPath: <{ir.doc_path}>\n"`。

- [x] **Step 5: 提交**

```bash
git add tools/api_contracts/codegen_render.py tools/restructure_hr.py
git commit -m "feat(codegen): docPath 发射 <URL> 替代裸 URL（enforce-bare-urls D2 防复发前置）"
```

---

## Task 2: 写批量包裹脚本 `tools/wrap_bare_urls.py`

**Files:**
- Create: `tools/wrap_bare_urls.py`

**Interfaces:**
- Produces: `tools/wrap_bare_urls.py`，幂等遍历 `crates/**/*.rs`，对 `//!`/`///` doc comment 行用 regex `(?<![<(])(https?://[^\s<>)]+)` → `<\1>` 包裹裸 URL；Task 3 调用执行。

**设计要点（Design Doc §3 改动 2）：**
- `(?<![<(])` lookbehind：跳过已包裹的 `<URL` 和 markdown 链接 `(url)`，**防重复包裹 + 防破坏 markdown**。
- `[^\s<>)]+` 匹配 URL body：遇空格/`<`/`>`/`)` 结束，避免贪心吞行尾标点。
- 仅 `//!`/`///` doc comment：bare_urls 是 rustdoc lint，普通 `//` 不触发；限范围降低误伤。
- 幂等：lookbehind 保证二次运行零改动（已 `<URL>` 不再包裹）。

- [x] **Step 1: 创建脚本**

新建 `tools/wrap_bare_urls.py`：

```python
#!/usr/bin/env python3
"""把 crates/**/*.rs 的 //! / /// doc comment 中的裸 URL 包裹为 <URL>。

用于 enforce-bare-urls change（清零 rustdoc::bare_urls）。
设计：
  - 仅处理 //! / /// doc comment（bare_urls 是 rustdoc lint，普通 // 不触发）。
  - lookbehind (?<![<(]) 跳过已包裹的 <URL 和 markdown 链接 (url)，防重复包裹。
  - URL body [^\\s<>)]+ 遇空格/<>/) 结束，避免贪心吞行尾标点。
幂等：重复运行零改动。
"""
import re
import sys
import pathlib

URL = re.compile(r'(?<![<(])(https?://[^\s<>)]+)')


def wrap_file(path: pathlib.Path) -> int:
    """返回该文件被包裹的 URL 数（0 = 无改动）。"""
    text = path.read_text(encoding='utf-8')
    lines = text.splitlines(keepends=True)
    changed = 0
    for i, line in enumerate(lines):
        s = line.lstrip()
        if s.startswith('//!') or s.startswith('///'):
            new_line, n = URL.subn(r'<\1>', line)
            if n:
                lines[i] = new_line
                changed += n
    if changed:
        path.write_text(''.join(lines), encoding='utf-8')
    return changed


def main(root: str = 'crates') -> int:
    total = 0
    files = 0
    for f in pathlib.Path(root).rglob('*.rs'):
        n = wrap_file(f)
        if n:
            files += 1
            total += n
    print(f'wrapped {total} bare URLs across {files} files')
    return 0


if __name__ == '__main__':
    root = sys.argv[1] if len(sys.argv) > 1 else 'crates'
    sys.exit(main(root))
```

- [x] **Step 2: 先做干跑验证——脚本逻辑正确（不写盘）**

```bash
cd /Users/zool/workspace/openlark && python3 -c "
import re, pathlib
URL = re.compile(r'(?<![<(])(https?://[^\s<>)]+)')
cases = [
    ('//! docPath: https://open.feishu.cn/a/b\n', '//! docPath: <https://open.feishu.cn/a/b>\n'),
    ('//! 文档: https://open.feishu.cn/x\n', '//! 文档: <https://open.feishu.cn/x>\n'),
    # 已包裹（幂等）
    ('//! docPath: <https://open.feishu.cn/a/b>\n', '//! docPath: <https://open.feishu.cn/a/b>\n'),
    # markdown 链接不破坏
    ('/// see [docs](https://x.com/y)\n', '/// see [docs](<https://x.com/y>)\n'),
    # 普通 // 不处理
    ('// https://x.com\n', '// https://x.com\n'),
    # 行尾多 URL
    ('//! a https://x.com b https://y.com\n', '//! a <https://x.com> b <https://y.com>\n'),
    # 尾随 ) 结束
    ('//! (see https://x.com)\n', '//! (see <https://x.com>)\n'),
]
ok = True
for inp, exp in cases:
    got = URL.sub(r'<\1>', inp) if inp.lstrip().startswith(('//!','///')) else inp
    flag = 'OK ' if got == exp else 'FAIL'
    if got != exp: ok = False
    print(f'{flag} | in={inp.rstrip()} | got={got.rstrip()}')
print('ALL OK' if ok else 'SOME FAILED')
"
```

Expected: 全部 `OK `，末行 `ALL OK`。
重点关注：
- markdown 链接 `[docs](https://x.com/y)` → `[docs](<https://x.com/y>)`（URL 被 `<>` 包裹但 `()` 保留——rustdoc 接受此形态，可接受）。
- 已包裹的 `<URL>` 不被二次处理（幂等）。
- 普通 `//` 不动。

> 若 markdown 链接的 `(url)` → `(<url>)` 行为不可接受，需调整 lookbehind 同时排除 `(`——但 rustdoc 对 `(<url>)` 不报警告，且全仓 1578 处无 markdown 链接形态（Design Doc §2：全在 `//! docPath:` / `文档:` 行），实际不触发此边界。

- [x] **Step 3: 提交脚本**

```bash
git add tools/wrap_bare_urls.py
git commit -m "feat(tools): 新增 wrap_bare_urls.py 批量包裹裸 URL 脚本（enforce-bare-urls D1）"
```

---

## Task 3: 跑脚本批量修复 + 闭环验证 `cargo doc` 零警告（核心断言）

**Files:**
- Modify: `crates/**/*.rs`（脚本批量改，预计 ~1578 处 URL）

**Interfaces:**
- Produces: workspace 全部 `//!`/`///` 行的 URL 均为 `<URL>` 形态；`cargo doc` 零 bare_urls warning。

> **核心断言任务**：`cargo doc --workspace --all-features 2>&1 | grep -c bare_urls` = **0**（迁移前 1578）。这是整个 change 的成败判据。

- [x] **Step 1: 跑脚本**

```bash
cd /Users/zool/workspace/openlark && python3 tools/wrap_bare_urls.py
```

Expected: 打印 `wrapped ~1578 bare URLs across ~600+ files`（数值以实际为准，应与 Task 0 Step 1 baseline 接近）。

- [x] **Step 2: 核心断言——cargo doc 零 bare_urls**

```bash
cargo doc --workspace --all-features --no-deps 2>&1 | grep -c 'rustdoc::bare_urls'
```

Expected: `0`

> 这是 **整个 change 的核心断言**。若非 0：加载 superpowers:systematic-debugging，可能原因（1）脚本漏匹配某 URL 形态（如含 `#` 锚点的 URL）；（2）有 URL 在普通 `//` 注释而非 doc comment（rustdoc 仅扫 doc comment，但若误报则需核查）。回 Task 2 调 regex 后重跑（脚本幂等，可直接重跑）。

- [x] **Step 3: cargo doc 总 warning 数（应仅 bare_urls 一种，现在应为 0）**

```bash
cargo doc --workspace --all-features --no-deps 2>&1 | grep -c '^warning:'
```

Expected: `0`（Design Doc §2：全量 warning 只有 bare_urls 一种，清零后应 0）。

- [x] **Step 4: 提交（大 diff，单独 commit）**

```bash
git add -A crates/
git commit -m "fix(docs): 批量包裹 1578 处裸 URL 为 <URL>（清零 rustdoc::bare_urls，enforce-bare-urls D1）"
```

---

## Task 4: git diff 抽样人工审计（防误伤兜底）

**Files:**
- 无（只读审计，不改代码）

**目的：** Design Doc §7 风险——脚本可能误伤非 URL 文本。Task 3 的 `cargo doc` 闭环只能证明「无残留裸 URL」，证明不了「无误伤」。本任务抽样审计兜底。

- [x] **Step 1: 审计 77 处手写 `文档:` URL（全部）**

```bash
cd /Users/zool/workspace/openlark && git diff HEAD~1 -- 'crates/**/*.rs' | grep -B1 -A1 '文档:' | head -100
```

Expected: 每处 `//! 文档: <https://...>` 形态，URL 之外中文文本未被改。
重点核对：
- `文档:` 标签保留中文未变。
- URL 后无尾随文本被吞（如 `文档: <https://x.com>（说明）` 的「（说明）」应在）。

- [x] **Step 2: 审计多 URL 行**

```bash
cd /Users/zool/workspace/openlark && git diff HEAD~1 -- 'crates/**/*.rs' | grep -E '\+.*https?://.*https?://' | head -20
```

Expected: 每行多 URL 均各自包裹 `<URL>`，无遗漏、无贪心吞跨 URL 文本。

- [x] **Step 3: 审计非 URL 文本未被误伤（随机抽样 10 文件）**

```bash
cd /Users/zool/workspace/openlark && git diff HEAD~1 --name-only -- 'crates/**/*.rs' | shuf -n 10
```

对每个文件 `git diff HEAD~1 -- <file>`，逐行确认：
- 只有 URL 行被改（`+//! ... <https://...>`）。
- 非 URL 的 doc comment 文本未变（中文描述、字段说明等）。
- 普通代码行 `//` 未动。

- [x] **Step 4: 审计无非 doc comment 误伤**

```bash
cd /Users/zool/workspace/openlark && git diff HEAD~1 -- 'crates/**/*.rs' | grep '^+' | grep -v '://' | grep -v '^+++' | head -20
```

Expected: 空输出（新增行都应包含 `://` URL 或为 doc comment）。若有非 URL 新增行，说明脚本误伤，需回查。

> Task 4 是只读审计，不产生 commit。若发现误伤，回 Task 2 调脚本 + 重跑（脚本幂等）+ 重做 Task 3 闭环。审计通过后才进 Task 5。

---

## Task 5: workspace `[workspace.lints.rustdoc] bare_urls = "deny"`（D3 锁死）

**Files:**
- Modify: `Cargo.toml`（在第 71 行 `[workspace.lints.clippy]` 段之后新增 `[workspace.lints.rustdoc]` 段）

**Interfaces:**
- Produces: workspace 全部 crate（经 `[lints] workspace = true` 继承）的 `bare_urls` lint 升级为 `deny`。

- [x] **Step 1: 确认 20 业务 crate + 根包均已继承 workspace lints**

```bash
cd /Users/zool/workspace/openlark && grep -rl 'lints.workspace = true' crates/ src/ Cargo.toml | wc -l
```

Expected: ≥ 21（20 业务 crate + 根包）。若有未继承的 crate，记录但**不在本任务处理**（Design Doc §3 改动 3：应已全部继承；若发现未继承，属另一致性问题，A1 范畴）。

- [x] **Step 2: 在根 Cargo.toml 新增 `[workspace.lints.rustdoc]` 段**

在 `Cargo.toml` 第 71 行（`module_name_repetitions = "allow"` 之后、第 73 行 `[workspace.dependencies]` 之前）插入：

```toml

[workspace.lints.rustdoc]
bare_urls = "deny"
```

即把 `[workspace.lints.clippy]` 段结束到 `[workspace.dependencies]` 之间加新段。修改后第 67-73 行区域应为：

```toml
[workspace.lints.clippy]
all = { level = "warn", priority = -1 }
# 允许某些常见的例外
multiple_crate_versions = "allow"
module_name_repetitions = "allow"

[workspace.lints.rustdoc]
bare_urls = "deny"

[workspace.dependencies]
```

- [x] **Step 3: 验证 cargo doc 在 deny 下仍零 warning（lint 升级后通过）**

```bash
cargo doc --workspace --all-features --no-deps 2>&1 | tee /tmp/doc-after-deny.log | grep -cE '^(warning|error)'
```

Expected: `0`
并确认无 bare_urls 相关 error：

```bash
grep -c 'bare_urls' /tmp/doc-after-deny.log
```

Expected: `0`（deny 下若有残留会变 error 而非 warning；由 Task 3 清零保证）。

- [x] **Step 4: 验证 cargo build 接受新 lint 配置**

```bash
cargo build --workspace --all-features 2>&1 | tail -5
```

Expected: `Finished`（无 lint 报错）。

- [x] **Step 5: 提交**

```bash
git add Cargo.toml
git commit -m "feat(workspace): 新增 [workspace.lints.rustdoc] bare_urls=deny 锁死裸 URL（enforce-bare-urls D3）"
```

---

## Task 6: CI doc job 移除 `-A rustdoc::bare_urls`（D4 解压）

**Files:**
- Modify: `.github/workflows/ci.yml:44`

**Interfaces:**
- Produces: CI doc job `RUSTDOCFLAGS` 仅保留 `-D warnings`，让 Task 5 的 workspace deny 在 CI 生效。

**安全性依据（Design Doc §2）：** cargo doc 全量 warning 实测只有 `rustdoc::bare_urls` 一种，无 `broken_intra_doc_links` / `private_doc_tests` 等 → 移除 `-A` 后 `-D warnings` 只会因 bare_urls 失败（已被 Task 3 清零），**解压零意外**。

- [x] **Step 1: 修改 ci.yml RUSTDOCFLAGS**

将 `.github/workflows/ci.yml` 第 44 行：

```yaml
      RUSTDOCFLAGS: "-D warnings -A rustdoc::bare_urls"
```

改为：

```yaml
      RUSTDOCFLAGS: "-D warnings"
```

- [x] **Step 2: 本地 CI 模拟——RUSTDOCFLAGS=-D warnings cargo doc exit 0**

```bash
cd /Users/zool/workspace/openlark && RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps; echo "EXIT=$?"
```

Expected: `EXIT=0`（CI 模拟，移除 `-A` 后 doc 不应 fail）。

> 若 EXIT≠0：加载 systematic-debugging。可能暴露 Task 3 未清的残留 bare_urls（变 error），或 Design Doc §2 未预见的其他 rustdoc lint。前者回 Task 2/3 补；后者记录并暂停（属范围外，需 design 决策）。

- [x] **Step 3: 提交**

```bash
git add .github/workflows/ci.yml
git commit -m "ci(doc): 移除 -A rustdoc::bare_urls 压制，让 workspace deny 生效（enforce-bare-urls D4）"
```

---

## Task 7: 完整验证（D1-D4 全过）

**Files:**
- 无（只读验证套件）

**对应 OpenSpec tasks 5.1-5.6 + Design Doc §6 测试矩阵。**

- [x] **Step 1: cargo fmt --check（CI lint job 第一步）**

```bash
cd /Users/zool/workspace/openlark && cargo fmt --check; echo "EXIT=$?"
```

Expected: `EXIT=0`（本 change 只动 doc comment URL 包裹 + Cargo.toml/yaml，不应引入格式问题；但 CI 第一步即此，必须验证）。

- [x] **Step 2: cargo doc deny 下 0 warning + exit 0（核心断言最终确认）**

```bash
cd /Users/zool/workspace/openlark && cargo doc --workspace --all-features --no-deps 2>&1 | tee /tmp/doc-final.log
echo "warnings=$(grep -c '^warning:' /tmp/doc-final.log) bare_urls=$(grep -c 'bare_urls' /tmp/doc-final.log)"
```

Expected: `warnings=0 bare_urls=0`。

- [x] **Step 3: just lint（clippy --all-features）**

```bash
cd /Users/zool/workspace/openlark && just lint; echo "EXIT=$?"
```

Expected: `EXIT=0`。

- [x] **Step 4: clippy --no-default-features（CI 同款，门控代码测试）**

```bash
cd /Users/zool/workspace/openlark && cargo clippy --workspace --all-targets --no-default-features 2>&1 | tail -5; echo "EXIT=${PIPESTATUS[0]}"
```

Expected: 无 error，EXIT=0。

> MEMORY 提醒：CI lint job 跑 `clippy --workspace --all-targets --no-default-features`；调用 cfg(feature) 门控方法的测试自身要 `#[cfg(feature)]`。本 change 不新增门控代码，应无影响，但仍需验证。

- [x] **Step 5: cargo build --all-features**

```bash
cd /Users/zool/workspace/openlark && cargo build --workspace --all-features 2>&1 | tail -3; echo "EXIT=${PIPESTATUS[0]}"
```

Expected: `Finished`，EXIT=0。

- [x] **Step 6: codegen 渲染抽样验证 `<URL>`（Spec Req 3）**

复用 Task 1 Step 4 的单元验证，确认 codegen 产物 docPath 行为 `<URL>`：

```bash
cd /Users/zool/workspace/openlark && grep 'docPath' tools/api_contracts/codegen_render.py
```

Expected: 见 `f"//! docPath: <{ir.doc_path}>\n"`（`<>` 包裹）。

- [x] **Step 7: MSRV --locked 验证（无回归）**

MEMORY 提醒：CI msrv job 用 `.github/msrv/Cargo.lock` pinned。本 change 未改依赖，无需同步 lockfile，但验证 `--locked` 通过。

```bash
cd /Users/zool/workspace/openlark && cargo +1.88 check --workspace --all-features --locked 2>&1 | tail -3; echo "EXIT=${PIPESTATUS[0]}"
```

Expected: `Finished`，EXIT=0。

> 若本地无 1.88 toolchain：`rustup toolchain install 1.88` 后重跑。或按 MEMORY 用 `docker run --rm -v "$PWD":/work -w /work rust:1.88 sh -c 'apt-get update && apt-get install -y protobuf-compiler && cargo check --workspace --all-features --locked'`。

- [x] **Step 8: 推送前总确认——本地 main clean**

```bash
cd /Users/zool/workspace/openlark && git status --short && git log --oneline -6
```

Expected: 工作区 clean；最近 6 commit 包含 Task 1/2/3/5/6 的 5 个 commit（codegen + 脚本 + 批量修 + workspace deny + CI 解压）。

---

## Self-Review 核对

**Spec 覆盖（OpenSpec tasks.md 5 组 17 任务 ↔ 本计划 8 任务）：**
- 组 1 codegen（1.1-1.3）→ Task 1 ✓
- 组 2 批量修（2.1-2.4）→ Task 2（2.1 脚本）+ Task 3（2.2 跑 + 2.3 闭环）+ Task 4（2.4 抽样）✓
- 组 3 workspace deny（3.1-3.2）→ Task 5 ✓
- 组 4 CI 解压（4.1-4.2）→ Task 6 ✓（4.2「暴露其他 rustdoc lint」由 Task 6 Step 2 CI 模拟覆盖）
- 组 5 验证（5.1-5.6）→ Task 7 全覆盖 ✓

**核心断言贯穿：** Task 0 baseline（1578）→ Task 3 Step 2 闭环（=0）→ Task 5 Step 3 deny 不触发新错 → Task 6 Step 2 CI 模拟 exit 0 → Task 7 Step 2 最终确认。

**防误伤双保险：** Task 3 cargo doc 闭环（无残留）+ Task 4 git diff 抽样（无误伤），对应 Design Doc §7 风险。

**placeholder 扫描：** 无 TBD/TODO；每个 Step 含具体命令与 Expected；批量脚本 Step 含完整可运行 Python 代码。

**类型/命名一致：** `_module_doc` / `ir.doc_path` / `wrap_bare_urls.py` / `URL` regex 在各任务间一致；`<URL>` 形态贯穿。

---

## 执行交接

计划已保存至 `docs/superpowers/plans/2026-07-01-enforce-bare-urls.md`。两种执行方式：

1. **Subagent-Driven（推荐）** — 每个 task 派发独立 subagent，任务间双审查（spec compliance + code quality），快速迭代。
2. **Inline Execution** — 本会话内用 executing-plans 批量执行，检查点暂停审查。

**选哪种？**
