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
