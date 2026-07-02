## Why

`openlark-application` crate 有 **578 行 `/// 待补充文档。` 占位 doc**（91 文件，占全仓占位 55%）——legacy codegen 产物（`///` 在 `#[derive]` 后 + 占位文案）。撑着 0 missing_docs 但无文档价值。本 change 清理 application 占位，回补真 doc。承接 #273 #1/#4/#2。

## What Changes
- 替换 application 全部 578 行 `/// 待补充文档。` 占位为有意义 doc（recipe 仿 #1：`<文件//! 标题>+<item 角色>`）。
- 修正 doc 位置到 `#[derive]` 前。
- delta：ADDED `missing-docs-quality` 的 application crate 场景（capability 由 change `cleanup-docs-placeholder-docs` 先建）。

非破坏性：仅 doc 文本 + 位置。

## Capabilities
### New Capabilities
（无——复用 `missing-docs-quality`，本 change 加 application 场景，依赖 `cleanup-docs-placeholder-docs` 先归档）
### Modified Capabilities
（本 change ADDED 新 requirement 到 `missing-docs-quality`：application crate 无占位）

## Impact
- 代码：`crates/openlark-application/src/**` 91 文件（578 占位 → 真 doc + 位置修正）。
- 无 API/依赖变更。
