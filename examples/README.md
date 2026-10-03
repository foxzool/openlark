# OpenLark 示例

当前仓库已经整理为少量可直接运行的根 crate 示例，全部围绕 `openlark` 的推荐入口展开。

## 当前可运行示例

| 示例 | 功能 | 需要的 feature |
|------|------|----------------|
| `readme_quick_start` | 对齐根 README 的快速开始与 docs helper 示例 | `auth,docs-bitable` |
| `client_setup` | 创建 `Client`，并演示 communication 常量 + 可选 lookup helper（默认不发真实请求） | `auth,communication` |
| `communication_workflows` | 用 2 个任务流演示 Communication / Workflow helper 的组合方式 | `auth,communication,workflow` |
| `docs_helpers` | 演示 docs helper：分页遍历、查找 sheet、批量读范围、多维表格全量读取 | `auth,docs-bitable` |
| `docs_workflows` | 用 3 个任务流演示 Drive / Sheets / Wiki / Bitable helper 的组合方式 | `auth,docs-bitable` |
| `websocket_echo_bot` | 长连接接收并回显文本消息 | `communication,websocket` |
| `http_event_inbound` | HTTP 事件入站（challenge / typed 分发 fixture；非 `openlark-webhook` 出站） | `event-http` |
| `websocket_typed_handler` | WS typed `im.message.receive_v1` fixture（可选真实长连接） | `communication,websocket` |
| `oauth_user_token` | 显式 `RequestOption.user_access_token`（不自动刷新） | `auth,communication` |
| `workflow_api_example` | 工作流模块调用 | `workflow` |

## 校验约定

这些对外公开示例会通过 `scripts/check-public-examples.sh` 进入 CI compile-check。

后续新增公开示例时，请遵守：

1. 如果示例会在 README / 文档中主推，必须加入 `scripts/check-public-examples.sh`
2. 如果示例不适合真实执行，也应至少保证 `cargo check --example ...` 可通过
3. 如果文档代码块不能真实运行，请使用 `no_run`、`ignore` 或补充原因说明

## 环境准备

```bash
cp examples/01_getting_started/.env.example .env
```

至少需要：

```bash
export OPENLARK_APP_ID="your_app_id"
export OPENLARK_APP_SECRET="your_app_secret"
```

如需切换到国际版 Lark：

```bash
export OPENLARK_BASE_URL="https://open.larksuite.com"
```

## 运行方式

```bash
cargo run --example readme_quick_start --features "auth,docs-bitable"
cargo run --example client_setup --features "auth,communication"
cargo run --example communication_workflows --features "auth,communication,workflow"
cargo run --example docs_helpers --features "auth,docs-bitable"
cargo run --example docs_workflows --features "auth,docs-bitable"
cargo run --example websocket_echo_bot --features "communication,websocket"
cargo run --example http_event_inbound --features "event-http"
cargo run --example websocket_typed_handler --features "communication,websocket"
cargo run --example oauth_user_token --features "auth,communication"
cargo run --example workflow_api_example --features "workflow"
```

## 配置文档

详细环境变量说明见 [docs/configuration.md](docs/configuration.md)。

Docs helper 设计与命名规范见 [`../docs/docs-helper-guidelines.md`](../docs/docs-helper-guidelines.md)。
Communication / Workflow helper 分层边界见 [`../docs/communication-workflow-helper-boundaries.md`](../docs/communication-workflow-helper-boundaries.md)。
