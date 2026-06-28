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
