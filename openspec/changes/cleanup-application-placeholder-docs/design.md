## Context
application crate 578 行 `待补充文档` 占位（91 文件），legacy codegen 产物。占位项 = `pub fn new`/`execute`/`execute_with_options`/struct/field（同 #1 analytics 同构）。承接 #1/#4/#2。

## Goals / Non-Goals
**Goals**：替换 578 占位为真 doc；修正 doc 位置；占位符 grep 守门。
**Non-Goals**：不改逻辑；不动其它 crate；不动作 TestCheck。

## Decisions
- **D1 recipe**：仿 #1，`<//! 标题>+<item 角色>`。
- **D2 执行**：578 项/91 文件，subagent-driven 按 version（v1/v5/v6/v7）+ sub-domain 分组（application 最大，分组并行）；占位符 + 位置双守门。
- **D3 验证**：逐组 `cargo doc -p openlark-application` 该组文件无 warning；全 crate 0 占位。

## Risks
- [规模 578 项诱发偷懒] → recipe 引用真实 API 名 + 占位符 grep 守门 + pilot 先行（仿 #1）。
- [doc 位置漏修] → 位置守门 grep。

## Migration
纯 doc。回滚 = revert。顺序：pilot → 按 version/domain 组回补 → 双守门 → 全局验证。
