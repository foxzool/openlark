# Communication / Workflow Helper 分层边界说明

本文档定义 `openlark-communication` 与 `openlark-workflow` 中 helper、typed API、以及更高层任务流示例之间的职责边界，供后续 issue / PR 直接引用。

## 1. 分层模型

三个层级：

1. **Typed API 层**
   - 直接映射服务端接口。
   - 负责参数、路径、序列化/反序列化、协议兼容。
   - 例：`CreateMessageRequest`、`ReplyMessageRequest`、`ListTasksRequest`、`ApproveTaskRequestV4`。

2. **Helper 层**
   - 组合 1~N 个 typed API 或将高频输入包装成稳定类型。
   - 负责默认值、常见任务动作、常见错误表达。
   - 例：`MessageRecipient`、`PostMessage`、`ReplyTarget`、`MediaImageUpload`、`WorkflowTaskListQuery`、`ApprovalTaskAction`。

3. **Scenario / Workflow 示例层**
   - 演示“如何把多个 helper 串成完整业务流”。
   - 不扩展底层能力，只展示推荐用法与环境变量约束。
   - 例：`communication_workflows`、`workflow_api_example`、`docs_workflows`。

## 2. 什么时候应该新增 helper

适合新增 helper 的信号：

- 调用方经常要重复拼装 2 个以上参数才能表达一个稳定业务动作。
- 同一类 JSON 内容需要在多个地方重复手写。
- 同一类“查找 -> 发送 / 查找 -> 变更 / 查找 -> 审批”会反复出现。
- typed API 已存在，但业务侧仍需要重复处理分页、唯一命中、默认值或上下文对象。

不适合新增 helper 的信号：

- 只是把单个 typed request 换了个名字。
- 需要完整 DSL、复杂缓存、索引或跨域状态机。
- 只是为了少写一行 `new(config)`。
- 无法说清 helper 隐藏了什么复杂度。

## 3. Communication 边界

### 3.1 应保留在 typed API 的能力

- 单个 endpoint 的细粒度参数控制。
- 全量消息类型支持。
- 复杂 chat 管理、菜单树、管理员管理、历史消息资源读取。
- 需要直接贴合服务端字段命名/行为的能力。

### 3.2 应进入 helper 的能力

- **高频发送动作**：
  - `send_text`
  - `send_post`
  - `reply_text`
  - `reply_post`
  - `forward_thread`
- **高频媒体动作**：
  - `upload_image`
  - `upload_file`
  - `send_image`
  - `send_file`
- **高频前置查找动作**：
  - `search_users_all`
  - `find_user_by_name`
  - `search_chats_all`
  - `find_chat_by_name`

### 3.3 Communication helper 设计规则

- **上下文对象优先**：
  - 接收者用 `MessageRecipient`
  - 回复上下文用 `ReplyTarget`
  - 媒体上传上下文用 `MediaImageUpload` / `MediaFileUpload`
- **消息体模板化，但不 DSL 化**：
  - `PostMessage` 只覆盖高频“标题 + 单段文本”场景
  - 更复杂卡片/富文本结构继续放在 typed API 或上层自行构建
- **查找 helper 必须处理歧义**：
  - 0 个命中：报业务错误
  - 多个命中：报业务错误
  - 不允许静默取第一个
- **helper 不应吞掉底层 typed API**：
  - 底层 request 仍应可直接使用
  - helper 只是推荐入口，不是替代品

## 4. Workflow 边界

### 4.1 应保留在 typed API 的能力

- Task/Tasklist/Approval 的完整字段控制。
- 需要完整遵守服务端契约的审批 v4 请求体。
- 较低频、参数组合复杂的审批实例/评论/外部实例管理。

### 4.2 应进入 helper 的能力

- **任务流 helper**：
  - `WorkflowTaskListQuery`
  - `WorkflowTaskMutation`
  - `list_tasks_all`
  - `mutate_task`
  - `complete_task`
  - `reopen_task`
- **审批流 helper**：
  - `ApprovalTaskQuery`
  - `ApprovalTaskAction`
  - `query_approval_tasks`
  - `approve_task` / `reject_task` / `resubmit_task`（返回 `SDKResult<()>`；成功/失败只由 `Result` 表达，不伪造 `success: bool`，见 #350）

### 4.3 Workflow helper 设计规则

- **helper 允许抽象高频动作，但不能隐藏服务端必填语义**。
  - 例如 approval v4 中 `approval_code`、`instance_code`、`user_id`、`task_id` 不能为了“简洁”而被删掉。
- **`list_*_all` 只有在真正自动翻页时才允许使用。**
  - 如果 helper 只做单页查询，应命名为 `query_*` 或 `search_*`。
- **任务型 helper 可以组合多个 typed request，但 typed request 仍是 source of truth。**
- **审批 helper 要以官方 `.md` 文档为准**，尤其是：
  - 请求参数
  - response data 是否为空
  - 字段别名映射（如 external_id / code）

## 5. 命名规范

### 5.1 Helper 类型

- 上下文对象：名词
  - `MessageRecipient`
  - `ReplyTarget`
  - `ApprovalTaskAction`
- 查询对象：`*Query`
  - `WorkflowTaskListQuery`
  - `ApprovalTaskQuery`
- 上传对象：`*Upload`
  - `MediaImageUpload`
  - `MediaFileUpload`

### 5.2 Helper 方法

- `send_*`：发送动作
- `reply_*`：回复动作
- `upload_*`：上传动作
- `find_*_by_*`：唯一命中查找
- `search_*_all`：自动翻页搜索
- `query_*`：单类条件查询，但不承诺全量翻页
- `mutate_*`：高频变更动作
- `complete_*` / `reopen_*`：状态切换动作
- `approve_*` / `reject_*` / `resubmit_*`：审批动作

## 6. 返回值与错误规则

### 6.1 返回值

- helper 优先返回“业务结果”而不是原始响应壳。
- 允许返回：
  - `Vec<T>`
  - typed model
  - helper 自己的稳定小类型
- 不鼓励 helper 再把 `serde_json::Value` 暴露给调用方，除非底层契约当前无法稳定建模。

### 6.2 错误

- **参数本地可判定错误**：`validation_error`
- **0 命中 / 多命中 / 业务歧义**：`business_error`
- **底层请求失败**：透传 typed API 错误

## 7. 现有 helper 的边界结论

### Communication

| 能力 | 应留层级 | 原因 |
|------|----------|------|
| text/post/reply/thread 常见动作 | helper | 高频、重复、上下文稳定 |
| image/file upload + send | helper | 消息流中的常见前置步骤 |
| user/chat lookup | helper | 前置发现动作，高频且易错 |
| card patch / urgent / reaction | typed API | 参数更细、场景更分散 |

### Workflow

| 能力 | 应留层级 | 原因 |
|------|----------|------|
| list/mutate/complete/reopen task | helper | 典型任务流动作 |
| query/approve/reject/resubmit approval task | helper | 高频审批动作，但必须保留官方必填字段 |
| approval instance/comment/external instance 深度能力 | typed API | 契约复杂、频率相对低 |

## 8. 对后续 issue / PR 的判断标准

新增 helper 前，请回答：

1. 它是否真的减少了重复业务动作？
2. 它是否隐藏了“默认值 / 分页 / 唯一命中 / 上下文组合”中的至少一个复杂点？
3. 它是否仍然尊重 typed API 的服务端契约？
4. 它是否有清晰示例或测试能说明价值？
5. 如果删掉这个 helper，调用方是否会明显重复写样板代码？

如果回答多数为“否”，优先补 typed API，而不是继续扩展 helper。
