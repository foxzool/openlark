# Brainstorm Summary

- Change: enforce-bare-urls
- Date: 2026-07-01

## 确认的技术方案（待用户确认）

**清零 1578 bare_urls + codegen 改造 + workspace deny + CI 解压，五位一体根治。**

改动（原子分组）：
1. **codegen 改造**：`tools/api_contracts/codegen_render.py:73` `_module_doc` 的 `//! docPath: {ir.doc_path}` → `//! docPath: <{ir.doc_path}>`；顺带 `tools/restructure_hr.py:46`（一次性脚本，一致性）
2. **批量修复**：Python 脚本把 `//!`/`///` doc comment 行内的裸 `https?://` 包成 `<...>`，regex `(?<![<(])(https?://[^\s<>)]+)` → `<\1>`（lookbehind 防重复包裹、防破坏 markdown `[t](url)`）
3. **workspace lint**：根 `Cargo.toml` 新增 `[workspace.lints.rustdoc] bare_urls = "deny"`
4. **CI 解压**：`.github/workflows/ci.yml:44` doc job `RUSTDOCFLAGS` 移除 `-A rustdoc::bare_urls`
5. **capability**：`rustdoc-bare-urls` spec（open 阶段已起草 3 Requirements）

## 关键技术事实（brainstorm 核实）

- **1578 条 bare_urls** 全在 `//!` doc comment：94.4%（1515）`docPath:` codegen 注入，4.8%（77）`文档:` 手写历史遗留；全仓已 `<URL>` 规范化的 = 0（无重复包裹风险）
- **codegen 发射点唯一**：`codegen_render.py:73` `_module_doc`（只发射 `docPath:` 一行 URL，不发射 `文档:`）；`restructure_hr.py:46` 是一次性 HR 重构脚本
- **rustdoc lint 种类**：cargo doc 全量 warning 中 **只有 `rustdoc::bare_urls` 一种**，无 broken_intra_doc_links / private_doc_tests 等 → 移除 CI `-A rustdoc::bare_urls` 后 `-D warnings` 只会因 bare_urls 失败（将被清零），安全
- **codegen 闭环**：`tools/codegen.py:185` 用 `-A missing_docs`（不在本范围，A1）；codegen 当前产物未大规模 --write 落盘（风格 A 6 PR 完成但未 --write），故 codegen 改造是「面向未来」预防

## 关键取舍与风险

- **五位一体必须同时落**：只清零不改 codegen → 未来 codegen --write 立刻 CI 红；只清零不加 deny → 无防复发；故 D1-D4 不可分割
- **脚本误伤风险低**：限 doc comment + lookbehind + `cargo doc` 闭环零警告 + git diff 抽样双保险
- **CI 解压零意外**：已确认无其他 rustdoc lint 被暴露
- **范围克制**：仅 bare_urls（A2）；missing_docs/源码 `#![]` outlier/just lint 不一致/1057 占位 doc 全部 A1 另案

## 测试策略

- `cargo doc --workspace --all-features 2>&1 | grep -c bare_urls` = **0**（核心断言）
- 加 `[workspace.lints.rustdoc] bare_urls=deny` 后 `cargo doc` 仍 0 warning（deny 不触发新错）
- 移除 CI `-A rustdoc::bare_urls` 后本地模拟 `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features` exit 0
- codegen 渲染抽样：`_module_doc` 输出含 `<URL>`
- `cargo fmt --check` / `just lint` 双模式 / `cargo build --all-features` / msrv `--locked` 无回归

## Spec Patch（open 阶段已起草，design 无需改）

`specs/rustdoc-bare-urls/spec.md` 的 3 Requirements（URL MUST `<...>` / workspace deny + CI 不压制 / codegen MUST 发射 `<URL>`）已与设计完全一致，无需 Spec Patch。

## 开放问题决议

- Q1（codegen 发射点）= `codegen_render.py:73` 单一 + `restructure_hr.py:46` 顺带
- Q2（脚本实现）= Python regex + lookbehind，限 doc comment
- Q3（其他 rustdoc lint）= 无，CI 解压安全
