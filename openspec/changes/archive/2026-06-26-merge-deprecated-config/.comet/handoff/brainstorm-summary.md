# Brainstorm Summary

- Change: merge-deprecated-config
- Date: 2026-06-26

## 确认的技术方案

消除 `openlark_client::Config`（deprecated），统一到 `openlark_core::config::Config`。core 全吸收 client::Config 的独有功能，v0.18 移除 client::Config（breaking）。

**5 个分叉决策**：
1. `builder().build()` **保持不校验**（不破坏现有 core 用户）；新增 `Config::validate()`；`from_env()` 内部调 validate。
2. 字段命名用 **core 命名**（`req_timeout`/`header` 单数），不保留 client 的 `timeout`/`headers` 别名（业务/client/traits 已用 core 命名）。
3. `ConfigInner` 加 `allow_custom_base_url: bool`，同步 Default(false)/Debug/with_token_provider/build/new 所有构造点。
4. `ClientBuilder` 内部从 client::Config 改持 `core::ConfigBuilder`；`build()` → core::Config → `Client::new`（已接 core）；`Client::from_env` 用 `core::Config::from_env`。
5. core `req_timeout` 默认 **None**（永不超时）；`from_env` 读 `OPENLARK_TIMEOUT` → `Some(Duration)`，未设则 None。迁移文档标注默认从 30s 变 None。

**core::Config 新增 API**：`ConfigInner.allow_custom_base_url`、`Config::from_env()/load_from_env()`、`Config::validate()`、`is_known_base_url()`、`ConfigSummary`+`summary()`、`ConfigBuilder::allow_custom_base_url()`。

**移除**：client::Config / ConfigBuilder / ConfigSummary 本体；根 crate re-export 改 core::Config。

## 关键取舍与风险

- **Breaking（v0.18）**：client::Config 移除；字段名 `timeout`→`req_timeout`、`headers`→`header`；timeout 默认 `30s`→`None`。CHANGELOG 给迁移对应表。
- ClientBuilder 持有类型变更（内部 client::Config → core::ConfigBuilder），但公开方法名保持。
- SSRF 防护（base_url 白名单）从 client 层上移到 core（HTTP 出口层），所有 core::Config 路径受益。
- `build()` 不校验意味着直接 build 的用户无 SSRF 防护——但这是 core 现状，不回归；from_env 路径有防护。

## 测试策略

- **core 单测**：from_env（OPENLARK_* 识别、timeout→req_timeout(Some)）；validate（白名单通过/非白名单拒绝/allow_custom 豁免/app_id 空 rejected）；ConfigSummary；allow_custom_base_url 字段默认+builder；build() 不校验回归。
- **client**：ClientBuilder 构建链路（core::ConfigBuilder → Client）；Client::from_env。
- **examples**：编译通过（迁移 ClientConfig → Config）。
- **业务 crate 回归**：core build 仍不校验，业务 crate 不受影响。

## Spec Patch

- `## ADDED Requirements`（capability=config）：core::Config 的 from_env / validate / base_url 白名单 SSRF 防护 / allow_custom_base_url / ConfigSummary
- `## REMOVED Requirements`（capability=config）：client::Config（含 ConfigBuilder/ConfigSummary，deprecated 迁移到 core）
