## Context

4 个 deprecated wiki `Params` struct（since 0.16.0）待移除，~6 处用法迁移到 Builder。deprecation 决策已做出，本 change 执行迁移 + 删除。

## Goals / Non-Goals

**Goals:** 迁移 4 个 Params 的 ~6 处用法到 Builder；删除 4 个 deprecated struct。

**Non-Goals:** 不动 F（im 别名）；不动非 deprecated Params；不改 Builder 实现。

## Decisions

**D1（迁移方式）**：每处 `XxxParams { field: value }` 用法 → `XxxRequest::builder().field(value)...`；迁移完删除 Params struct。逐文件处理。

## Risks

- **[Breaking]** 移除公开 struct → 编译失败；缓解：CHANGELOG 迁移；用法少（~6），影响小。
- **[迁移正确性]** Params → Builder 字段映射需逐个核对（字段名/类型可能略异）。build 阶段 clippy 验证。

## Migration Plan

1. 逐文件迁移 ~6 处 Params 用法到 Builder。
2. 删除 4 个 Params struct。
3. 三组 clippy + test。
4. CHANGELOG breaking + 迁移表。
5. 回滚：git revert。

## Open Questions

- 无（deprecation pre-decided；用法少）。
