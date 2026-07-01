## Context

OpenLark workspace 的 `rustdoc::bare_urls` lint 当前处于「CI 压制 + workspace 未治理」状态：

| 层 | 现状 |
|----|------|
| `[workspace.lints]`（根 Cargo.toml:62-71） | 有 `[workspace.lints.rust]` / `[.clippy]`，**无 `[.rustdoc]` 段**——bare_urls 未治理 |
| CI doc job（ci.yml:40-57） | `RUSTDOCFLAGS="-D warnings -A rustdoc::bare_urls"`——显式 allow 压制 |
| 实测 warning | **1578 条**（cargo doc --workspace --all-features），16 crate 分布，被 CI 静默放过 |
| 形态 | 94.4% codegen 注入的 `//! docPath: <裸URL>`；4.8% 手写 `//! 文档: <裸URL>`；全仓 `<URL>` 规范化的 URL = 0 |

约束：codegen 渲染器（`tools/api_contracts/codegen_render.py`）的 `_module_doc()` 等函数负责发射 `//! docPath:` 行；`tools/codegen.py` 闭环用 `-A missing_docs`（不在本范围）；20 crate 全 `[lints] workspace = true` 继承。

issue #273 Part A 现象 B 要求清零 bare_urls 并 workspace 级统一（deny 锁死）。Part A1（missing_docs/一致性治理）另开 change。

## Goals / Non-Goals

**Goals:**

- 全 workspace bare_urls warning **1578 → 0**（`cargo doc --workspace --all-features` 实测零警告）
- `[workspace.lints.rustdoc] bare_urls = "deny"` 落盘，全 crate 继承
- CI doc job 移除 `-A rustdoc::bare_urls` 压制，真正执行 deny
- codegen 渲染器发射 `<URL>`，未来 `--write` 不再生裸 URL
- 落盘 `rustdoc-bare-urls` capability spec（判定准则 + codegen 发射规范）

**Non-Goals:**

- 不动 missing_docs 策略（A1，另开 change）
- 不动源码级 `#![...]` outlier（security deny / analytics allow / client 注释，A1）
- 不修 `just lint -A missing_docs` 与 CI 的不一致（A1）
- 不动 codegen 的 `-A missing_docs` 闭环（`tools/codegen.py:185`，A1/另案）
- 不清理 1057 行占位 doc（独立债务）
- 不改 doc comment 的文本内容（仅包裹 URL）

## Decisions

### D1: 批量修复用脚本 + `cargo doc` 闭环验证

1578 处裸 URL 形态极统一（`//! <label>: <bare-url>`），用脚本（sed 或小 Python）把行内 `https://...` 包成 `<https://...>`。脚本须处理：行尾 URL、同一 `//!` 行多 URL、`docPath:` / `文档:` 两类标签。修完跑 `cargo doc --workspace --all-features 2>&1 | grep -c bare_urls` 必须 = 0 闭环验证。具体脚本实现（sed vs Python）+ 边界抽样在 design 阶段定。

### D2: codegen 渲染器同步改造（防复发必需）

定位 `tools/api_contracts/codegen_render.py` 中发射 `docPath:` 的函数（调研指向 `_module_doc()` :69-76），把裸 URL 改为 `<URL>`。这样未来 codegen `--write` 生成的新 API doc 不含裸 URL。**若不改 codegen，workspace deny 后任何 codegen --write 会立刻 CI 红**——故 codegen 改造是 deny 锁死的前置必要条件。

### D3: workspace `[workspace.lints.rustdoc] bare_urls = "deny"`

根 Cargo.toml 新增 `[workspace.lints.rustdoc]` 段（当前不存在），声明 `bare_urls = "deny"`。全 crate 已 `[lints] workspace = true`，自动继承。选 `deny`（而非 warn）以锁死、防复发——前提是 D1 已清零存量、D2 已防新增。

### D4: CI doc job 移除 `-A rustdoc::bare_urls`

`ci.yml:44` 的 `RUSTDOCFLAGS` 去掉 `-A rustdoc::bare_urls`，让 deny 生效。移除后 CI doc job 会因任何残留 bare_urls 失败——由 D1 的零警告保证通过。

### D5: 落盘 `rustdoc-bare-urls` capability

对齐 `workspace-dependency-policy` / `feature-naming-convention` 范式，新建 capability spec 记录「doc comment URL MUST `<...>` 包裹 + workspace deny 强制 + CI 不压制 + codegen 发射规范」，供 code-review/design-review 与 codegen 引用防复发。

## Risks / Trade-offs

- **[脚本误伤非 URL 文本]** → Mitigation：脚本只匹配 `//!` 行内的 `https://` / `http://` token 并包裹；修完 `git diff` 抽样人工核对 + `cargo doc` 闭环零警告双保险。
- **[77 处手写 `文档:` URL 边界]** → Mitigation：design 阶段抽样核实是否有非 URL 尾随文本；脚本对这类行单独处理。
- **[deny 后 codegen --write 破坏 CI]** → Mitigation：D2 同步改 codegen 渲染器；验证阶段跑一次 codegen 渲染（若可行）确认输出含 `<URL>`。
- **[codegen 当前未 --write 落盘]** → 现状 codegen 产物未大规模入库（风格 A 6 PR 完成但未 --write），故 D2 是「面向未来」的预防性改造，不阻塞当前清零。

## Open Questions

- **Q1（codegen 发射点精确位置）**：`codegen_render.py` 中发射 `docPath: <URL>` 的具体函数与行（调研指向 `_module_doc:69-76`，design 阶段读源确认）。是否还有 `_field_lines` / 其他函数也发射 URL？
- **Q2（脚本实现：sed vs Python）**：1578 处用单行 sed 还是写小 Python 脚本（更易处理多 URL/边界）？design 阶段看样本后定。
- **Q3（CI doc job 移除后是否需要 rustdoc 其他 lint 配置）**：移除 `-A rustdoc::bare_urls` 后，`-D warnings` 会不会暴露其他 rustdoc lint（如 broken_intra_doc_links）？design 阶段实测 `cargo doc` 全量 warning 种类确认。
