---
change: merge-deprecated-config
design-doc: docs/superpowers/specs/2026-06-26-merge-deprecated-config-design.md
base-ref: 540f9559a0507870d344ed2ba321fbe11375c353
---

# 实施计划：merge-deprecated-config

> 关联 Design Doc 的 5 个分叉已定稿，本计划据此拆分，不重新决策。
> 验收以 cargo 命令 + grep 断言为准。

## 任务依赖图

```
T1 (ConfigInner 字段) ─┬─▶ T2 (validate+白名单) ─▶ T3 (from_env)
                       ├─▶ T5 (Builder)
                       └─▶ T4 (Summary)
T1..T5 ────────────────▶ T6 (移除 client::Config) ─▶ T7 (ClientBuilder) ─▶ T8 (根 crate) ─▶ T9 (examples)
                                                                                        T10 (文档/CHANGELOG)
                                                                                        T11 (全量验证)
```

## 任务

### T1: core `ConfigInner` 加 `allow_custom_base_url`
- 文件：`crates/openlark-core/src/config.rs`
- 要点：
  - `ConfigInner` 加 `pub(crate) allow_custom_base_url: bool`
  - `Default`：`allow_custom_base_url: false`
  - `Debug`：加 `.field("allow_custom_base_url", &self.allow_custom_base_url)`
  - `Config::with_token_provider`：逐字段重建 ConfigInner 时补 `allow_custom_base_url: self.allow_custom_base_url`
  - `ConfigBuilder`：加 `allow_custom_base_url: Option<bool>` 字段；`build()` 补 `allow_custom_base_url: self.allow_custom_base_url.unwrap_or(default.allow_custom_base_url)`
  - `Config` 加 accessor `allow_custom_base_url(&self) -> bool`
- 验收：`cargo test -p openlark-core --lib config` 全绿；`grep allow_custom_base_url crates/openlark-core/src/config.rs` 命中所有构造点

### T2: core `Config::validate()` + `is_known_base_url()`（SSRF 白名单）
- 文件：`crates/openlark-core/src/config.rs`（从 `client/config.rs:14` 迁移 `is_known_base_url`）
- 要点：
  - 迁移 `pub(crate) fn is_known_base_url(url) -> bool`（白名单 `feishu.cn`/`larksuite.com`/`larkoffice.com`）
  - `Config::validate(&self) -> Result<(), ConfigError>`（core 错误类型；client 用 `crate::error::validation_error`，core 用对应 core error）：app_id/secret 非空 + base_url 格式 + 白名单（`allow_custom_base_url` 豁免）+ retry_count<=10
  - **`builder().build()` 保持不校验**（分叉 1）
- 验收：单测覆盖 — 白名单域名 Ok（feishu.cn/larksuite.com/larkoffice.com）、非白名单 Err（evil.com）、allow_custom=true 豁免 Ok、app_id 空 Err；`build()` 不校验回归（`Config::builder().app_id("").build()` 不抛错）

### T3: core `Config::from_env()` / `load_from_env()`
- 文件：`crates/openlark-core/src/config.rs`（从 `client/config.rs:141-196` 迁移）
- 要点：
  - `Config::from_env() -> Config`（load_from_env 后返回；**内部调 validate，Err 仅记录不阻塞**——与 build 不校验语义一致）
  - `load_from_env(&mut self)`：`OPENLARK_APP_ID/APP_SECRET/APP_TYPE/BASE_URL/ENABLE_TOKEN_CACHE/RETRY_COUNT/MAX_RESPONSE_SIZE/ENABLE_LOG`
  - `OPENLARK_TIMEOUT` → `req_timeout(Some(Duration::from_secs(n)))`（分叉 5，未设保持 None）
  - 适配 Arc<ConfigInner>：from_env 生成 ConfigInner 并 Config::new(inner)
- 验收：from_env 单测（with_env_vars 辅助：各 OPENLARK_* 识别 + TIMEOUT→req_timeout(Some)）；缺失字段用默认

### T4: core `ConfigSummary` + `Config::summary()`
- 文件：`crates/openlark-core/src/config.rs`（从 `client/config.rs:507-565` 迁移 ConfigSummary）
- 要点：
  - `ConfigSummary`（app_id, app_secret_set: bool, app_type, enable_token_cache, base_url, allow_custom_base_url, req_timeout, retry_count, enable_log, header_count, max_response_size）
  - `Config::summary() -> ConfigSummary`（app_secret 以 `!is_empty()` 布尔表示）
  - 保留 `friendly_description()`/`Display`（可选）
- 验收：summary 单测（字段正确，app_secret 不泄露）

### T5: core `ConfigBuilder::allow_custom_base_url()`
- 文件：`crates/openlark-core/src/config.rs`
- 要点：`ConfigBuilder::allow_custom_base_url(mut self, allow: bool) -> Self`（T1 已加字段，本任务加方法）
- 验收：`Config::builder().allow_custom_base_url(true).build()` 后 `config.allow_custom_base_url() == true`

### T6: client 移除 deprecated `Config`/`ConfigBuilder`/`ConfigSummary`
- 文件：`crates/openlark-client/src/config.rs`、`utils.rs`、`lib.rs`
- 依赖：T1-T5（core 吸收完成）
- 要点：
  - 删除 `client/config.rs` 的 `Config`/`ConfigBuilder`/`ConfigSummary`/`is_known_base_url` 本体（可整个文件删除或保留迁移说明）
  - `utils.rs:3` 的 `use crate::config::ConfigSummary` → 改用 `openlark_core::config::ConfigSummary`（或移除该用法）
  - `lib.rs:277` `pub use config::Config` → 移除；`pub mod config` 移除或改为 re-export core
  - 处理 `client/config.rs` 内 `#![allow(deprecated)]` 等残留
- 验收：`rg 'pub struct Config\b' crates/openlark-client/src` 无命中；client crate `cargo check` 通过

### T7: client `ClientBuilder` 改持 core::ConfigBuilder
- 文件：`crates/openlark-client/src/client/builder.rs`、`client.rs`
- 依赖：T6
- 要点：
  - `ClientBuilder.config: crate::Config` → 改持 `openlark_core::config::ConfigBuilder`（或持 core::Config + 增量构建）
  - 公开方法名保持（`enable_token_cache`/`retry_count`/`enable_log`/`max_response_size`/`req_timeout`/`allow_custom_base_url` 等），内部委托 core::ConfigBuilder
  - `build()`：`core::ConfigBuilder.build()` → `Client::new(core_config)`
  - `from_env(mut self)`：合并 OPENLARK_* 到 core::ConfigBuilder（或 build 后 load_from_env）
  - `Client::from_env`：用 `openlark_core::config::Config::from_env` + validate → `Client::new`
- 验收：`cargo test -p openlark-client` 全绿（含 client/tests.rs 的 req_timeout/retry_count 断言）；Client 构建链路通

### T8: 根 crate re-export 改 core::Config
- 文件：`src/lib.rs`
- 依赖：T7
- 要点：`src/lib.rs:31` `pub use openlark_client::Config` → `pub use openlark_core::config::Config`
- 验收：根 crate `cargo check` 通过；`openlark::Config` 现为 core::Config

### T9: examples 迁移
- 文件：`examples/`（grep `ClientConfig`/`client::Config`/`openlark_client::Config`）
- 依赖：T8
- 要点：把所有用 `ClientConfig`/`client::Config` 的示例改用 `Config`（core）或 `Client::builder()`/`Client::from_env()`。`examples/test_debug.rs` 同时用 Config 和 ClientConfig → 统一
- 验收：`cargo check --workspace --all-targets`（含 examples）通过

### T10: 文档 + CHANGELOG
- 文件：CHANGELOG.md、相关 README、AGENTS.md（更新 Config 迁移条目）
- 依赖：T8/T9
- 要点：breaking 迁移对应表（`timeout`→`req_timeout`、`headers`→`header`、默认 timeout 30s→None、`client::Config`→`core::Config`、`builder().build()` 返回类型 Result→Config）
- 验收：CHANGELOG 含 v0.18 breaking 区段 + 迁移表

### T11: 全量验证
- 依赖：全部
- 验收：`cargo test --workspace`、`cargo clippy --workspace --all-targets`、`cargo check --workspace --all-targets` 全绿；`rg 'openlark_client::Config|pub struct Config\b' crates/openlark-client/src` 无残留
