## Why

5 个中小 crate 有 **335 行 `/// 待补充文档。` 占位 doc**：mail 104 / workflow 78 / meeting 65 / user 47 / hr 41——legacy codegen 产物（`///` 在 `#[derive]` 后 + 占位文案）。本 change 清理这 5 个 crate 的占位，回补真 doc。承接 #273 #1/#4/#2，是 #3 拆分的最后一批。

## What Changes
- 替换 mail/workflow/meeting/user/hr 全部 335 行 `/// 待补充文档。` 占位为有意义 doc（recipe 仿 #1）。
- 修正 doc 位置到 `#[derive]` 前。
- delta：ADDED `missing-docs-quality` 的 small-crates 场景（capability 由前序 change 先建）。

非破坏性：仅 doc 文本 + 位置。

## Capabilities
### New Capabilities
（无——复用 `missing-docs-quality`，本 change 加 small-crates 场景）
### Modified Capabilities
（本 change ADDED 新 requirement 到 `missing-docs-quality`：mail/workflow/meeting/user/hr 无占位）

## Impact
- 代码：5 个 crate 的 `src/**`（335 占位 → 真 doc + 位置修正）。
- 无 API/依赖变更。
