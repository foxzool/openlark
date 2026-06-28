# Brainstorm Summary

- Change: remove-deprecated-accessors
- Date: 2026-06-28

## 确认的技术方案

**方案 A（已用户确认）**：直接删除 10 个 `#[deprecated] pub fn` —— HR 8 个 service 访问器（`attendance()/corehr()/compensation()/payroll()/performance()/okr()/hire()/ehr()`，用户改字段访问）+ analytics 2 个未接线存根（`SearchV2::query()/user()`）。零内部调用，干净移除。

**D2**：analytics 移除存根**访问器方法**，但**保留** `QueryApi`/`UserSearchApi` 类型与 `v2/query.rs`/`v2/user.rs` 模块（避免连带删除有补全计划 #276 的类型）。

## 关键取舍与风险

- **取舍**：方案 A（移除 10）vs B（保留到 v1.x）vs C（只删 HR）。选 A——v1.0 是 breaking 窗口，analytics 存根也是 0 调用死代码。
- **风险①[Breaking]**：移除公开方法，外部 `.attendance()` 编译失败 → 缓解：CHANGELOG 迁移指引；字段访问 drop-in（`.attendance()` → `.attendance`）。
- **风险②[analytics user() 无替代]**：user-search 未实现 → 缓解：保留 `UserSearchApi` 类型（完整路径可达），#275/#276 跟踪补全。

## 测试策略

- 三组 feature clippy（default / --all-features / --no-default-features）`-D warnings` 全 exit 0。
- `cargo test --workspace` 通过。
- examples 不引用已移除方法（`.attendance()`/`.query()` 等 = 0）。
- HR/analytics grep 目标 deprecated = 0（剩余 B/C/D/F/G 不在本范围）。

## Spec Patch

无。delta spec `no-deprecated-compat-accessors` 与方案 A 完全一致。
