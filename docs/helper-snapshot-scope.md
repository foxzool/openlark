# 高价值 Helper Snapshot Scope

本文件定义当前 snapshot tests 覆盖的 helper 范围，避免在 issue #74 中把范围无限扩大成“所有 helper 全覆盖”。

## 选择原则

当前 snapshot tests 只覆盖满足以下条件的 helper：

1. 已经在 README / examples / helper 边界文档中被主推
2. 输出结构稳定、适合作为回归锚点
3. 一旦字段、默认值或输出表达变化，会直接影响 SDK 使用体验

## 当前覆盖范围

### Docs

- `SheetRange`
- `SheetWriteRange`
- `DriveDownloadRange`
- `DriveUploadFile`
- `WikiNodePath`
- `TypedPage`
- `BitableRecordQuery`（debug snapshot）

### Communication

- `MessageRecipient`
- `PostMessage`
- `ReplyTarget`
- `MediaImageUpload`
- `MediaFileUpload`
- `UserLookupItem`
- `ChatLookupItem`

### Workflow

- `WorkflowTaskListQuery`
- `WorkflowTaskCreate`
- `WorkflowTaskMutation`
- `ApprovalTaskQuery`
- `ApprovalTaskAction`

## 当前不覆盖的内容

本轮刻意不覆盖：

- 所有 helper 的穷举快照
- 网络请求结果的真实服务端响应
- 低频尾部 helper
- 每个 helper 的所有异常分支

## 扩展顺序

后续新增 snapshot tests 时，按以下顺序扩展：

1. README / examples 主推 helper
2. 跨 crate 组合型 helper
3. 复杂但高频的业务 helper
4. 尾部或低频 helper
