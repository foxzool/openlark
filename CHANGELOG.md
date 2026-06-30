# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- **Lint 清理**：移除全 workspace 392 处 `#[allow(dead_code)]`（376 处 cruft 删除 + 16 个
  不完整脚手架的死 `config` 字段改名为 `_config` + reserved 注释；跨 platform/ai/analytics/
  user/helpdesk/docs/application）。dead_code lint 信号重新生效。`_config` 均为私有字段，
  不影响公开 API。补全访问器/execute 的工作拆至 #274 / #275 / #276。
- **CI 防复发**：`lint` job 新增 `tools/check_no_dead_code_allows.sh` 检查，禁止非测试代码
  引入 `#[allow(dead_code)]`（本地 `just no-dead-code-allows`）；`#[expect(dead_code)]` 为
  受控的预期死代码豁免。闭环 #267。
- **Transport 边界 hygiene**：移除 12 个业务 crate（analytics/auth/bot/application/
  communication/mail/hr/docs/helpdesk/platform/user/workflow）未用的 `reqwest` 依赖声明，
  并清除这些 crate 的 `[package.metadata.cargo-machete] ignored` 列表里对应的 `"reqwest"`
  项（此前用 ignore 列表承认债务而非删除，是 `cargo machete` 假阴性根因）。
  业务 crate 经 core `Transport<T>` 发请求、源码 0 处直接使用 reqwest（#270 实证）。
  保留例外：`openlark-core`（抽象本体）/ `openlark-client`（装配 + websocket）/ `openlark-webhook`
  （by-design 连接池复用例外，见 ARCHITECTURE.md「Transport HTTP 边界」）。新增
  `tools/check_reqwest_boundary.sh` 守卫并接入 just 与 CI lint job 防回归。
  **非 breaking**：不改公开 API（业务 crate 无 re-export reqwest）。

### Breaking Changes

- **platform 小批请求类型统一 `RequestBuilder` 后缀**（#271 platform 批 1，软 breaking）：
  openlark-platform 的 trust_party/mdm/tenant/spark 子系统 12 个请求 builder `XxxBuilder`
  重命名为 `XxxRequestBuilder`（含 `UserAuthDataRelationBind/Unbind`、`Collaboration*`、
  `CountryRegion*`、`DirectoryUserIdConvert`、`TenantQuery`、`AssignInfoListQuery`、
  `VisibleOrganization`），旧名作 `#[deprecated]` type alias 保留至 v1.0。

- **application+docs 请求类型统一 `RequestBuilder` 后缀**（#271 批次，软 breaking）：
  openlark-application 的 3 个（`AccessDataSearchBlock`/`AccessDataSearchCustom`/`AccessDataSearchWorkplace`）
  与 openlark-docs 的 1 个（`PatchFormFieldQuestion`）请求 builder `XxxBuilder` 重命名为
  `XxxRequestBuilder`，旧名作 `#[deprecated]` type alias 保留至 v1.0。body 模型不动；
  `RecordFieldsBuilder`（真 builder）不动。迁移：`XxxBuilder` → `XxxRequestBuilder`。

- **auth 请求类型统一 `RequestBuilder` 后缀**（#271 pilot，软 breaking）：openlark-auth 的
  12 个请求类型 builder `XxxBuilder` 重命名为规范 `XxxRequestBuilder`，旧名作 `#[deprecated]`
  type alias 保留至 v1.0（调用方用旧名仍可编译，仅 deprecation warning）。涉及
  `AppAccessToken`/`AppAccessTokenInternal`/`AppTicketResend`/`Authorization`/`IdentityCreate`/
  `OidcAccessToken`/`OidcRefreshAccessToken`/`RefreshUserAccessTokenV1`/`TenantAccessToken`/
  `UserAccessTokenV1`/`UserInfo`/`VerificationGet` 各 `Builder` → `RequestBuilder`；
  `TenantAccessTokenInternalRequestBuilder` 已是目标形式（不动）。body 模型（`XxxRequest`）不动。
  `AuthorizationUrlBuilder`（URL builder）不动。迁移：`XxxBuilder` → `XxxRequestBuilder`。
  方向说明：原拟「→Request」，实证发现 5/13 撞 body 模型名，改「→RequestBuilder」对齐 helpdesk。
  v1.0 移除 alias。

- **Removed deprecated wiki Params**：移除 `SearchWikiParams` / `ListWikiSpacesParams` /
  `CreateWikiSpaceParams` / `MoveDocsToWikiParams`（deprecated since 0.16.0）→ 用对应
  `XxxRequest` 流式 Builder。无生产用法（仅兼容测试，一并删除）。关联 #268（B）。

- **Removed deprecated im::im 嵌套别名**：移除 `im::im` 旧嵌套路径别名（deprecated since 0.15.0）→ 迁移
  `im::im::v1` / `im::im::v2` → `im::v1` / `im::v2`。关联 #278（F）。

- **Removed deprecated tenant_access_token legacy 链**：移除 `TenantAccessTokenBuilder`
  的 `app_id`/`app_secret`/`app_ticket` 旧链式入口及两步换取逻辑（deprecated legacy
  chain）→ 迁移：先 `AppAccessTokenBuilder` 取 `app_access_token`，再
  `TenantAccessTokenBuilder::new(config).app_access_token(..).tenant_key(..)`。关联 #278。

- **Removed docs deprecated 方法**：移除 `RecordFieldValue::to_value()`（deprecated since 0.15.0，→ 直接用 `RecordFieldValue` 类型）与 `impl_required_builder!` 宏生成的 `new()`（deprecated since 0.5.0，→ 用 `builder()`）。均零调用/dead。关联 #278（D+C 子集）。

- **Removed deprecated 兼容访问器**：移除 `Hr` 的 8 个 service 访问器方法
  （`attendance()`/`corehr()`/`compensation()`/`payroll()`/`performance()`/`okr()`/`hire()`/`ehr()`）
  与 `SearchV2` 的 `query()`/`user()` 未接线存根（deprecated since 0.15.0）。迁移：

  | 旧（移除） | 新 |
  |---|---|
  | `hr.attendance()` / `corehr()` / ... | `hr.attendance` / `hr.corehr` / ...（字段访问） |
  | `search_v2.query()` | 用 `doc_wiki` / `schema` / `app` / `message` surface |
  | `search_v2.user()` | 无 surface（user-search 未实现）；`UserSearchApi` 经完整路径可达，`execute()` 显式返回未接线错误 |

  `QueryApi`/`UserSearchApi` 类型保留（仅移除便捷存根访问器）。关联 #268。

- **Removed** `openlark_client::Config` / `ConfigBuilder` / `ConfigSummary` (deprecated
  since 0.17.0). All functionality is merged into `openlark_core::config::Config`. The
  root crate `openlark::Config` now re-exports `openlark_core::config::Config` directly.
- **`Client::with_config(client::Config)`** removed — use `Client::with_core_config(core::Config)`
  or `Client::builder()`.
- **`From<client::Config> for Result<Client>`** removed.
- **WebSocket** `LarkWsClient::open` now takes `Arc<openlark_core::config::Config>`
  (was `Arc<openlark_client::Config>`).

### Migration: `client::Config` → `core::Config`

| v0.17 (`openlark_client::Config`) | v0.18 (`openlark_core::config::Config`) |
|---|---|
| `timeout: Duration` (default 30s) | `req_timeout: Option<Duration>` (default `None` = never timeout) |
| `headers: HashMap` | `header: HashMap` (singular) |
| `Config::builder().build()` → `Result<Config>` (validates) | `Config::builder().build()` → `Config` (no validation); call `.validate()` explicitly |
| `Config::from_env()` | `Config::from_env()` (now on core; same `OPENLARK_*` vars) |
| `Client::with_config(cfg)` | `Client::with_core_config(cfg)` |
| `config.app_id` (public field) | `config.app_id()` (accessor; `ConfigInner` fields are `pub(crate)`) |
| base_url whitelist SSRF (client-only) | preserved via `Config::validate()` + `allow_custom_base_url` |

Set `.allow_custom_base_url(true)` on the builder to use a non-whitelisted base_url
(known domains: `*.feishu.cn`, `*.larksuite.com`, `*.larkoffice.com`).

### Added

- `openlark_core::config::Config::validate()` + `is_known_base_url()` — base_url whitelist
  SSRF protection migrated from client (previously client-only).
- `openlark_core::config::Config::from_env()` / `load_from_env()` — env-var loading migrated
  from client; `OPENLARK_TIMEOUT` (seconds) now maps to `req_timeout(Some(Duration))`.
- `openlark_core::config::ConfigSummary` + `Config::summary()` — redacts `app_secret`.
- `openlark_core::config::ConfigInner.allow_custom_base_url` field + builder method.

### Fixed

- **fix(platform)**: 移除 `openlark-platform` 四个 service（Admin/AppEngine/Directory/Spark）
  facade 与 intermediate 层多余的 `#[cfg(feature = "v1")]` 门控。此前 `default`/`full`
  feature 下 service 启用却暴露空壳 facade（四个 service 的全部 v1 API 实现被排除在标准构建外）。
  移除后 "service 启用 = API 可达"，与 hr/communication/meeting 一致。行为补全，非 breaking：
  仅让原本不可达的公开 API 变为可达，不移除任何符号。`v1` feature 保留（测试依赖）。

### Security

- **升级 anyhow 1.0.102 → 1.0.103**（修复 RUSTSEC-2026-0190）：1.0.102 的
  `Error::downcast_mut()` 在 `Error::context` 后调用时违反借用规则（UB）。patch 版本升级，
  无 breaking。CI security-audit（cargo-deny）恢复绿。

## [0.17.0] - 2026-05-30

### Breaking Changes

- **Removed** all v0.15.0 deprecated crate re-exports (`open_lark::openlark_client`,
  `open_lark::openlark_core`, `open_lark::openlark_auth`, etc.)
- **Removed** all v0.15.0 deprecated `*Client` type aliases from root crate
  (`AuthClient`, `DocsClient`, `HrClient`, etc.). Use `client.auth`, `client.docs`,
  `client.hr` field access instead.
- **Cleaned** root `prelude` — no longer exports deprecated `*Client` aliases
- **Cleaned** root and `openlark-client` preludes — no longer export deprecated
  `openlark_client::Config`; use `CoreConfig` or `Client::builder()` instead.
- **`Client.config()` now returns `&openlark_core::config::Config`** (was `&openlark_client::Config`).
  Access fields via methods: `client.config().app_id()`, `client.config().base_url()`, etc.
- **Migration**: Replace `use open_lark::AuthClient` → access via `client.auth` field
  or use `open_lark::auth` module namespace directly. Replace `client.config().app_id`
  → `client.config().app_id()` (method call, not field access)

### Deprecated

- `openlark_client::Config` — planned for removal after the migration window.
  Use `Client::builder()` or `openlark_core::config::Config` directly.

### Added

- `SecurityClient` struct in `openlark-security` — proper wrapper with `Deref`
  to `SecurityServices` (replaces `Arc<SecurityServices>` alias)
- `XxxClient` type aliases in all business crates for consistent naming:
  `WorkflowClient`, `PlatformClient`, `ApplicationClient`, `HelpdeskClient`,
  `MailClient`, `AnalyticsClient`, `UserClient`
- `[package.metadata.docs.rs]` configuration for complete documentation generation
- `docs/CLIENT_NAMING_CONVENTION.md` — naming convention documentation

### Changed

- `openlark-core` no longer enables `testing` feature by default. Crates using
  `openlark_core::testing` in tests must add `openlark-core = { features = ["testing"] }`
  to their `[dev-dependencies]`.
- All business crate `XxxClient` types now exported from source crates instead of
  defined as type aliases in `openlark-client`

### Removed

- `#![allow(async_fn_in_trait)]` from `openlark-client` (MSRV 1.88 no longer needs it)

### Compatibility

#### Typed APIs

#### Helpers & Convenience Methods

#### Breaking Changes

#### Deprecations

#### Migration Notes

### Added

### Changed

### Fixed

## [0.16.1] - 2026-05-20

### Added

- **feat(api)**: 同步 70 个新增飞书 API catalog 条目，并补齐 application v7、docs drive/minutes、HR corehr v2、IM reaction、mail v1、meeting v1、spark v1 等 typed SDK 模块。
- **coverage(api)**: 将 application、communication、docs、hr、mail、meeting、platform 等相关 crate 的 strict API 覆盖率恢复到 0 missing APIs。

### Changed

- **build(deps)**: 放宽 `uuid` 与 `serde_with` 的精确版本锁定，降低下游依赖解析冲突风险。

## [0.16.0] - 2026-05-10

### 🔄 变更

- **style(fmt)**: 统一代码格式化，修复多个文件的格式问题
- **docs(docPath)**: 为 260+ 个 API 文件补充 docPath 官方文档链接
- **refactor(api)**: 删除 explorer/permission v2 的函数式 API，统一使用 Builder 模式
- **refactor(api)**: 统一 platform/helpdesk 40 个文件的 execute() 委托模式，消除代码重复
- **refactor(validate)**: 统一 44 个文件的必填字段校验，使用 validate_required! 宏替换手工校验
- **refactor(types)**: 替换 calendar v4 的 serde_json::Value 为强类型结构体
- **fix(exports)**: 补充 5 个 mod.rs 文件的模型显式导出
- **fix(url)**: 修复 exchange_binding/get.rs 的 API 端点路径拼写错误
- **ci**: 修复 clippy 警告和文档注释缺失
- **build(deps)**: 升级安全相关依赖（tokio-tungstenite、url、reqwest）
- **build(rust)**: 对齐 Rust 2024 / MSRV 1.88

### 🐛 修复

- **fix(ci)**: 修复 CI 持续失败问题（clippy 警告、格式问题）
- **fix(security)**: 添加 max_response_size / ResponseTooLarge HTTP 与 WebSocket 响应大小限制
- **fix(security)**: Token/PII 日志脱敏
- **fix(security)**: path 参数 percent-encoding 安全修复
- **fix(code)**: 生产代码与测试代码 unwrap() 清理
- **fix(auth)**: AuthTokenProvider 多租户缓存 key 修复

## [0.15.0] - 2026-04-05

### 🔄 变更

- **release(root)**: 将工作区版本与根 crate 文案切换到 `0.15.0`
- **docs(release)**: 新增 `0.15` 迁移指南、public API 稳定性策略与正式版发布清障清单
- **ci(examples)**: 为根 README 对齐示例与主推 examples 增加编译校验入口
- **build(metadata)**: 修正 crates.io 元数据中的仓库与文档链接

### 🐛 修复

- **fix(release)**: 修复 GitHub Release 对 RC tag 一律标为正式版的问题
- **fix(docs)**: 修复多个 crate README 中错误的 crate 名、仓库链接和过期版本示例

## [0.15.0-rc.2] - 2026-03-26

### 🔄 变更

- **refactor(root)**: 将 `openlark` 收敛为唯一官方入口 crate，直接导出 `Client`、`ClientBuilder`、`Config`、`RequestOption`、`CoreError` 等高频类型
- **refactor(features)**: 重构根 crate feature 模型，移除面向用户的 `client`/`protocol` 心智，统一为业务 feature、技术 feature 与组合 feature
- **refactor(client)**: 将 `openlark-client` 明确为高级入口，不再作为普通用户的默认接入方式
- **docs(examples)**: 统一 README 和主 examples 到 `openlark` 根入口，修复 `workflow` 示例门槛与实际依赖不一致问题
- **build(lints)**: 将 `workspace.lints` 真正落到各成员 crate，统一工作区 lint 配置

## [0.15.0-rc.1] - 2026-03-17

### ✨ 新增功能

- **feat(webhook)**: 集成 openlark-webhook 模块到工作空间（8 个 API）
  - 自定义机器人、Webhook 事件处理
- **feat(hr)**: 实现 462 个 API (Wave 1-5)，涵盖招聘、CoreHR、考勤、薪酬等模块，总计 562 个 API
- **feat(workflow)**: 完成 workflow 模块 100% API 覆盖（117 个 API）
  - TASK v1 剩余 28 个 API、TASK v2 剩余 24 个 API
  - APPROVAL v4 16 个 API
  - BOARD 模块命名规范修复
- **feat(platform)**: 完成 openlark-platform Transport API 迁移（102 个 API）
- **feat(ai)**: 完成 openlark-ai 模块 27 个 API 实现
- **feat**: 实现缺失的 bizTag API（100% 覆盖）
- **feat(examples)**: 新增长连接 WebSocket Echo 示例并补充测试
- **feat(core)**: 新增测试基础设施模块（testing）
  - `test_runtime()` - 安全的测试运行时
  - `assert_res_ok!`, `assert_err_contains!` 等断言宏
- **feat(client)**: 新增 LazyService 延迟初始化工具
- **docs**: 添加 AGENTS.md 项目知识库

### 🔄 变更

- **refactor(docs)**: 简化 API 入口设计，删除 Service 层，统一 Request 模式
- **refactor(docs)**: 将 glob 重导出转换为显式导出（258 → 7 处）
- **perf(docs)**: 优化 Config 传递，使用 `Arc<Config>` 替代 `Config`
- **refactor(meeting)**: 统一 Request 模式，删除冗余 RequestBuilder
- **refactor(hr)**: 统一架构并添加 feature gating 支持
- **refactor(core)**: 为 testing 模块添加 feature gate
- **refactor(core)**: 清理未使用的空 features，将测试依赖移动到 dev-dependencies
- **refactor**: 实现显式导出系统，消除 251+ 个通配符导出
- **style(security)**: 修复命名规范异常，替换硬编码 URL，统一代码风格

### 🐛 修复

- **fix(core)**: 统一 validate_required 语义，支持字符串 trim
- **fix(docs)**: 修复 sheets_v2 数据读取 API 路径
- **fix(docs)**: 修复 Arc<Config> 类型不匹配错误
- **fix(docs)**: 修复 explorer v2 模块编译错误和导出问题
- **fix(hr)**: 添加 CoreHR 缺失的 API 端点定义并修复语法错误
- **fix**: 修复 no-default-features 编译错误
- **fix**: 修复多个 crate 的代码风格和导出
- **fix(examples)**: 修复 examples 编译错误
- **fix(ci)**: 修复 Coverage 工作流覆盖率收集问题（多次迭代修复）

### 🧪 测试

- 大幅提升测试覆盖率至 ~47%
- 为所有主要模块添加测试：docs、workflow、platform、cardkit、hr、meeting、auth、core
- 为 workflow v1/v2 模块添加完整测试套件
- 迁移 44 个测试文件到新框架，消除 144 处 `Runtime::new().unwrap()`

## [0.15.1] - 2025-11-20

### 🔄 架构优化 - openlark-docs 链式调用支持与 API 覆盖率更新

#### ✅ 核心能力增强

- **🔗 openlark-docs 完整链式调用支持** - 通过 openlark-client 提供流畅的链式调用体验
  - **DocsClient 集成** - Client 结构体包含 DocsClient 字段，启用 `docs` feature 即可使用
  - **完整链式调用路径** - `client.docs.ccm.drive.v1()`, `client.docs.ccm.sheets.v3()`, `client.docs.base.bitable()` 等
  - **类型安全** - 编译时验证所有链式调用路径

#### 📊 openlark-docs API 覆盖率验证

- **✅ 254 个 API，100% 完成** - 全面验证 openlark-docs 的 API 实现情况
  - **✅ 零未完成标记** - 无 TODO/FIXME/unimplemented! 标记
  - **✅ 代码质量优异** - 所有实现文件经过验证，零编译警告

#### 📈 模块实现状态详情

| 模块 | API 数量 | 已实现 | 未实现 | 完成率 |
|------|---------|--------|--------|--------|
| **CCM** | 174 | 174 | 0 | 100% |
| **BASE** | 49 | 49 | 0 | 100% |
| **BAIKE** | 27 | 27 | 0 | 100% |
| **MINUTES** | 4 | 4 | 0 | 100% |

#### 🏗️ 架构优化

- **链式调用架构** - DocsClient 通过字段链式访问所有子服务
  - **模块化设计** - ccm, base, baike, minutes 清晰的功能分层
  - **类型安全接口** - 提供服务访问器方法，如 `client.docs.ccm.drive.v1()`
  - **配置透传** - 支持从 DocsClient 获取底层服务

#### 📋 文档与示例

- **链式调用文档** - 详细的链式调用路径和使用示例
- **API 验证报告** - `docs/API_COVERAGE_REPORT.md` 提供详细的实现状态
- **模块级文档** - 每个 AGENTS.md 提供模块特定的使用指南

#### 🔧 技术债务清理

- **零未完成标记** - 扫描所有 API 文件，无 TODO/FIXME/unimplemented! 标记
- **代码质量优秀** - 所有实现文件通过编译和 lint 检查
- **架构清晰** - 严格的模块划分和命名规范

#### 📊 性能与质量

- **编译性能** - 默认功能 0.6s，全功能验证 0.37s
- **零警告构建** - 所有模块通过 clippy 检查
- **测试覆盖** - 核心功能完整测试覆盖
