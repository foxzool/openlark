# 入门示例

这一组示例对齐 `0.16.0` 的根 crate 用法，重点是 `Client` 单入口和按需 feature。

## 示例列表

| 示例 | 说明 | 运行命令 |
|------|------|----------|
| `readme_quick_start` | 根 README 对齐示例，覆盖文档 helper 入口 | `cargo run --example readme_quick_start --features "auth,docs-bitable"` |
| `client_setup` | communication 配置示例，验证客户端初始化、模块访问以及可选 user/chat lookup helper（默认不发真实请求） | `cargo run --example client_setup --features "auth,communication"` |
| `communication_workflows` | 用 2 组任务流串起 Communication / Workflow helper 的组合方式 | `cargo run --example communication_workflows --features "auth,communication,workflow"` |
| `docs_helpers` | 演示 docs helper，包括文件夹遍历、sheet 查找、批量读范围、多维表格全量读取 | `cargo run --example docs_helpers --features "auth,docs-bitable"` |
| `docs_workflows` | 以 Drive / Sheets / Wiki / Bitable 三组任务流展示 Docs helper 的组合方式 | `cargo run --example docs_workflows --features "auth,docs-bitable"` |
| `websocket_echo_bot` | 长连接消息回显 | `cargo run --example websocket_echo_bot --features "communication,websocket"` |

## 环境变量

```bash
cp examples/01_getting_started/.env.example .env
export OPENLARK_APP_ID="your_app_id"
export OPENLARK_APP_SECRET="your_app_secret"
```

可选变量：

```bash
export OPENLARK_BASE_URL="https://open.feishu.cn"
export OPENLARK_USER_SEARCH_NAME="zhangsan"
export OPENLARK_CHAT_SEARCH_NAME="项目群"
export OPENLARK_TEXT_MESSAGE="你好"
export OPENLARK_POST_TITLE="项目播报"
export OPENLARK_POST_TEXT="今天完成发布"
export OPENLARK_FOLDER_TOKEN="folder_token"
export OPENLARK_DOWNLOAD_FILE_TOKEN="file_token"
export OPENLARK_UPLOAD_FILE_PATH="/path/to/file"
export OPENLARK_SPREADSHEET_TOKEN="spreadsheet_token"
export OPENLARK_SHEET_TITLE="汇总表"
export OPENLARK_SHEETS_WRITE_RANGE="A20:B20"
export OPENLARK_SHEETS_APPEND_RANGE="A30:B30"
export OPENLARK_BITABLE_APP_TOKEN="app_token"
export OPENLARK_BITABLE_TABLE_ID="table_id"
export OPENLARK_BITABLE_FILTER_FIELD="状态"
export OPENLARK_BITABLE_FILTER_VALUE="进行中"
export OPENLARK_WIKI_SPACE_ID="space_id"
export OPENLARK_WIKI_NODE_PATH="产品文档/发布计划"
export OPENLARK_WORKFLOW_TASKLIST_GUID="tasklist_guid"
export OPENLARK_WORKFLOW_TASK_GUID="task_guid"
export OPENLARK_APPROVAL_USER_ID="ou_example_user"
export OPENLARK_APPROVAL_TOPIC="1"
```

## 学习顺序

1. 先运行 `client_setup`，确认 `Client::from_env()` 和 `Client::builder()` 都能正常工作。
2. 如需验证真实 lookup 请求，可设置 `OPENLARK_USER_SEARCH_NAME` / `OPENLARK_CHAT_SEARCH_NAME` 后再次运行 `client_setup`。
3. 再运行 `communication_workflows`，查看 Communication / Workflow helper 如何串成完整业务流。
4. 接着运行 `readme_quick_start` 或 `docs_helpers`，理解 `0.16.0` 推荐的文档 helper 能力。
5. 再运行 `docs_workflows`，查看任务型 Docs 工作流如何组合 Drive / Sheets / Wiki / Bitable helper。
6. 最后运行 `websocket_echo_bot`，验证长连接能力。

## 风险提示

- `communication_workflows` 在你提供完整环境变量时，会执行真实的消息发送、任务更新和审批动作。
- 如只想做编译验证，请保持这些可选环境变量为空或不要设置。
