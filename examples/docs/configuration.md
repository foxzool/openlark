# OpenLark 示例配置

本文档只描述当前仓库里真实存在的示例配置方式。

## 必需环境变量

```bash
export OPENLARK_APP_ID="your_app_id"
export OPENLARK_APP_SECRET="your_app_secret"
```

## 可选环境变量

```bash
export OPENLARK_BASE_URL="https://open.feishu.cn"
export OPENLARK_USER_SEARCH_NAME="zhangsan"
export OPENLARK_CHAT_SEARCH_NAME="项目群"
export OPENLARK_TEXT_MESSAGE="你好"
export OPENLARK_POST_TITLE="项目播报"
export OPENLARK_POST_TEXT="今天完成发布"
export OPENLARK_FOLDER_TOKEN="folder_token"
export OPENLARK_DOWNLOAD_FILE_TOKEN="file_token"
export OPENLARK_DOWNLOAD_RANGE_DEMO="1"
export OPENLARK_UPLOAD_FILE_PATH="/path/to/file"
export OPENLARK_SPREADSHEET_TOKEN="spreadsheet_token"
export OPENLARK_SHEET_TITLE="汇总表"
export OPENLARK_SHEETS_WRITE_DEMO="1"
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

如果你使用国际版 Lark：

```bash
export OPENLARK_BASE_URL="https://open.larksuite.com"
```

## 推荐做法

```bash
cp examples/01_getting_started/.env.example .env
```

然后把上面的变量写进 `.env`。

## 运行示例

```bash
cargo run --example client_setup --features "auth,communication"
cargo run --example communication_workflows --features "auth,communication,workflow"
cargo run --example docs_helpers --features "auth,docs-bitable"
cargo run --example docs_workflows --features "auth,docs-bitable"
cargo run --example websocket_echo_bot --features "communication,websocket"
```

## 说明

- `client_setup` 默认只演示客户端初始化；当你提供 `OPENLARK_USER_SEARCH_NAME` / `OPENLARK_CHAT_SEARCH_NAME` 时，才会额外触发 user/chat lookup helper。
- `communication_workflows` 会串起“查人/查群 -> 发消息”和“创建/列任务 -> 更新任务 -> 处理审批”两类任务流。
- `docs_helpers` 会按你是否提供相关 token，分别演示文件夹遍历、Drive 上传/下载、sheet 查找、批量读范围与多维表格读取；如需启用分片下载/批量写入演示，再额外设置 `OPENLARK_DOWNLOAD_RANGE_DEMO=1` / `OPENLARK_SHEETS_WRITE_DEMO=1`。
- `docs_workflows` 会以任务流方式串起 Drive 文件流转、Spreadsheet 周报处理，以及 Wiki / Bitable 巡检流程。
- `websocket_echo_bot` 需要额外完成飞书事件订阅和长连接配置。

## 风险提示

- `communication_workflows` 在环境变量完整时，会执行真实的消息发送、任务创建/更新和审批动作；
  任务创建需额外设置 `OPENLARK_WORKFLOW_CREATE_TASK=1`（与 `OPENLARK_WORKFLOW_TASKLIST_GUID` 联用）。
- `workflow_api_example` 使用的是可编译的占位参数示例，适合先验证 helper 调用结构。
- 如果你只想验证示例可编译，请不要设置这些动作相关环境变量，或保持其值为空。
