## Why

`openlark-workflow` task v2 的 `section` / `custom_field` 两组 CRUD（共 9 个 API、10 个 `TaskApiV2` variant）endpoint URL 写错，调用必返回 404。代码错用 tasklist 作用域前缀 `/tasklists/{tasklist_guid}/...`，而飞书官方这两组是**全局端点** `/sections`、`/custom_fields`（custom_field 通过 body `resource_type`/`resource_id` 定位清单）。

证据：`reports/api_validation/coverage-gap-verification-2026-06-26.md`（43 路交叉验证 + 官方文档 `.md` 逐项核对）；追踪 issue #264。

严重度 P0：实现文件、Request/Response 结构体、Builder、单测全齐，但 URL 不存在——静态检查/编译发现不了，用户调用直接 404。

## What Changes

1. `common/api_endpoints.rs` 的 10 个 variant URL 模板去掉 `tasklists/{tasklist_guid}/` 前缀，改为全局端点；variant 签名去掉 `tasklist_guid` 参数。
2. `section` / `custom_field` 各 5 个 CRUD Request 文件 + 两个 `mod.rs` builder 去掉 `tasklist_guid` 字段与 `with_tasklist()` 方法。
3. `custom_field/create`：`tasklist_guid` 移入 `CreateCustomFieldBody` 的 `resource_type="tasklist"` + `resource_id`。
4. 同步 10 个 variant 的 URL 单测断言。

## Capabilities

### Modified Capabilities
本次为**修正错误实现以匹配飞书官方文档**，section/custom_field 的功能契约不变（仍是这些 CRUD），仅 endpoint URL 纠正，**不引入 spec 级需求变更**，不产出 delta spec。

## Impact

- **Breaking change（公开 API）**：`Section`/`CustomField` builder 的 `with_tasklist()` 移除；10 个 CRUD 方法/Request 构造签名变更（去掉 `tasklist_guid` 参数）。这是修不可用 API（调用即 404）的必然结果，breaking 可接受。
- 受影响文件 ~14 个（清单见 design.md）。
- 影响用户：现有调用 task v2 section/custom_field CRUD 的代码需迁移（迁移说明见 design.md）。
