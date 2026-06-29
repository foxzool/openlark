# Brainstorm Summary

- Change: remove-deprecated-wiki-params
- Date: 2026-06-28

## 确认的技术方案

**方案 A（已确认）**：逐处把 4 个 deprecated wiki Params（SearchWikiParams/ListWikiSpacesParams/CreateWikiSpaceParams/MoveDocsToWikiParams）的 ~6 处用法迁移到对应 `XxxRequest` Builder，然后删除 4 个 Params struct。逐处核对字段映射（Params 字段 → Builder setter）。

## 关键取舍与风险

- **取舍**：迁移到 Builder（deprecation note 指引）vs 保留。选迁移（v1.0 breaking 窗口）。
- **风险**：字段映射需逐处核对（Params 字段名/类型 vs Builder setter）；缓解：build 阶段 clippy 验证 + 用法少（~6）。

## 测试策略

- 三组 feature clippy（default/all-features/no-default）`-D warnings` exit 0。
- `cargo test --workspace` 通过。
- 4 个 Params struct grep = 0。

## Spec Patch

无。delta spec `no-deprecated-wiki-params` 与方案一致。
