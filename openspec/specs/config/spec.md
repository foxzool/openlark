# config Specification

## Purpose
TBD - created by archiving change merge-deprecated-config. Update Purpose after archive.
## Requirements
### Requirement: 环境变量加载 Config

`openlark_core::config::Config` SHALL 提供 `from_env()` / `load_from_env()`，从 `OPENLARK_*` 环境变量加载配置。

#### Scenario: 识别 OPENLARK_* 环境变量
- WHEN 调用 `Config::from_env()` 且环境含 `OPENLARK_APP_ID`/`APP_SECRET`/`APP_TYPE`/`BASE_URL`/`ENABLE_TOKEN_CACHE`/`TIMEOUT`/`RETRY_COUNT`/`MAX_RESPONSE_SIZE`/`ENABLE_LOG`
- THEN 返回的 Config 各字段对应环境变量值；`OPENLARK_TIMEOUT` 映射为 `req_timeout(Some(Duration))`；缺失字段用默认值

#### Scenario: from_env 不阻塞于无效配置
- WHEN `from_env()` 加载的配置无效（如 app_id 空）
- THEN `from_env()` 仍返回 Config（不 panic），无效性由后续显式 `validate()` 发现——与 `build()` 不校验语义一致

### Requirement: Config 校验与 base_url 白名单 SSRF 防护

`openlark_core::config::Config` SHALL 提供 `validate()`，校验 app_id/app_secret 非空、base_url 格式与域名白名单（SSRF 防护），由 `allow_custom_base_url` 控制白名单豁免。`builder().build()` SHALL NOT 自动校验。

#### Scenario: 白名单域名通过
- WHEN base_url 为 `*.feishu.cn`/`*.larksuite.com`/`*.larkoffice.com` 且 `allow_custom_base_url=false`
- THEN `validate()` 返回 Ok

#### Scenario: 非白名单域名拒绝
- WHEN base_url 为非白名单域名（如 `https://evil.com`）且 `allow_custom_base_url=false`
- THEN `validate()` 返回 Err，提示可设置 `allow_custom_base_url(true)`

#### Scenario: allow_custom_base_url 豁免白名单
- WHEN base_url 为非白名单域名但 `allow_custom_base_url=true`
- THEN `validate()` 返回 Ok

#### Scenario: app_id 或 app_secret 为空
- WHEN app_id 或 app_secret 为空字符串
- THEN `validate()` 返回 Err

#### Scenario: build 不自动校验
- WHEN `Config::builder().app_id("").build()`（app_id 空）
- THEN `build()` 返回 Config（不抛错），与 core 现有行为一致

### Requirement: Config 配置摘要

`openlark_core::config::Config` SHALL 提供 `summary() -> ConfigSummary`，返回不含敏感信息（app_secret 以布尔「是否已设置」表示）的配置摘要。

#### Scenario: 生成不含敏感信息的摘要
- WHEN 调用 `config.summary()`
- THEN 返回的 ConfigSummary 含 app_id/base_url/req_timeout/retry_count/enable_log/header_count/max_response_size，app_secret 仅以布尔表示是否已设置

### Requirement: Config 支持 allow_custom_base_url

`openlark_core::config::ConfigInner` SHALL 含 `allow_custom_base_url: bool` 字段（默认 false）；`ConfigBuilder` SHALL 提供 `allow_custom_base_url(bool)` 设置方法。

#### Scenario: 默认 false
- WHEN `Config::default()`
- THEN `allow_custom_base_url == false`

#### Scenario: builder 设置 true
- WHEN `Config::builder().allow_custom_base_url(true).build()`
- THEN `config.allow_custom_base_url() == true`

#### Scenario: Arc 操作保持一致
- WHEN 对含 allow_custom_base_url 的 Config 执行 `clone()` 或 `with_token_provider()`
- THEN 新 Config 的 allow_custom_base_url 值与原一致（ConfigInner 所有构造点同步）

