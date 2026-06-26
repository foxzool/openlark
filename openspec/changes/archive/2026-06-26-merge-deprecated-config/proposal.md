## Why

`openlark_client::Config`（`client/config.rs:60`，deprecated since 0.17.0）与 `openlark_core::config::Config`（`core/config.rs:39`）并存，造成双轨。现状问题：

- **业务 crate 已统一用 core::Config**，但**根 crate `src/lib.rs:31` 仍 re-export deprecated 的 client::Config**——用户从根 crate 拿到的是 deprecated 类型。
- client::Config 持有 core::Config **缺失的安全功能**（`validate` + base_url 白名单 SSRF 防护 + `allow_custom_base_url`）和便利功能（`from_env`、`ConfigSummary`）。简单删除会丢失这些。
- 两个 Config 字段命名/结构不一致（client 普通 struct + `timeout: Duration`/`headers`；core Arc 封装 + `req_timeout: Option<Duration>`/`header`），维护负担。

目标：合并为单一 `core::config::Config`，v0.18 移除 client::Config（breaking，已 deprecated 一个版本）。

## What Changes

1. **core::Config 全吸收** client::Config 的有价值功能：`from_env`/`load_from_env`、`validate`、base_url 白名单（`is_known_base_url`）、`allow_custom_base_url`、`ConfigSummary`。`ConfigInner` 加 `allow_custom_base_url` 字段。
2. **移除 client::Config**：删除 deprecated 的 `Config`/`ConfigBuilder`/`ConfigSummary` 本体；`Client::new`/`builder` 改用 `core::Config`。
3. **根 crate re-export** 改指向 `openlark_core::config::Config`。
4. **examples + 文档 + CHANGELOG** 迁移，给出 breaking 迁移指引（字段/方法对应表）。

## Capabilities

### Modified Capabilities
- **config**（现有 capability）：core::Config 新增 `from_env` / `validate` / base_url 白名单 / `ConfigSummary` 等 requirements；移除 client::Config 的独立 requirements。delta spec 在 design 阶段（brainstorming 后）产出。

## Impact

- **Breaking（v0.18）**：`openlark_client::Config` 移除；根 crate `openlark::Config` 类型从 client::Config 改为 core::Config（字段结构变化：Arc 封装、字段名 `req_timeout`/`header`）。用户需按 CHANGELOG 迁移。
- **core**：`config.rs` 扩展（吸收功能 + `ConfigInner` 新字段）。
- **client**：`config.rs` 大幅缩减；`Client` 构造衔接 core::Config。
- **examples/docs**：迁移到 core::Config。
- **安全正向影响**：base_url 白名单 SSRF 防护从 client 层上移到 core（HTTP 出口层），所有用 core::Config 的路径都受保护。
