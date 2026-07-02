## Why

`openlark-docs` crate 有 **144 行 `/// 公开项说明。` 占位 doc**（14 文件），撑着 0 missing_docs 警告但无实际文档价值——这是 #273 missing_docs 深度治理的最后一块（#1/#4/#2 已归档）。占位 doc 是 legacy codegen 产物（`///` 在 `#[derive]` 后 + 占位文案，非当前 codegen 风格，#4 已让 codegen 产真 doc）。本 change 清理 docs crate 的占位，回补真 doc。

## What Changes

- 替换 docs crate 全部 144 行 `/// 公开项说明。` 占位为有意义 doc（recipe 仿 #1：`<文件//! 标题>+<item 角色>`，对齐 workspace 规范）。
- 修正 doc 位置到 `#[derive]` 前（legacy 把 `///` 放在 `#[derive]` 后，非标准）。
- 新 capability `missing-docs-quality`（ADDED：公开项 MUST 有有意义 doc，非占位符 + docs crate 无占位场景）。

**非破坏性**：仅 doc 文本 + 位置，不改逻辑/签名。承接 #273 #1（recipe）+ #4（codegen 不再产占位）+ #2（测试治理）。

## Capabilities

### New Capabilities
- `missing-docs-quality`: 公开项 MUST 有有意义 doc（禁止 `/// 待补充文档。`/`/// 公开项说明。` 等占位）；docs crate 占位已清。

### Modified Capabilities
（无）

## Impact
- 代码：`crates/openlark-docs/src/**` 14 文件（144 占位 → 真 doc + 位置修正）。
- Spec：新建 `openspec/specs/missing-docs-quality/spec.md`。
- 无 API/依赖变更。
