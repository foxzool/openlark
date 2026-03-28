# OpenLark 示例

当前仓库已经整理为少量可直接运行的根 crate 示例，全部围绕 `openlark` 的推荐入口展开。

## 当前可运行示例

| 示例 | 功能 | 需要的 feature |
|------|------|----------------|
| `simple_api_call` | 创建 `Client` 并访问 communication 常量 | `auth,communication` |
| `docs_helpers` | 演示 docs helper：分页遍历、查找 sheet、批量读范围、多维表格全量读取 | `auth,docs-bitable` |
| `websocket_echo_bot` | 长连接接收并回显文本消息 | `communication,websocket` |
| `workflow_api_example` | 工作流模块调用 | `workflow` |

## 环境准备

```bash
cp examples/.env.example .env
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
cargo run --example simple_api_call --features "auth,communication"
cargo run --example docs_helpers --features "auth,docs-bitable"
cargo run --example websocket_echo_bot --features "communication,websocket"
cargo run --example workflow_api_example --features "workflow"
```

## 配置文档

详细环境变量说明见 [../docs/configuration.md](../docs/configuration.md)。
