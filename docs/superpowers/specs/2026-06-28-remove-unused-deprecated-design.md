---
comet_change: remove-unused-deprecated
role: technical-design
canonical_spec: openspec
archived-with: 2026-06-28-remove-unused-deprecated
status: final
---

# Design — remove-unused-deprecated

> OpenSpec delta spec `no-unused-deprecated` 是事实源（canonical）。本文档为技术设计。

## 背景与根因

5 个 `#[deprecated]` 项零调用/dead，v1.0 breaking 窗口移除。deprecation 决策已做出（since 0.5.0–0.16.0），本 change **执行移除**，无新设计空间。

- **G. auth（3）**：`TenantAccessTokenBuilder::app_id()/app_secret()/app_ticket()`（`auth/auth/v3/auth/tenant_access_token.rs:85,92,99`），0 调用（builder 被用但这 3 方法无人调）。→ `app_access_token()`+`tenant_key()`。
- **D. docs（1）**：`RecordFieldValue::to_value()`（`base/bitable/v1/field_types.rs:248`），0 调用。→ 用 `RecordFieldValue` 类型。
- **C. docs（1）**：`impl_required_builder!` 宏的 `new()`（`common/request_builder.rs:97`），dead（`#[expect(dead_code)]`），唯一调用者用 `builder()`。→ `builder()`。

来源：#278（剩余 10 deprecated 的 G+D+C 子集）。B（wiki Params ~16 用法）+ F（im 别名 47 文件）需迁移，留 #278。

## 决策

**D1（移除方式，已确认）**：直接删除 5 个 `#[deprecated] pub fn`（C 为宏内 `new()` 生成块）。无替代方案。

**D2（C 宏 new() 移除，已确认）**：从 `impl_required_builder!` 宏删除 `new()` 生成块（含 `#[deprecated]`+`#[expect(dead_code)]`）。唯一调用者 TestRequest 用 `builder()` → 安全。

## 改动清单

| 文件 | 动作 |
|------|------|
| `crates/openlark-auth/src/auth/auth/v3/auth/tenant_access_token.rs` | 删除 app_id/app_secret/app_ticket 3 个 `#[deprecated] pub fn` |
| `crates/openlark-docs/src/base/bitable/v1/field_types.rs` | 删除 `to_value()` |
| `crates/openlark-docs/src/common/request_builder.rs` | 从 `impl_required_builder!` 宏删除 `new()` 生成块 |
| `CHANGELOG.md` | `[Unreleased] > Breaking Changes` + 迁移映射表 |

不动：B（wiki Params）/ F（im 别名）；Builder 实现；examples（已确认不引用）。

## 迁移映射（CHANGELOG）

| 旧（移除） | 新 |
|------------|-----|
| `tenant_access_token().app_id(x)/.app_secret(x)/.app_ticket(x)` | `app_access_token(...)` + `tenant_key(...)` |
| `record_field_value.to_value()` | 直接用 `RecordFieldValue`（已 serde） |
| 宏 `Builder::new()` | `Builder::builder()` |

## 风险与缓解

- **[Breaking]** 公开方法移除 → 编译失败；缓解：CHANGELOG 迁移；5 项 0 调用，影响极小。
- **[C 宏]** 移除宏 new() 影响 1 调用者（不用 new()）→ 安全。

## 测试策略

1. 三组 feature clippy（default/all-features/no-default）`-D warnings` exit 0。
2. `cargo test --workspace` 通过。
3. examples/tests 不引用已移除项。
4. auth/docs 目标 deprecated 已删。

## 迁移与回滚

纯公开方法删除 + CHANGELOG；`git revert` 即可。

## 关联 issue

- #278（G+D+C 本 change 处理；B+F 留 #278）

## Implementation Divergence（实现偏差，2026-06-28 verify 记录）

**G（auth app_id/secret/ticket）未删除**——build 核实发现这 3 个 deprecated 方法是 **functional legacy two-step flow**（`execute_with_options` 读取 `legacy_app_id/secret/ticket` 字段做两步换 token，`test_execute_legacy_chain` 验证），**非 unused**。删除需连带移除 legacy flow + 测试，是更大的独立改动。

- **偏差**：proposal/design/spec 原述 G+D+C(5)，实际只做 **D+C(2)**。
- **处理**：G 还原保留；delta spec req1 已修正为「auth legacy 方法保留（functional）」；移除 legacy flow 的工作留 #278。
- D（to_value）+ C（宏 new）确实是零调用/dead，按计划删除（连带空 impl 移除 + `json!` import 改 `#[cfg(test)]`）。

