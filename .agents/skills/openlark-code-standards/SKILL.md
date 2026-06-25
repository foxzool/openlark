---
name: openlark-code-standards
description: OpenLark 项目代码规范检查技能。用于快速审查仓库内的架构一致性、API 实现套路、参数校验、命名与导出规范，并输出可执行检查清单与证据路径。Triggers: code review / consistency check / architecture audit / 规范检查 / 风格一致性 / 体检 / 对齐约定。项目锚点见 AGENTS.md#CONVENTIONS 与 AGENTS.md#ANTI-PATTERNS。
argument-hint: "[crate-name|path]"
allowed-tools: Read, Grep, Glob, Bash
---

# OpenLark 代码规范检查（Skill）

## 适用场景

- 用户要求“检查项目代码规范”
- 新增 API 前想确认实现套路是否一致
- 评审 PR 时需要快速验证是否遵循 OpenLark 既有约定
- 发现模块风格漂移，想做一次统一体检

## 目标

输出一份可落地的规范检查结果，包含：

- 规范结论（通过/风险）
- 规则-证据对（每条规则附 `path:line`）
- 风险分级（P0/P1/P2）
- 新 API 最小检查清单（可直接用于 PR Review）

## 检查范围

覆盖全部 `crates/openlark-*`（按 `AGENTS.md#STRUCTURE` 有 18 个业务/基础设施 crate）。

- 重点核审：核心基础设施（`openlark-core`、`openlark-client`、`openlark-protocol`）与高频业务 crate（`openlark-docs`、`openlark-communication`、`openlark-hr`）。
- Client 命名（`XxxClient`）权威映射表见 `docs/CLIENT_NAMING_CONVENTION.md` 的「当前映射」，命名核审以此为准。
- 可按参数缩小为某个 crate 或目录（如 `openlark-mail`、`crates/openlark-hr/src/`）。

## 核心检查项

### 1) API 实现套路一致性

- 是否使用 `Request/Response + Builder` 模式
- 是否提供 `execute()` 与 `execute_with_options(RequestOption)`
- 是否通过 `Transport::request(...)` 发送请求

**🔴 硬规则（grep 命中即 P0 违规）：**
- 业务 crate（除 `openlark-core` 与 **`openlark-webhook`**）**不得出现 `reqwest::Client::new()`**——一旦命中，说明有端点绕过了 `Transport`，必为不一致源头。检查命令：
  ```bash
  rg "reqwest::Client::new" crates/ --type rust -g '!openlark-core/**' -g '!openlark-webhook/**'
  ```
- **白名单说明：`openlark-webhook` 是有意例外**——自定义机器人不是飞书开放平台 API（目标 URL 为用户配置的绝对地址、用 URL 携带签名密钥鉴权、响应体为非标准 `{code,msg}`），不适用 `Transport` 的 `/open-apis/` 基址与 token 注入，故保留独立 reqwest 路径（见 issue #214 调研结论，注释位于 `crates/openlark-webhook/src/robot/v1/send.rs:13-26`）。**勿改 webhook 的 reqwest 用法**。
- **不得手工 `Authorization` 头 / `get_app_token`**（token 由 `Transport` 自动注入）
- **`Service`/`Request` 不得持有 HTTP client 字段**（只持 `Config`）

**🟡 R 语义检查（防止双重嵌套陷阱）：**
- `ApiRequest<R>` 的 `R` 应是响应 `data` 字段的**内容类型**，不是外层包装。
- 可疑信号：某个 `XxxResponse` struct 带 `data: Option<...>` 字段、同时又作为 `ApiRequest<XxxResponse>` 的泛型——大概率双重嵌套（core 已把 `R` 当作 data 内容解析）。
- 详细契约见 `Skill(openlark-api)` 的"🔒 核心契约"。

### 2) 端点定义规范

- 是否使用 **per-crate 类型安全端点枚举**：每个 crate 在 `src/common/api_endpoints.rs` 定义各自的 `<Domain>ApiV1` 枚举，并实现 `pub fn to_url(&self) -> String`（注意 `to_url()` 返回的是相对路径 `String`，形如 `/open-apis/mail/v1/...`，不带基址）。
- 是否避免手写业务 URL（统一走枚举 `to_url()`）
- 命名注意：枚举名是 `<Domain>ApiV1`，`Domain` **通常但并非总是**等于 crate 名——以现况为准：
  - 与 crate 同名：`MailApiV1`（`crates/openlark-mail/src/common/api_endpoints.rs:5`）、`DocsApiV1`（`crates/openlark-docs/src/common/api_endpoints.rs:440`）、`HelpdeskApiV1`、`AppApiV1`（application）。
  - 与 crate 名不同：`VcApiV1`（meeting，`crates/openlark-meeting/src/common/api_endpoints.rs:224`）、`TaskApiV1`/`BoardApiV1`（workflow）、`AdminApiV1`（platform）、`AuthenApiV1`（auth）、HR 多枚举（`AttendanceApiV1`/`HireApiV1`/`OkrApiV1`/...）、Docs 多枚举（`BitableApiV1`/`WikiApiV1`/`DocxApiV1`/`MinutesApiV1`/...）。
- 新增端点时改枚举与 `to_url()` match 分支，不要在 Request/Builder 里手写 URL 字面量。

### 3) 参数校验规范

- 必填校验是否统一用 `openlark_core::validate_required!`
- 字符串是否优先 `trim()` 后再校验
- 列表字段是否校验非空与长度上限（如 `validate_required_list!`）

**🟡 Token 类型与 Config 形态：**
- 应用级接口（文档要求 `tenant_access_token`/`app_access_token`）是否显式 `.with_supported_access_token_types(vec![AccessTokenType::App])`——`ApiRequest` 默认 `[User, Tenant]`，漏设会导致飞书拒绝。
- `Request`/`Service` 是否用 owned `Config`（非 `Arc<Config>`）——新代码与重构以 owned 为准（`openlark-docs` 历史用 `Arc` 属例外）。

**🟡 `RequestOption`（单数）vs `RequestOptions`（复数）切勿混用：**
- `openlark_core::req_option::RequestOption`（**单数**，`crates/openlark-core/src/req_option.rs:7`）——`Transport`/`execute_with_options` 实际接受的类型（token、tenant_key、file_upload/download 等运行时请求控制），业务 crate 调用 `execute_with_options(RequestOption::default())`。
- `openlark_client::types::client::RequestOptions`（**复数**，`crates/openlark-client/src/types/client.rs:180`）——高级客户端的 timeout/retry_count/headers 构造参数，语义不同。
- 二者字段集合完全不同，**不得互相替代或导出混名**；业务 crate 写 Builder 时用单数 `RequestOption`。

### 4) 命名与公开 API 表达

- `Client/Service/Resource/Request/Builder` 命名是否语义清晰
- 对外入口是否统一（避免同义入口并存）
- meta 调用链命名是否与仓库约定一致

### 5) 导出与 feature gating

- `mod.rs` 与 `prelude` 是否完整导出新增 API
- `Cargo.toml` feature 与 `#[cfg(feature = "...")]` 是否对齐
- 是否存在导出但不可编译或不可访问路径

### 6) CI 与测试门控（#228 审核闭环）

> 详细背景见 `docs/CI_TEST_TARGET_COVERAGE.md`（相关 issue：#228 / #246 / #248 / #250 / #251；PR：#247 / #249 / #252 / #253 / #254）。

- **测试/`#[cfg(test)]` 改动后必跑 `just lint`**：`just lint` = `cargo clippy --workspace --all-targets --all-features`（justfile:12-14）。CI 在 **all-features / no-default-features / 各 feature 组合** 三个维度都用 `--all-targets`，因此 `#[cfg(test)]` 类 lint 回归（如 #248 的未用 `use super::*`）会被 CI 直接拦住，不只依赖本地。
- **`tests/*.rs` feature 门控约定**（#251）：文件引用 feature-gated 模块时，文件顶部加 `#![cfg(feature = "...")]`；且 **`//!` 模块文档必须在 `#![cfg]` 之上**（顺序反了会触发 clippy `missing_docs`）。
- **`examples/*.rs` 禁用 `#![cfg]`**（#251）：feature 门控走 `Cargo.toml` 的 `[[example]] required-features`。example 里写 `#![cfg]` 会把文件清空成无 `main`，报 **`E0601`**。
- **event 模块（`openlark-communication`）不违规**：event 在 `tools/api_priority.toml` 标为 **P2**（非 P0 核心业务），按基础设施类对待（与 WebSocket `LarkWsClient` 同类），**有意不经统一 client `declare_client!` re-export**（#228 决策）。审查时勿把它当作「漏 re-export 的违规」上报。

## 输出模板（必须）

1. 结论概览（3-6 条）
2. 规则-证据对（至少 8 条）
   - 规则
   - 证据（`path:line`）
   - 风险等级（P0/P1/P2）
   - 修复建议
3. 新 API 最小检查清单（5-8 条）
4. 建议行动项（按优先级排序）

## 推荐执行顺序

1. 先读 `AGENTS.md` 与目标 crate 的 `AGENTS.md/CLAUDE.md`
2. 再扫规则高频证据：`api_endpoints`、`execute_with_options`、`validate_required`、`prelude`、`mod.rs`
3. 最后输出规则-证据对与整改建议

## 重点提醒

- 结论必须基于代码证据，不做“纯经验判断”
- 证据至少精确到文件路径，推荐精确到 `path:line`
- 不在规范检查中进行大规模重构，先给出可执行清单

## 与其他技能的关系

- 需要审查整体架构与公共 API：`openlark-design-review`
- 只聚焦校验写法统一：`openlark-validation-style`
- 需要新增/重构具体 API：`openlark-api`
- 需要做覆盖率统计：`openlark-api-validation`

## 技能分流决策表

| 场景 | 优先技能 | 何时转交 |
|---|---|---|
| 只想做一次项目规范体检（给出规则-证据对和风险清单） | `openlark-code-standards` | 若发现架构级冲突/公共 API 设计分歧，转 `openlark-design-review` |
| 重点是架构收敛、范式选型、兼容策略（含 breaking 评估） | `openlark-design-review` | 若需要补充全仓规范一致性证据，可回补 `openlark-code-standards` |
| 只处理 `validate()`、`validate_required!`、空白字符串与校验聚合 | `openlark-validation-style` | 若校验问题已扩展到命名/导出/端点体系，转 `openlark-code-standards` |
| 新增或重构某个具体 API 文件（Request/Response/Builder/导出） | `openlark-api` | 若实现前需先做规范体检，先跑 `openlark-code-standards` |
| 关注 API 覆盖率、实现数量、缺失清单 | `openlark-api-validation` | 若覆盖率问题背后是设计不一致，转 `openlark-design-review` |

### 快速判断

- 问题是“这个项目现在规范是否一致” → `openlark-code-standards`
- 问题是“这个设计该怎么收敛” → `openlark-design-review`
- 问题是“这个校验写法到底怎么统一” → `openlark-validation-style`
- 问题是“我现在就要实现某个 API” → `openlark-api`

## 关键词触发映射

| 用户关键词/表述 | 建议技能 |
|---|---|
| 代码规范、规范检查、风格一致性、体检、对齐约定 | `openlark-code-standards` |
| 架构设计、public API、收敛方案、feature gating、兼容策略、breaking change | `openlark-design-review` |
| validate、必填校验、validate_required、空白字符串、校验聚合 | `openlark-validation-style` |
| 新增 API、重构 API、Builder、Request/Response、mod.rs 导出 | `openlark-api` |
| 覆盖率、实现数量、缺失 API、统计、对比 CSV | `openlark-api-validation` |

### 组合关键词优先级

- 同时出现“规范检查 + 覆盖率”时，先用 `openlark-api-validation` 产出缺失清单，再用 `openlark-code-standards` 做规范归因。
- 同时出现“规范检查 + 架构收敛”时，先用 `openlark-code-standards` 做现状证据，再转 `openlark-design-review` 定迁移方案。
- 同时出现“新增 API + 校验统一”时，实现阶段用 `openlark-api`，校验规则判定用 `openlark-validation-style`。
