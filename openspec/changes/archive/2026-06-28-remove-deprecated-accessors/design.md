## Context

10 个 `#[deprecated]` 兼容访问器待移除（v1.0 breaking 窗口）：HR 8 个 service 访问器（v0.15.0 起 deprecated，字段访问替代）+ analytics 2 个未接线存根。零内部调用，移除干净。

deprecated 决策早已做出（since 0.15.0 / 0.5.0），本 change 只是**执行移除**，无新设计空间。关键点是迁移指引与 breaking 标注。

## Goals / Non-Goals

**Goals:**
- 移除 HR 8 + analytics 2 共 10 个 deprecated 访问器方法。
- CHANGELOG 提供清晰迁移指引（方法 → 字段 / 其他 surface）。

**Non-Goals:**
- 不动 B/C/D/F/G（docs wiki 旧构造/macro new/to_value/im 别名/auth builder）——异构，另开 issue。
- 不移除 `QueryApi`/`UserSearchApi` 类型与模块（仅移除便捷存根访问器）。
- 不改字段访问实现、不新增 API。

## Decisions

**D1（移除方式）—— 已确定**：直接删除 10 个 `#[deprecated] pub fn`。无替代方案（deprecation 早有 note 指引替代）。

**D2（QueryApi/UserSearchApi 模块去留）—— 保留**：移除 `SearchV2::query()`/`user()` 存根访问器，但 `v2/query.rs`、`v2/user.rs` 模块与 `QueryApi`/`UserSearchApi` 类型保留为 `pub`（仍可经完整路径访问；其补全见 #276）。仅移除便捷存根方法，避免连带删除尚有补全计划的类型。

## 迁移映射（写入 CHANGELOG）

| 旧（移除） | 新 |
|------------|-----|
| `hr.attendance()` | `hr.attendance`（字段） |
| `hr.corehr()` / `compensation()` / ... | `hr.corehr` / `hr.compensation` / ...（字段） |
| `search_v2.query()` | 用 `doc_wiki`/`schema`/`app`/`message` surface，或 `SearchV2` 不再暴露 query 存根 |
| `search_v2.user()` | 同上（user-search 未实现，无替代 surface） |

## Risks / Trade-offs

- **[Breaking]** 移除公开方法 → 用户代码 `client.attendance()` 编译失败。缓解：CHANGELOG 迁移指引 + 字段访问是 drop-in 替代（`.attendance()` → `.attendance`，1 字符之差）。
- **[analytics user() 无替代]** user-search 未实现，移除 `user()` 后无 surface。缓解：保留 `UserSearchApi` 类型（完整路径可达），#276/#275 跟踪补全。

## Migration Plan

1. 删除 HR 8 + analytics 2 共 10 个 `#[deprecated] pub fn`。
2. `cargo clippy` 三组 feature + `cargo test --workspace`。
3. CHANGELOG `[Unreleased] > Breaking Changes` + 迁移表。
4. 回滚：git revert。

## Open Questions

- 无（移除是 pre-decided；仅执行）。
