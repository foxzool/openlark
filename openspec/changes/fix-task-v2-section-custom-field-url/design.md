## 方案

单根因修复：把 task v2 section/custom_field CRUD 的 endpoint URL 从错误的 tasklist 作用域改为官方全局端点。无设计分歧，无备选方案。

### URL 映射（10 个 variant）

| variant | 修复前（错误，404） | 修复后（官方全局） |
|---|---|---|
| SectionCreate | `POST /tasklists/{tl}/sections` | `POST /sections` |
| SectionList | `GET /tasklists/{tl}/sections` | `GET /sections` |
| SectionGet | `GET /tasklists/{tl}/sections/{s}` | `GET /sections/{section_guid}` |
| SectionUpdate | `PATCH /tasklists/{tl}/sections/{s}` | `PATCH /sections/{section_guid}` |
| SectionDelete | `DELETE /tasklists/{tl}/sections/{s}` | `DELETE /sections/{section_guid}` |
| CustomFieldCreate | `POST /tasklists/{tl}/custom_fields` | `POST /custom_fields` |
| CustomFieldList | `GET /tasklists/{tl}/custom_fields` | `GET /custom_fields` |
| CustomFieldGet | `GET /tasklists/{tl}/custom_fields/{f}` | `GET /custom_fields/{custom_field_guid}` |
| CustomFieldUpdate | `PATCH /tasklists/{tl}/custom_fields/{f}` | `PATCH /custom_fields/{custom_field_guid}` |
| CustomFieldDelete | `DELETE /tasklists/{tl}/custom_fields/{f}` | `DELETE /custom_fields/{custom_field_guid}` |

variant 签名相应去掉 `tasklist_guid`：`SectionGet(section_guid)`、`CustomFieldCreate()`、`CustomFieldGet(field_guid)` 等。

### custom_field/create 的 body 改造

官方 `POST /custom_fields` 通过 body 指定挂载清单：`{ resource_type: "tasklist", resource_id: <清单 GUID>, ... }`。

`CreateCustomFieldBody`（`custom_field/models.rs`）增加两字段：
```rust
pub resource_type: String, // "tasklist"（目前官方仅支持 tasklist）
pub resource_id: String,   // 清单 GUID
```
`CreateCustomFieldRequest::new(config, resource_id)` 自动填 `resource_type = "tasklist"`。

### 不变的部分（已确认 URL 正确，不动）

- `section/tasks`：`SectionGetTasks` → `/sections/{guid}/tasks`，全局，正确。
- `custom_field/add`、`remove`、`option/{create,patch}`：均全局 URL，正确。

## Breaking Change 与迁移

公开 builder 方法签名变更（修 bug 的必然结果）：

- `Section` builder：移除 `with_tasklist(guid)` 与构造期 `tasklist_guid`；CRUD 方法不再收 `tasklist_guid`（section 官方仅靠 `section_guid` 定位）。
- `CustomField` builder：同上移除 `tasklist_guid`；`create` 改为通过 body `resource_id` 传清单 GUID。

迁移：原 `tasklist_guid` 参数从这些方法消失——section 完全不需要，custom_field create 改入 body。CHANGELOG 标注 breaking。

## 受影响文件（~14）

- `common/api_endpoints.rs`（10 variant 定义 + URL 模板 + 测试断言）
- `v2/section/{create,get,update,delete,list}.rs` + `v2/section/mod.rs`（`patch.rs` 为 update 的 re-export，跟随）
- `v2/custom_field/{create,get,update,delete,list}.rs` + `v2/custom_field/mod.rs`
- `v2/custom_field/models.rs`（`CreateCustomFieldBody` 加 `resource_type`/`resource_id`）

## 风险与回归

- breaking：用户代码编译失败。缓解：被改的方法当前调用即 404，等于不可用，breaking 可接受。
- 回归面集中在 workflow task v2 的 section/custom_field，由各 variant 的 URL 单测断言守住。
