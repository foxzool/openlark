## Context
5 个中小 crate 共 335 行 `待补充文档` 占位（mail 104/workflow 78/meeting 65/user 47/hr 41），legacy codegen 产物。占位项同 #1 analytics 同构。承接 #1/#4/#2。

## Goals / Non-Goals
**Goals**：替换 335 占位为真 doc；修正 doc 位置；占位符 grep 守门。
**Non-Goals**：不改逻辑；不动 application/docs（各自 change）；不动作 TestCheck。

## Decisions
- **D1 recipe**：仿 #1，`<//! 标题>+<item 角色>`。
- **D2 执行**：按 crate 分 5 组 subagent-driven（每 crate 独立单元）；占位符 + 位置双守门。mail 最大（104）先行。
- **D3 验证**：逐 crate `cargo doc -p <crate>` 无 warning；5 crate 0 占位。

## Risks
- [跨 5 crate 一致性] → 同 recipe + 双守门；per-crate 自验。
- [doc 位置漏修] → 位置守门 grep。

## Migration
纯 doc。回滚 = revert。顺序：按 crate（mail→workflow→meeting→user→hr）回补 → 双守门 → 全局验证。
