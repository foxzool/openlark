---
comet_change: merge-deprecated-config
role: technical-design
canonical_spec: openspec
---

# Design Doc: merge-deprecated-config

## 目标

消除 `openlark_client::Config`（deprecated since 0.17.0），统一到 `openlark_core::config::Config`。core 全吸收 client::Config 的有价值功能（含 base_url 白名单 SSRF 防护）。v0.18 移除 client::Config（breaking）。

## 架构

合并后单一 Config：

```
openlark_core::config::Config（Arc<ConfigInner>，零拷贝）
├── ConfigInner
│   ├── app_id, app_secret, base_url, enable_token_cache, app_type
│   ├── http_client, req_timeout: Option<Duration>, header: HashMap
│   ├── token_provider: Arc<dyn TokenProvider>
│   ├── max_response_size, retry_count, enable_log
│   └── allow_custom_base_url: bool          ← 新增（分叉 3）
├── 构造
│   ├── builder() -> ConfigBuilder           （build() 不校验，分叉 1）
│   ├── from_env() / load_from_env()         ← 从 client 迁移，内部 validate
│   └── new(ConfigInner)
├── 校验
│   ├── validate() -> Result<()>             ← 从 client 迁移（SSRF 防护）
│   └── is_known_base_url(url)               ← 迁移到 core
├── 摘要
│   └── summary() -> ConfigSummary           ← 从 client 迁移
└── （保留）with_token_provider / accessors / reference_count
```

## 数据流

```
环境变量 ──from_env()──▶ Config ──内部 validate──▶ （Ok/Err 记录）
                          │
                          ▼
ClientBuilder（持 core::ConfigBuilder）
    │  .app_id/.app_secret/.allow_custom_base_url/...
    ▼
core::ConfigBuilder.build() ──▶ core::Config ──▶ Client::new(config)
```

## 5 个分叉决策

| 分叉 | 决策 | 理由 |
|---|---|---|
| 1. build() 是否校验 | **不校验**；新增 `validate()`；from_env 内部校验 | 不破坏现有所有 core::Config 用户；SSRF 防护通过 from_env + 显式 validate 保留 |
| 2. 字段命名 | 用 core 命名（req_timeout/header），不保留 client 别名 | 业务/client/traits 已全用 core 命名，别名零价值 |
| 3. ConfigInner 新字段 | 加 `allow_custom_base_url: bool`，同步所有构造点 | Arc 封装要求所有 ConfigInner 构造点一致 |
| 4. ClientBuilder | 改持 `core::ConfigBuilder`；Client::from_env 用 core::Config::from_env | Client::new 已接 core；ClientBuilder 是最后残留 client::Config 持有者 |
| 5. timeout 默认 | core 保持 None；from_env 读 OPENLARK_TIMEOUT → Some | 保持 core 原生语义；迁移文档标注 30s→None |

## core::Config 新增 API 详述

```rust
// ConfigInner 新字段
pub(crate) allow_custom_base_url: bool,  // default: false

impl Config {
    pub fn from_env() -> Config { /* load_from_env + 内部 validate（Err 仅日志/记录，不阻塞） */ }
    pub fn load_from_env(&mut self) { /* OPENLARK_* → 字段 */ }
    pub fn validate(&self) -> Result<(), ConfigError> { /* app_id/secret 非空 + base_url 白名单 + allow_custom */ }
    pub fn summary(&self) -> ConfigSummary { /* 不含敏感信息 */ }
    pub fn allow_custom_base_url(&self) -> bool { ... }
}

// free function 迁移到 core
pub fn is_known_base_url(url: &str) -> bool { /* *.feishu.cn/*.larksuite.com/*.larkoffice.com */ }

impl ConfigBuilder {
    pub fn allow_custom_base_url(mut self, allow: bool) -> Self { ... }
    // build() 保持不校验
}
```

**环境变量映射**（from_env）：
- `OPENLARK_APP_ID`→app_id, `OPENLARK_APP_SECRET`→app_secret, `OPENLARK_APP_TYPE`→app_type
- `OPENLARK_BASE_URL`→base_url, `OPENLARK_ENABLE_TOKEN_CACHE`→enable_token_cache
- `OPENLARK_TIMEOUT`→**req_timeout(Some(Duration::from_secs(n)))**（分叉 5，未设则 None）
- `OPENLARK_RETRY_COUNT`→retry_count, `OPENLARK_MAX_RESPONSE_SIZE`→max_response_size, `OPENLARK_ENABLE_LOG`→enable_log

## ClientBuilder 迁移

```rust
// before: ClientBuilder { config: crate::Config (deprecated) }
// after:
pub struct ClientBuilder {
    config_builder: openlark_core::config::ConfigBuilder,  // 或持 core::Config + 增量
    // ...
}
impl ClientBuilder {
    pub fn build(self) -> Result<Client> {
        let config = self.config_builder.build();  // core::Config，不校验
        // 可选：config.validate()?
        Ok(Client::new(config))
    }
    pub fn from_env(mut self) -> Self { /* 合并 OPENLARK_* 到 config_builder */ }
}
```

公开方法名保持（`enable_token_cache`/`retry_count`/`enable_log`/`max_response_size`/`req_timeout` 等），内部委托 core::ConfigBuilder。

## 错误处理

- `validate()` 返回 core 的错误类型（`ConfigError` 或复用现有 core error）。base_url 非白名单错误消息提示 `allow_custom_base_url(true)`。
- `from_env()` 不因 validate 失败而 panic/阻塞（返回 Config，问题留待显式 validate 或运行时）——保持「build 不校验」语义一致。

## 测试策略

- **core**（`config.rs` 单测）：
  - `from_env`：各 OPENLARK_* 识别；TIMEOUT→req_timeout(Some)；缺失字段用默认
  - `validate`：白名单域名 Ok；非白名单 Err；allow_custom=true 豁免；app_id/app_secret 空 Err
  - `ConfigSummary`：字段正确、app_secret 不泄露
  - `allow_custom_base_url`：default false；builder 设置 true
  - 回归：`build()` 不校验（原 core 行为不变）
  - Arc 效率：新字段不破坏 ConfigInner 克隆/with_token_provider 重建
- **client**：ClientBuilder 构建链路；Client::from_env；Client::new(core::Config)
- **examples**：迁移后编译通过
- **业务 crate 回归**：workspace `cargo test` 全绿

## 迁移指引（CHANGELOG 摘要）

| client::Config（旧） | core::Config（新） |
|---|---|
| `timeout: Duration`（默认 30s） | `req_timeout: Option<Duration>`（默认 None） |
| `headers: HashMap` | `header: HashMap` |
| `Config::builder().build()` 返回 `Result`（校验） | `Config::builder().build()` 返回 `Config`（不校验）；显式 `validate()` |
| `openlark_client::Config` | `openlark_core::config::Config`（根 crate `openlark::Config` 自动指向） |

## Spec Patch 摘要

- ADDED（capability=config）：from_env / validate + base_url 白名单 / allow_custom_base_url / ConfigSummary
- REMOVED（capability=config）：client::Config（含 ConfigBuilder/ConfigSummary）
