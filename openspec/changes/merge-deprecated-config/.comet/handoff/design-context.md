# Comet Design Handoff

- Change: merge-deprecated-config
- Phase: design
- Mode: compact
- Context hash: 39d13b8d395c65567aa674d9f3d759ea22f5b8d4a96805c5f5fd6af12a3bb230

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/merge-deprecated-config/proposal.md

- Source: openspec/changes/merge-deprecated-config/proposal.md
- Lines: 1-29
- SHA256: 4205973e9769734bcd5e67fcf82d59a2cf8f09d7cb9d38235bdf9132ce755e1c

```md
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
```

## openspec/changes/merge-deprecated-config/design.md

- Source: openspec/changes/merge-deprecated-config/design.md
- Lines: 1-58
- SHA256: 359bc3a8043d0b5cebdf59d958dd271e9de949893c546c67bb19a8bd8f2def35

```md
## 背景

两个 Config 并存：

| | core::config::Config | client::Config |
|---|---|---|
| 结构 | `Arc<ConfigInner>` 零拷贝 | 普通 struct（pub 字段） |
| 状态 | ✅ 业务 crate 在用 | ⚠️ deprecated 0.17.0 |
| from_env | ❌ | ✅ |
| validate + 白名单 | ❌ | ✅（SSRF 防护） |
| allow_custom_base_url | ❌ | ✅ |
| ConfigSummary | ❌ | ✅ |
| 超时字段 | `req_timeout: Option<Duration>` | `timeout: Duration` |
| headers 字段 | `header` | `headers` |

根 crate `src/lib.rs:31` re-export 的是 deprecated 的 client::Config。

## 高层决策（open 阶段已与用户确认）

1. **core 全吸收**：from_env / validate / base_url 白名单 / allow_custom_base_url / ConfigSummary 全部迁移到 core::Config，成为唯一丰富的 Config。client::Config 移除。
2. **v0.18 移除 client::Config**（breaking）：已 deprecated 一个版本，直接移除。根 crate re-export 改 core::Config。
3. **全量范围**：core + client + 根 crate + examples + 文档/CHANGELOG 一次性闭环。

## 方案大纲

```
core::config::Config（吸收后）
├── ConfigInner += allow_custom_base_url: bool
├── from_env() / load_from_env()       ← 从 client 迁移，适配 Arc 封装
├── validate() + is_known_base_url()   ← SSRF 防护上移到 core
├── ConfigSummary + summary()          ← 从 client 迁移
├── builder() += allow_custom_base_url()
└── （保留）Arc 零拷贝 / with_token_provider / accessors

client crate
├── 移除 config.rs 的 Config/ConfigBuilder/ConfigSummary
├── Client::new(Config) / builder() 衔接 core::Config
└── CoreConfig 别名可保留或移除

根 crate src/lib.rs:31
└── pub use openlark_core::config::Config;   // 改指向 core
```

## 留待 design 阶段（brainstorming）解决的分叉

以下未知项不在 open 阶段定案，进入 comet-design 深度设计：

1. **`builder().build()` 是否引入 validate**：core 当前 build 不校验，client build 校验。吸收后若 core build 默认校验，是行为变化（影响所有 core::Config 用户，含业务 crate）。需权衡「默认安全」vs「不破坏现有 core 用户」。
2. **字段命名统一**：client `timeout`/`headers`（复数）vs core `req_timeout`/`header`。是否在 core 保留 `timeout()`/`headers()` 别名方便迁移？
3. **`ConfigInner` 加 `allow_custom_base_url`** 对 Arc 克隆、`Debug`、`with_token_provider`（逐字段重建 ConfigInner）的影响——需同步所有构造点。
4. **`Client::builder()` 迁移路径**：client crate 的 Client 构造当前依赖 client::Config，如何衔接 core::Config（含/不含 TokenProvider）。
5. **`OPENLARK_TIMEOUT`（client `Duration`）vs core `req_timeout: Option<Duration>`** 语义对齐（默认值差异：client 默认 30s，core 默认 None/永不超时）。

## 非目标

- 不改 core::Config 的 `Arc<ConfigInner>` 零拷贝架构
- 不改业务 crate 已有的 core::Config 用法（保持源码兼容，除非字段名 breaking）
- 不引入新配置能力（仅迁移已有功能）
```

## openspec/changes/merge-deprecated-config/tasks.md

- Source: openspec/changes/merge-deprecated-config/tasks.md
- Lines: 1-15
- SHA256: 255e228cd904a1848747110bb61a04d3a10751a31c3b2ffe89eb010625a84ac2

```md
## 任务（open 阶段草案，design brainstorming 后定稿）

> 标注 ⚠️ 的任务依赖 design 阶段未定决策（见 design.md「留待 design 阶段解决的分叉」），定稿后可能拆分或调整。

- [ ] T1: core `ConfigInner` 加 `allow_custom_base_url: bool` 字段，同步 `Default`/`Debug`/`with_token_provider`/`build` 等所有构造点
- [ ] T2: core `Config::from_env()` / `load_from_env()` 从 client 迁移，适配 Arc<ConfigInner> 封装；保留 `OPENLARK_*` 环境变量语义 ⚠️（timeout 语义对齐见分叉 5）
- [ ] T3: core `Config::validate()` + `is_known_base_url()` + base_url 白名单 SSRF 防护上移
- [ ] T4: core `ConfigSummary` + `Config::summary()` 从 client 迁移
- [ ] T5: ⚠️ core `ConfigBuilder` 加 `allow_custom_base_url()`；`build()` 是否校验按 design 决策（分叉 1）
- [ ] T6: ⚠️ 字段命名兼容按 design 决策（分叉 2）：core 是否保留 `timeout()`/`headers()` 等迁移别名
- [ ] T7: client 移除 deprecated `Config`/`ConfigBuilder`/`ConfigSummary` 本体（`client/config.rs`）；`Client::new`/`builder` 改用 core::Config（分叉 4）
- [ ] T8: 根 crate `src/lib.rs:31` re-export 改 `openlark_core::config::Config`
- [ ] T9: examples 迁移到 core::Config
- [ ] T10: 文档 + CHANGELOG：breaking 迁移指引 + client::Config → core::Config 字段/方法对应表
- [ ] T11: `cargo test` + `cargo clippy --all-targets` + `cargo check --workspace --all-targets` 全绿
```

## openspec/changes/merge-deprecated-config/specs/config/spec.md

- Source: openspec/changes/merge-deprecated-config/specs/config/spec.md
- Lines: 1-73
- SHA256: 749ff320eecee73f5eb73f952bed227b3eb4e7d529205ab2e3f8476c444aa1b1

```md
# config capability delta — merge-deprecated-config

## ADDED Requirements

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

## REMOVED Requirements

### Requirement: openlark_client::Config（deprecated）

移除 `openlark_client::Config` / `ConfigBuilder` / `ConfigSummary`（deprecated since 0.17.0）。其功能已迁移到 `openlark_core::config::Config`。

#### Scenario: client::Config 不再可用
- WHEN v0.18 代码引用 `openlark_client::Config`
- THEN 编译失败（类型不存在），用户按 CHANGELOG 迁移到 `openlark_core::config::Config`（根 crate `openlark::Config` 自动指向 core）
```

