# Comet Design Handoff

- Change: remove-unused-deprecated
- Phase: design
- Mode: compact
- Context hash: cb01158f5911d0a71ec4a79004c7de2e3ae2428540f5f0f5594f98fb81fbe974

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/remove-unused-deprecated/proposal.md

- Source: openspec/changes/remove-unused-deprecated/proposal.md
- Lines: 1-29
- SHA256: 9a019494970f42d1b1f3c0d044f824619462d1c91a519862061f659a20a5efcd

```md
## Why

5 个 `#[deprecated]` 项**零调用/dead**，v1.0 breaking 窗口应移除（实证核实无任何内部/外部调用）：

- **G. auth（3）**：`TenantAccessTokenBuilder` 的 `app_id()` / `app_secret()` / `app_ticket()`（`auth/auth/v3/auth/tenant_access_token.rs:85,92,99`）。deprecation note 指引改用 `app_access_token()` + `tenant_key()` 流程。**0 调用**（builder 被用，但这 3 个方法无人调）。
- **D. docs（1）**：`RecordFieldValue::to_value()`（`base/bitable/v1/field_types.rs:248`）。note 指引直接用 `RecordFieldValue` 类型。**0 调用**。
- **C. docs（1）**：`impl_required_builder!` 宏生成的 `new()`（`common/request_builder.rs:97`，`#[deprecated] since 0.5.0`，指引用 `builder()`）。**dead**（cleanup-dead-code-allows 已加 `#[expect(dead_code)]`，唯一调用者 TestRequest 测试用 `builder()` 不用 `new()`）。

来源：issue [#278](https://github.com/foxzool/openlark/issues/278)（剩余 10 个 deprecated 的 G+D+C 子集）。**B（4 wiki Params，~16 用法）+ F（im 别名，47 文件）需迁移、非干净删除**，留在 #278 另作（本 change 不动）。

## What Changes

- **BREAKING**：移除 G 的 3 个 auth builder 方法 + D 的 `to_value()` + C 的宏 `new()`。
- 用户迁移：auth 改 `app_access_token()`+`tenant_key()`；`to_value()` 改直接用 `RecordFieldValue`；宏生成的 `new()` 改 `builder()`。

## Capabilities

### New Capabilities
- `no-unused-deprecated`: openlark SHALL 不保留零调用/dead 的 deprecated 公开项；本 change 移除 G（auth 3 方法）+ D（docs to_value）+ C（docs 宏 new）共 5 项。

### Modified Capabilities
<!-- 无现有 spec 需要修改（remove-deprecated-accessors 的 no-deprecated-compat-accessors 针对 HR/analytics 访问器，范围不同） -->

## Impact

- **openlark-auth**：删除 `tenant_access_token.rs` 的 3 个 `#[deprecated] pub fn`（app_id/app_secret/app_ticket）。
- **openlark-docs**：删除 `field_types.rs` 的 `to_value()`；`impl_required_builder!` 宏移除 `new()` 生成（唯一调用者 TestRequest 不用它）。
- **破坏性**：移除公开 deprecated 方法（≥ 一个次版本）。CHANGELOG breaking + 迁移指引。
- **非目标**：不动 B（wiki Params，~16 用法）/ F（im 别名，47 文件）——留在 #278；不改 Builder 实现；不新增 API。
```

## openspec/changes/remove-unused-deprecated/design.md

- Source: openspec/changes/remove-unused-deprecated/design.md
- Lines: 1-45
- SHA256: b502ceb0e9b8e3f798e7a04c8f720f6cbe8f2a38940769c2f13516b897deab4b

```md
## Context

5 个 `#[deprecated]` 项零调用/dead，v1.0 breaking 窗口移除。deprecation 决策已做出（since 0.5.0–0.16.0），本 change 执行移除，无新设计空间。

```
G auth TenantAccessTokenBuilder: app_id()/app_secret()/app_ticket()  → app_access_token()+tenant_key()  [0 调用]
D docs RecordFieldValue::to_value()                                  → 用 RecordFieldValue 类型          [0 调用]
C docs impl_required_builder! 宏 new()                                → builder()                          [dead]
```

## Goals / Non-Goals

**Goals:** 移除 G+D+C 共 5 个零调用/dead 的 deprecated 项；CHANGELOG 迁移指引。

**Non-Goals:** 不动 B（wiki Params ~16 用法）/ F（im 别名 47 文件）——留在 #278；不改 Builder 实现；不新增 API。

## Decisions

**D1（移除方式，已确认）**：直接删除 5 个 `#[deprecated] pub fn`（C 为宏内 `new()` 生成块）。无替代方案（deprecation note 早有指引）。

**D2（C 宏 new() 移除）**：从 `impl_required_builder!` 宏删除 `new()` 生成块（含 `#[deprecated]`+`#[expect(dead_code)]`）。唯一调用者 TestRequest 测试用 `builder()`，不依赖 `new()` → 移除安全。

## 迁移映射（CHANGELOG）

| 旧（移除） | 新 |
|------------|-----|
| `tenant_access_token().app_id(x)` / `.app_secret(x)` / `.app_ticket(x)` | `app_access_token(...)` + `tenant_key(...)` 流程 |
| `record_field_value.to_value()` | 直接用 `RecordFieldValue` 类型（已实现 serde） |
| 宏生成 `Builder::new()` | `Builder::builder()` |

## Risks / Trade-offs

- **[Breaking]** 移除公开方法 → 用户编译失败。缓解：CHANGELOG 迁移指引；这 5 项 0 调用（实证），影响面极小。
- **[C 宏]** 移除宏的 `new()` 影响所有宏调用者——实证仅 1 调用者（TestRequest），且不用 `new()` → 安全。

## Migration Plan

1. 删 G（auth 3 方法）+ D（to_value）+ C（宏 new）。
2. 三组 clippy + test + grep + examples 验证。
3. CHANGELOG breaking + 迁移表。
4. 回滚：git revert。

## Open Questions

- 无（移除是 pre-decided；0 调用已实证）。
```

## openspec/changes/remove-unused-deprecated/tasks.md

- Source: openspec/changes/remove-unused-deprecated/tasks.md
- Lines: 1-27
- SHA256: 3fe2fdf215476fdd5cdb1a69e26d43fe33faa0522e757f8b22add0ffaa270568

```md
# Tasks — remove-unused-deprecated

> 范围：移除 5 个零调用/dead 的 deprecated 项（G auth ×3 + D docs to_value ×1 + C docs 宏 new ×1）。BREAKING。关联 #278（G+D+C 子集）；B（wiki Params）+ F（im 别名）留在 #278。

## 1. 移除 G：auth 3 个 deprecated 方法

- [ ] 1.1 删除 `crates/openlark-auth/src/auth/auth/v3/auth/tenant_access_token.rs` 的 `app_id()`/`app_secret()`/`app_ticket()` 3 个 `#[deprecated] pub fn`（含 deprecation 注释）

## 2. 移除 D：docs to_value

- [ ] 2.1 删除 `crates/openlark-docs/src/base/bitable/v1/field_types.rs` 的 `to_value()` 方法（含 `#[deprecated]`）

## 3. 移除 C：docs 宏 new()

- [ ] 3.1 从 `crates/openlark-docs/src/common/request_builder.rs` 的 `impl_required_builder!` 宏删除 `new()` 生成块（含 `#[deprecated]`+`#[expect(dead_code)]`）；确认唯一调用者 TestRequest 不受影响（用 builder()）

## 4. 验证

- [ ] 4.1 目标 deprecated 已删：auth 3 + docs to_value + 宏 new = 5
- [ ] 4.2 examples/tests 不引用已移除项（`.to_value()` / `tenant_access_token().app_id` 等 = 0）
- [ ] 4.3 三组 feature clippy（default/all-features/no-default）`-D warnings` exit 0
- [ ] 4.4 `cargo test --workspace` 通过

## 5. 文档与收尾

- [ ] 5.1 CHANGELOG `[Unreleased] > Breaking Changes` 加条目 + 迁移映射表
- [ ] 5.2 评论 #278：本 change 处理 G+D+C（5 干净项）；B（wiki Params ~16 用法）+ F（im 别名 47 文件）仍 open
```

## openspec/changes/remove-unused-deprecated/specs/no-unused-deprecated/spec.md

- Source: openspec/changes/remove-unused-deprecated/specs/no-unused-deprecated/spec.md
- Lines: 1-30
- SHA256: b1b91a5efbc9ea87ea3d8c99564a0e7712bee526f0c1b72c9a01b38c89497a15

```md
## ADDED Requirements

### Requirement: auth 不暴露 deprecated TenantAccessTokenBuilder 方法
openlark-auth 的 `TenantAccessTokenBuilder` SHALL 不再提供 `app_id()` / `app_secret()` / `app_ticket()` deprecated 方法。用户 SHALL 经 `app_access_token()` + `tenant_key()` 流程。

#### Scenario: auth 无目标 deprecated 方法
- **WHEN** 在 `crates/openlark-auth/src/auth/auth/v3/auth/tenant_access_token.rs` 中 grep `pub fn app_id|pub fn app_secret|pub fn app_ticket`
- **THEN** 命中数为 0（3 个 deprecated 方法移除）

### Requirement: docs 不暴露 deprecated to_value 与宏 new
openlark-docs SHALL 不再提供 `RecordFieldValue::to_value()` 与 `impl_required_builder!` 宏生成的 `new()`。

#### Scenario: docs to_value 移除
- **WHEN** 在 `crates/openlark-docs/src/base/bitable/v1/field_types.rs` 中 grep `pub fn to_value`
- **THEN** 命中数为 0

#### Scenario: 宏不再生成 new()
- **WHEN** 在 `crates/openlark-docs/src/common/request_builder.rs` 的 `impl_required_builder!` 宏中 grep `pub fn new`
- **THEN** 命中数为 0（宏的 new() 生成块移除）

### Requirement: 移除不破坏构建与测试
本次移除 SHALL 不导致 default/full/no-default 任一 feature 组合 clippy 或测试失败。

#### Scenario: 三组 feature clippy 通过
- **WHEN** 运行 `cargo clippy --workspace --all-targets` 分别以 default、`--all-features`、`--no-default-features` + `-D warnings`
- **THEN** 三组均 exit 0

#### Scenario: examples/tests 不引用已移除项
- **WHEN** 在 `examples/` 与 `tests/` 中 grep `.to_value()` 与 `tenant_access_token().app_id/app_secret/app_ticket`
- **THEN** 命中数为 0
```

