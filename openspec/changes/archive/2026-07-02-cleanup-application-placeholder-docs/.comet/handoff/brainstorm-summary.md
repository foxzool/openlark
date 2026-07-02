# Brainstorm Summary

- Change: cleanup-application-placeholder-docs
- Date: 2026-07-02

## 确认的技术方案

替换 `openlark-application` crate 578 行 `/// 待补充文档。` 占位为有意义 doc（#1 analytics 机械 recipe），+ 修正 190 个 struct 占位位置（`#[derive]` 后→前）。subagent-driven 按**版本×子域**分组执行（~7-8 组）。

### Recipe（`<//! 标题>+<item 角色>`）

| item | doc 文案 |
|------|---------|
| Request struct | `<API 中文名>的请求。` |
| Response struct | `<API 中文名>的响应。` |
| field `data` | `响应数据。` |
| field（named） | `<字段中文名>。`（读字段名翻译，17 个） |
| `fn new` | `创建请求实例。` |
| `fn execute` | `执行<API>请求。` |
| `fn execute_with_options` | `带自定义请求选项执行。` |
| module（`pub mod`） | `<子模块 API 说明>。` |

API 中文名取自文件 `//!` 头（如 `//! app create`→"创建应用"）。

### 探勘事实（grounded）

- 578 占位全是 `待补充文档。`，91 文件全在 `src/application/application/v{1,5,6}/<sub-domain>/`
- item 类型：**fn 269 / struct 190 / field 116 / module 3**（0 enum、0 variant）
- 版本：v1=213 / v5=12 / v6=352 / root=1
- field：87 `data`（机械）+ 29 named（17 字段名）
- 位置修正机械性：190 struct 占位**全部**紧跟 `#[derive(...)]` 后，0 多属性叠加 → 统一 3 行交换；388 fn/field/module 原位换文案

## 关键取舍与风险

- **取舍**：纯机械 recipe（非 docs 那种逐项语义）——因占位全在 Request/Response/new/execute，模式同构。
- **风险 [规模 578 诱发偷懒]** → recipe 强制引用真实 API 名（来自 `//!` 头）+ 占位符 grep 守门 + pilot 先行。
- **风险 [doc 位置漏修]** → 位置守门 grep（`#[derive]` 后不紧跟 `///`）。
- **取舍 [分组 B]**：版本×子域，边界清晰、覆盖校验简单，与 design.md D2 一致。弃 A（子域家族跨版本）与 C（扁平切）——B 与目录结构对齐最易验证。

## 测试策略

- 无新增测试（纯 doc 改动，不改逻辑）。
- 双守门：占位 grep 空 + 位置 grep（`#[derive]` 后无 `///`）。
- `cargo doc --workspace --all-features` missing_docs=0；`cargo fmt --check` + `just lint` exit 0；application 现有测试不破。

## Spec Patch

无。delta spec（`specs/missing-docs-quality/spec.md`）已含 2 scenarios（application 无占位 + doc 不在 `#[derive]` 后），主 spec `missing-docs-quality` 由 `cleanup-docs-placeholder-docs` 归档时已建。本 change 仅 ADDED application crate 场景到既有 capability。
