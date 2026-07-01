---
comet_change: enforce-bare-urls
role: technical-design
canonical_spec: openspec
---

# enforce-bare-urls 技术设计

> 关联 OpenSpec change：`openspec/changes/enforce-bare-urls/`（canonical spec 以 OpenSpec 为准）。
> 范围：issue #273 Part A **现象 B（A2）**——清零 workspace `rustdoc::bare_urls` warning 并 deny 锁死。
> Part A1（missing_docs/一致性治理）另开 change；Part B（protocol workspace deps）已归档完结。

## 1. 背景与现状

OpenLark workspace 的 `rustdoc::bare_urls` lint 处于「CI 压制 + workspace 未治理」状态：

| 层 | 现状 |
|----|------|
| `[workspace.lints]`（根 Cargo.toml:62-71） | 有 `[.rust]` / `[.clippy]`，**无 `[.rustdoc]` 段**——bare_urls 未治理 |
| CI doc job（ci.yml:40-57） | `RUSTDOCFLAGS="-D warnings -A rustdoc::bare_urls"`——显式 allow 压制 |
| 实测 warning | **1578 条**（cargo doc --workspace --all-features），16 crate 分布 |
| 形态 | 94.4%（1515）codegen 注入的 `//! docPath: <裸URL>`；4.8%（77）手写 `//! 文档: <裸URL>`；全仓已 `<URL>` 规范化 = 0 |

每 crate 分布（降序）：hr 542、communication 352、meeting 215、platform 149、workflow 67、docs 56、security 39、cardkit 37、application 31、ai 22、auth 17、mail 16、analytics 16、helpdesk 12、user 6、bot 1。

## 2. 关键技术事实（brainstorming 核实）

- **1578 条全在 `//!` doc comment**（非 `///`），分两类标签：`docPath:`（codegen 注入 1515）+ `文档:`（手写 77）。
- **codegen 发射点唯一**：`tools/api_contracts/codegen_render.py:73` `_module_doc()` 的 `f"//! docPath: {ir.doc_path}\n"`。该函数只发射 `docPath:` 一行 URL，不发射 `文档:`。另有 `tools/restructure_hr.py:46`（一次性 HR 重构脚本）同形态。
- **rustdoc lint 种类**：cargo doc 全量 warning 中 **只有 `rustdoc::bare_urls` 一种**，无 `broken_intra_doc_links` / `private_doc_tests` 等 → 移除 CI `-A rustdoc::bare_urls` 后 `-D warnings` 只会因 bare_urls 失败（将被清零），**解压零意外**。
- **codegen 闭环**：`tools/codegen.py:185` 用 `-A missing_docs`（不在本范围，A1）；codegen 当前产物未大规模 `--write` 落盘，故 codegen 改造是「面向未来」预防。

## 3. 实现方案（五位一体，D1-D4 不可分割）

### 改动 1（D2）：codegen 渲染器发射 `<URL>`

`tools/api_contracts/codegen_render.py:73`：

```python
# before
f"//! docPath: {ir.doc_path}\n"
# after
f"//! docPath: <{ir.doc_path}>\n"
```

顺带 `tools/restructure_hr.py:46`（`//! docPath: {doc_path}` → `//! docPath: <{doc_path}>`，一次性脚本，一致性）。**若不改 codegen，workspace deny 后任何 codegen --write 会立刻 CI 红**——故 codegen 改造是 deny 锁死的前置必要条件。

### 改动 2（D1）：批量修复现存 1578 裸 URL

Python 脚本（放 `tools/`，一次性但保留供复发时复用），遍历 `crates/**/*.rs`，对 `//!` / `///` doc comment 行用 regex 包裹裸 URL：

```python
import re, pathlib
URL = re.compile(r'(?<![<(])(https?://[^\s<>)]+)')
for f in pathlib.Path('crates').rglob('*.rs'):
    lines = f.read_text(encoding='utf-8').splitlines(keepends=True)
    changed = False
    for i, line in enumerate(lines):
        s = line.lstrip()
        if s.startswith('//!') or s.startswith('///'):
            new = URL.sub(r'<\1>', line)
            if new != line:
                lines[i] = new; changed = True
    if changed:
        f.write_text(''.join(lines), encoding='utf-8')
```

设计要点：
- `(?<![<(])` lookbehind：跳过已包裹的 `<URL` 和 markdown 链接 `(url)`，**防重复包裹、防破坏 markdown**。
- `[^\s<>)]+` 匹配 URL body：遇空格/`<`/`>`/`)` 结束，避免贪心吞掉行尾标点。
- 仅 doc comment（`//!`/`///`）：bare_urls 是 rustdoc lint，普通 `//` 不触发；限范围降低误伤。
- 闭环：修完 `cargo doc --workspace --all-features 2>&1 | grep -c bare_urls` 必须 = **0**。

### 改动 3（D3）：workspace `[workspace.lints.rustdoc] bare_urls = "deny"`

根 `Cargo.toml` 新增 `[workspace.lints.rustdoc]` 段（当前不存在），声明 `bare_urls = "deny"`。20 业务 crate + 根包已 `[lints] workspace = true`，自动继承。选 `deny`（非 warn）以锁死防复发——前提是 D1 已清零存量、D2 已防新增。

### 改动 4（D4）：CI doc job 移除 `-A rustdoc::bare_urls`

`.github/workflows/ci.yml:44` 的 `RUSTDOCFLAGS` 去掉 `-A rustdoc::bare_urls`，让 deny 生效。移除后 CI doc job 会因任何残留 bare_urls 失败——由 D1 的零警告保证通过。`-D warnings` 保留（继续压制其他 rustdoc warning，实测当前只有 bare_urls 一种）。

### 改动 5（D5）：capability `rustdoc-bare-urls`

对齐 `workspace-dependency-policy` / `feature-naming-convention` 范式，open 阶段已起草 `specs/rustdoc-bare-urls/spec.md`（3 Requirements）。**design 阶段无需 Spec Patch**——3 Requirements（URL MUST `<...>` / workspace deny + CI 不压制 / codegen MUST 发射 `<URL>`）与五位一体设计完全一致。

## 4. 决策

- **D1 脚本批量修 + cargo doc 闭环**：Python regex 限 doc comment，lookbehind 防误伤。
- **D2 codegen 同步改造（防复发必需）**：deny 锁死的前置条件。
- **D3 workspace deny（非 warn）**：锁死，靠 D1 清零 + D2 防新增兜底。
- **D4 CI 解压**：让 deny 在 CI 生效（已确认无其他 rustdoc lint 暴露）。
- **D5 capability 落盘**：判定准则 + codegen 发射规范，供 code-review/design-review 引用。

## 5. 范围外（诚实限制）

- **missing_docs 策略统一**（A1）：源码 `#![]` outlier（security deny / analytics allow / client 注释）、`just lint -A missing_docs` 与 CI 不一致、1057 行占位 doc——全部另开 change。
- **codegen `-A missing_docs` 闭环**（`tools/codegen.py:185`）：A1/另案。
- **doc comment 文本内容**：本 change 仅包裹 URL，不改文案。

## 6. 测试策略

| 验证项 | 命令 | 预期 |
|--------|------|------|
| **核心断言** | `cargo doc --workspace --all-features 2>&1 \| grep -c bare_urls` | **0**（迁移前 1578） |
| deny 不触发新错 | 加 `[workspace.lints.rustdoc] bare_urls=deny` 后 `cargo doc --workspace --all-features` | 0 warning，exit 0 |
| CI 模拟 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features`（移除 -A 后） | exit 0 |
| codegen 抽样 | 渲染含 URL 的模块 doc | 输出 `//! docPath: <https://...>` |
| 格式 | `cargo fmt --check` | exit 0 |
| lint 双模式 | `just lint` + `clippy --no-default-features`（CI 同款） | 双过 |
| 构建 | `cargo build --workspace --all-features` | Finished |
| MSRV | `cargo +1.88 check --locked`（pinned msrv lockfile） | Finished |

## 7. 风险与取舍

- **[脚本误伤非 URL 文本]** → 限 doc comment + lookbehind + `cargo doc` 闭环零警告 + `git diff` 抽样双保险。
- **[deny 后 codegen --write 破坏 CI]** → D2 同步改 codegen；验证阶段抽样渲染确认 `<URL>`。
- **[77 处手写 `文档:` 边界]** → 脚本 regex 不区分标签，统一包裹 `https?://`；修完 cargo doc 闭环验证。
- **[范围克制]**：仅 bare_urls（A2）；A1 全另案，不混入。
