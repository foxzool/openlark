## Context

docs crate 有 144 行 `/// 公开项说明。` 占位（14 文件），legacy codegen 产物（`///` 在 `#[derive]` 后 + 占位文案）。承接 #273 #1（recipe）+ #4（codegen 已修）+ #2（测试）。

## Goals / Non-Goals
**Goals**：替换 docs 144 占位为真 doc；修正 doc 位置到 `#[derive]` 前；建立 `missing-docs-quality` capability。
**Non-Goals**：不改逻辑/签名；不动其它 crate（application/small-crates 各自 change）；不动作 TestCheck。

## Decisions
- **D1 recipe**：仿 #1，`<文件//! 标题>+<item 角色>`（struct/field/fn 等）。逐文件读 `//!` 标题套 recipe。
- **D2 doc 位置**：`///` 移到 `#[derive]` 前（标准 + 对齐 #1/communication 规范）。
- **D3 执行**：14 文件、144 项，subagent-driven 按域分组（docs crate 内的 sub-domain）；占位符 grep 守门。

## Risks
- [doc 准确性] → 派生自文件级 `//!` + item 角色；占位符 grep + review 守门。
- [漏改位置] → 自验 `grep -A1` 确认无 `#[derive]` 后紧跟 `///`。

## Migration
纯 doc 改动。回滚 = revert。顺序：逐文件回补 → grep 守门 → cargo doc 0 警告。
