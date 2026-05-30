## ADDED Requirements

### Requirement: 单一 Config 入口
用户 SHALL 仅通过 `Client::builder()` 或 `openlark_core::config::Config` 接触配置。`openlark_client::Config` SHALL 被标记为 deprecated 并最终移除。

#### Scenario: 通过 Client::builder 创建客户端
- **WHEN** 用户调用 `Client::builder().app_id("x").app_secret("y").build()`
- **THEN** 返回的 Client 内部使用 `openlark_core::config::Config`（Arc\<ConfigInner\>），无需感知两种 Config

#### Scenario: Client::from_env 创建客户端
- **WHEN** 环境变量 `OPENLARK_APP_ID` 和 `OPENLARK_APP_SECRET` 已设置
- **THEN** `Client::from_env()` 成功创建客户端，内部使用统一的 CoreConfig

### Requirement: Client 不持有两种 Config
Client 结构体 SHALL 仅包含一个 `core_config: openlark_core::config::Config` 字段，不另存 `openlark_client::Config`。

#### Scenario: Client::config() 返回 CoreConfig
- **WHEN** 用户调用 `client.config()` 获取配置
- **THEN** 返回 `&openlark_core::config::Config` 类型

### Requirement: Config 构建器支持所有原有选项
`openlark_core::config::Config::builder()` SHALL 支持与原 `openlark_client::Config` 相同的构建选项（app_id, app_secret, base_url, timeout, enable_log, retry_count 等）。

#### Scenario: 构建带超时的配置
- **WHEN** 用户调用 `Config::builder().app_id("x").app_secret("y").timeout(Duration::from_secs(30)).build()`
- **THEN** 配置中 `req_timeout` 为 `Some(30s)`
