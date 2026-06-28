# Brainstorm Summary

- Change: remove-unused-deprecated
- Date: 2026-06-28

## 确认的技术方案

**方案 A（已用户确认）**：直接删除 5 个零调用/dead 的 `#[deprecated]` 项 —— G auth `TenantAccessTokenBuilder` 的 `app_id()`/`app_secret()`/`app_ticket()`（→`app_access_token()`+`tenant_key()`）、D docs `RecordFieldValue::to_value()`（→用类型）、C docs `impl_required_builder!` 宏的 `new()`（→`builder()`）。

**D2**：宏 `new()` 移除安全（唯一调用者 TestRequest 用 `builder()`，不用 `new()`）。

## 关键取舍与风险

- **取舍**：方案 A（移除 5）vs B（保留到 v1.x）。选 A——v1.0 是 breaking 窗口，5 项 0 调用。
- **风险①[Breaking]**：移除公开方法，外部编译失败 → 缓解：CHANGELOG 迁移指引；5 项 0 调用，影响极小。
- **风险②[C 宏]**：移除宏 `new()` 影响所有宏调用者——实证仅 1 调用者且不用 → 安全。

## 测试策略

- 三组 feature clippy（default / --all-features / --no-default-features）`-D warnings` exit 0。
- `cargo test --workspace` 通过。
- examples/tests 不引用已移除项（`.to_value()` / `tenant_access_token().app_id` 等 = 0）。
- auth/docs 目标 deprecated 已删。

## Spec Patch

无。delta spec `no-unused-deprecated` 与方案 A 完全一致。
