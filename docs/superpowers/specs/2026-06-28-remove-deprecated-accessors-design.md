---
comet_change: remove-deprecated-accessors
role: technical-design
canonical_spec: openspec
---

# Design — remove-deprecated-accessors

> OpenSpec delta spec `no-deprecated-compat-accessors` 是事实源（canonical）。本文档为技术设计。

## 背景与根因

10 个 `#[deprecated]` 兼容访问器方法 clutter API 表面，v1.0 breaking 窗口应移除：

- **HR（8）**：`Hr::attendance()/corehr()/compensation()/payroll()/performance()/okr()/hire()/ehr()`，v0.15.0 起 deprecated（「用字段直接访问 `client.attendance`」）。方法式访问与字段访问并存。
- **analytics（2）**：`SearchV2::query()/user()`，未接线 runtime stub（「not wired to endpoint, prefer implemented surfaces」）。

deprecation 决策早已做出（since 0.15.0 / 0.5.0），本 change **执行移除**，无新设计空间。10 个方法**零内部调用**（实证）→ 干净移除。

来源：架构审计 #268（20 个 deprecated 拆分；本 change 处理 A+E 共 10 个最干净的，B/C/D/F/G 10 个异构项另开）。

## 决策

**D1（移除方式，已确认）**：直接删除 10 个 `#[deprecated] pub fn`。无替代方案（deprecation note 早有替代指引）。

**D2（QueryApi/UserSearchApi 类型去留，已确认）—— 保留**：移除 `SearchV2::query()`/`user()` **访问器方法**，但 `v2/query.rs`、`v2/user.rs` 模块与 `QueryApi`/`UserSearchApi` 类型保留 `pub`（完整路径可达）。仅移除便捷存根方法，避免连带删除有补全计划（#276）的类型。

## 改动清单

| 文件 | 动作 |
|------|------|
| `crates/openlark-hr/src/lib.rs` | 删除 8 个 `#[deprecated] pub fn`（含 `#[cfg(feature)]` 门控 + 文档注释） |
| `crates/openlark-analytics/src/search/search/v2.rs` | 删除 2 个 `#[deprecated] pub fn`（query/user） |
| `CHANGELOG.md` | `[Unreleased] > Breaking Changes` + 迁移映射表 |

不动：字段定义、`QueryApi`/`UserSearchApi` 类型与模块、B/C/D/F/G（另开）、examples（已确认不引用）。

## 迁移映射（CHANGELOG）

| 旧（移除） | 新 |
|------------|-----|
| `hr.attendance()` / `corehr()` / ... | `hr.attendance` / `hr.corehr` / ...（字段） |
| `search_v2.query()` | 用 `doc_wiki`/`schema`/`app`/`message` surface |
| `search_v2.user()` | 无 surface（user-search 未实现）；`UserSearchApi` 经完整路径可达 |

## 风险与缓解

- **[Breaking]** 公开方法移除 → 外部编译失败；缓解：CHANGELOG 迁移 + 字段访问 drop-in。
- **[analytics user() 无替代]** 保留 `UserSearchApi` 类型（#275/#276 补全）。

## 测试策略

1. `cargo clippy --workspace --all-targets` 三组 feature（default / all-features / no-default）`-D warnings` exit 0。
2. `cargo test --workspace` 通过。
3. HR/analytics grep 目标 deprecated = 0（B/C/D/F/G 不在范围）。
4. examples 不引用已移除方法。

## 迁移与回滚

纯公开方法删除 + CHANGELOG，无数据迁移；`git revert` 即可。

## 关联 issue

- #268（deprecated 清理，本 change 处理 A+E；B/C/D/F/G 待 follow-up）
- #275 / #276（analytics search 补全）
