---
name: openlark-naming
description: OpenLark Rust SDK 命名与对外 API 表达规范（Client/Service/Resource/Request/Builder）。用于新增/重构公开类型、设计 meta 调用链、调整模块导出与 prelude、或排查 *Service 同名/语义错配/V{N}Service 版本层错位、*Resource 与 *Service 同类型两名、以及 Client/Service/Resource 调用风格不一致的问题。触发关键词：命名规范、Client vs Service、Resource、重命名、V1Service 版本层、meta 调用链、公开 API
argument-hint: "[module|type-name|path]"
allowed-tools: Bash, Read, Grep, Glob, Edit
---

# OpenLark 命名规范（Client / Service / Resource / Request / Builder）

## 🧭 技能路由指南

**本技能适用场景：**
- 你在设计/调整对外公开类型名（`pub struct` / `pub type` / re-export / prelude）
- 你在设计 `client.xxx.v1.yyy.zzz` 这类 **meta 调用链**
- 你发现 `*Service` 同名、语义混乱、调用方式不一致，想系统性收敛

**其他技能：**
- 项目级规范体检（架构/API/导出/校验一体）→ `Skill(openlark-code-standards)`
- 设计审查（更广）→ `Skill(openlark-design-review)`
- 新增/重构单个 API（落盘/端点/Builder 模板）→ `Skill(openlark-api)`

### 关键词触发映射

- 命名规范、Client vs Service、Resource、重命名、meta 调用链、公开 API → `openlark-naming`
- 代码规范、规范检查、风格一致性、体检 → `openlark-code-standards`
- 架构设计、public API、收敛方案、feature gating、兼容策略 → `openlark-design-review`
- 新增 API、重构 API、Builder、Request/Response、mod.rs 导出 → `openlark-api`
- validate、必填校验、validate_required、空白字符串、校验聚合 → `openlark-validation-style`

### 双向跳转规则

- 若命名问题已扩展为入口/范式收敛问题，转 `openlark-design-review`。
- 若命名调整涉及具体 API 文件实现与导出补齐，转 `openlark-api`。
- 若需要先确认全仓规则基线，再做命名调整，先跑 `openlark-code-standards`。

---

## 0) 快速决策：先选类型职责，再命名（必须）

- **顶层入口/门面（面向用户）**：持有 `Config`（或 `Arc<Config>`），组织调用链与透传配置 → `*Client`
- **业务能力集合（可执行）**：对外暴露一组 API，承接/实现通用 trait（如 `Service`、`ExecutableBuilder`）→ `*Service`
- **资源节点/命名空间（只组织层级）**：处在 meta 调用链的中间层，主要做字段分组与 config 透传 → `*Resource`
- **版本层对象**：必须把版本写进类型名 → `*V1Service` / `*V2Service`（或 `*V1Client` 视职责而定）
- **单 endpoint 请求类型**：`*Request` 或 `*RequestBuilder`（同一 crate/模块树二选一并保持一致）

> 约束：**不要用 `*Service` 去命名“仅做层级组织/透传 config 的节点”。**

## 1) `*Client` 命名规则

- 语义：**入口 / 门面 / 组合根**；类型名要让读者知道“从这里开始调用”。
- 典型结构：
  - 持有 `Arc<Config>`
  - 暴露 `pub xxx: XxxResource` / `pub v1: XxxV1Client` 之类字段链
  - 很少直接实现业务方法（除非规模很小且能保持一致）
- 建议放置：`common/chain.rs`（避免被 API 实现校验脚本当成 endpoint 实现文件）

### 两种合法实现（XxxClient 的落盘方式）

每个业务 crate 必须导出一个 `XxxClient` 类型作为主入口，有两种合法实现方式，按 crate 规模/复杂度二选一：

| 方式 | 写法 | 适用 | 现码实例（核实） |
|------|------|------|------------------|
| **A. 单 facade type alias** | `pub type XxxClient = XxxService;`（写在 `lib.rs`） | crate 规模小、内部仅一个 Service、无需多层链式入口 | `crates/openlark-workflow/src/lib.rs:121` `pub type WorkflowClient = WorkflowService;`；同类还有 bot/application/platform/mail/helpdesk/analytics/user 等 |
| **B. 分层调用链 struct** | `pub struct XxxClient { ... }`（写在 `common/chain.rs`，经 `lib.rs` re-export） | crate 含多 bizTag/project、需要 `client.xxx.v1.yyy` 链式入口 | `crates/openlark-docs/src/common/chain.rs` → `lib.rs:125` `DocsClient`；同类有 cardkit/communication/meeting/ai |

> **权威映射表**：各 crate 当前采用 A 还是 B、内部类型与导出名对照，以 `docs/CLIENT_NAMING_CONVENTION.md` 的「当前映射」表为准；新增/切换实现方式时同步更新该表，勿仅凭记忆判断。

## 2) `*Service` 命名规则

- 语义：**能力载体**；需要能回答“这个类型里提供了一组可执行 API/操作”。
- 若引入通用 trait（例如 `openlark_core::trait_system::Service` / `ExecutableBuilder`），优先在 `*Service` 层承接，保证可观测性（service_name/version）一致。

## 3) 版本层命名（强制：避免同名灾难）

- 版本对象必须显式带版本号：`*V1Service` / `*V2Service`（或 `*V1Client`，按下方双轨约定选择）
- 禁止：外层 `DocsService`，内层 `v1::DocsService` 也叫 `DocsService`
  - 会导致 `use` 歧义、re-export 冲突、文档示例难以写清

### 双轨约定（执行 Service 侧 vs chain.rs facade 侧）

同一个版本层在 crate 内会同时出现两个名字，分别承担不同职责，**二者都必须带版本号**且语义不混：

| 侧 | 命名 | 职责 | 典型位置（现码核实） |
|----|------|------|----------------------|
| **执行 Service 侧** | `*V{N}Service` | 能力载体，承接 trait（`Service`/`ExecutableBuilder`），提供一组可执行 API | `crates/openlark-cardkit/src/service.rs:27` 返回 `CardkitV1Service`；`crates/openlark-meeting/src/calendar/service.rs:26` 定义 `CalendarV4Service` |
| **chain.rs facade 侧** | `*V{N}Client` | 门面/链式入口节点，持有 `Arc<Config>` + 暴露资源字段链 | `crates/openlark-cardkit/src/common/chain.rs:37` `pub struct CardkitV1Client` |

- 两层之间由 facade 构造并返回 Service（如 `service.rs` 的 `v1()` 返回 `CardkitV1Service`）
- 命名上务必成对出现：`CardkitV1Client`（facade）↔ `CardkitV1Service`（执行），版本号一致，**不可一侧带版本号、另一侧不带**

## 4) `*Resource` 命名规则（meta 调用链中间层）

- 语义：**资源节点/命名空间**，主要职责是组织层级与透传 config
- 参考（正例）：`openlark-cardkit` 的 `CardResource` / `CardElementResource`
- 反例：把所有中间层都叫 `*Service`，最终变成“同名泛滥 + 读者不知道哪里能 execute”

## 5) `*Request` vs `*RequestBuilder`（风格统一，禁止混用）

在同一个 crate（至少同一业务域目录树）里二选一：

### A. Builder 风格（推荐：可统一 execute_with_options）
- `XxxRequestBuilder`：负责参数收集与构建
- `execute(&XxxService)` / `execute_with_options(&XxxService, ...)`：统一执行入口

### B. Self-contained Request 风格（可行但要全局一致）
- `XxxRequest::new(config)`：请求对象持有 `Config`
- `.execute()` / `.execute_with_options(option)`：无需传 service

> 禁止：同一层级里一部分 API 需要 `execute(&service)`，另一部分是 `.new(config).execute()`，会显著增加使用心智与封装成本。

## 6) 名字必须与路径/模块语义一致（避免“路径-名字”错配）

- 模块叫 `doc`，类型不要叫 `DocsService`
- 模块叫 `permission`，类型不要叫 `DriveService`
- 类型名应能让读者大致推断它在哪个 bizTag/project/version/resource 下（至少不会“指向错误模块”）

## 6.5) 事件/回调类（P2）命名与归属

事件（event）、回调（callback）、webhook 推送这类**被动接收**能力，命名与归属规则与主动调用的业务 API 不同：

- **独立 P2 crate，不进统一门面**：webhook/event 类能力放在独立 crate（`openlark-webhook`），**不注册进 `openlark-client` 的 ServiceRegistry 统一门面**。
  - 现码核实：`openlark-webhook` 是独立 crate，其入口 `WebhookClient` 由 `crates/openlark-webhook/src/lib.rs:60` re-export（`pub use robot::v1::client::WebhookClient;`）；在 `crates/openlark-client/src/` 下搜索 `webhook` 无任何注册项。
- **命名**：沿用 `*Client` 作为该 crate 的对外入口（如 `WebhookClient`），但**不要求**与 `openlark-client::Client` 形成 `client.xxx` 链式段，因为它不属于主动 API 调用链。
- **构造差异**：webhook 类常不需要 `Config`/app_id，构造可能为 `WebhookClient::new()`（无参）或仅传 webhook URL/secret，与业务 `*Client::new(config)` 不同；命名上仍叫 `*Client`，但归属与生命周期独立。
- **判断要点**：新增一个“接收推送/回调”的能力时，优先问“它是否被动触发？”——是，则归独立 P2 crate，不要塞进 `openlark-client`。

## 7) 已治理案例与现存反例

### 7.1 已治理：openlark-docs 的 `DocsService` 重名问题

历史问题：`openlark-docs` 曾出现多处 `DocsService` 同名但语义不同（`ccm::docs` 入口、`ccm::docs::v1` 版本层、`ccm::doc` 模块错配），是本技能最初触发的典型案例。

**治理结论**（已落地，作为同类问题的参考样板）：
- 全 crate 统一以 **`DocsClient` 作为唯一对外入口**，prelude 注释明确记载：
  - `crates/openlark-docs/src/prelude.rs:16` — `// 已移除 Service 的 prelude 导出，统一使用 DocsClient 作为唯一入口`
- `DocsClient` 由分层调用链实现，定义于 `crates/openlark-docs/src/common/chain.rs`，经 `crates/openlark-docs/src/lib.rs:125` re-export（`pub use common::chain::DocsClient;`）
- `*Service` 类型（`CcmService`/`BaseService`/`BitableService` 等）保留为内部执行层，不再进 prelude，仅在需要时通过完整路径访问
- **启示**：当某 crate 的 `*Service` 泛名泛滥、re-export 冲突难收敛时，优先收敛为单一 `*Client` 门面 + 内部 Service 分层，而非逐个改名 `*Service`

> 现码核实：上述 prelude.rs:16 与 lib.rs:125 的 file:line 均有效；旧的 `ccm/docs/mod.rs:8`、`ccm/docs/v1/mod.rs:25`、`ccm/doc/mod.rs:65` 三处引用已全部失效，勿再引用。

### 7.2 现存反例：同一类型同时挂两个名字（Resource + Service）

现码中仍存在“同一结构体同时暴露 `*Resource` 与 `*Service` 两个名字”的兼容残留，是典型的待收敛反例：

- `crates/openlark-cardkit/src/cardkit/cardkit/v1/card/element/mod.rs:47`
  ```rust
  /// 兼容历史命名：card.element 服务
  pub type CardElementService = CardElementResource;
  ```
  - 真实结构体是 `CardElementResource`（mod.rs:42，资源节点语义），却又 `pub type` 出一个 `CardElementService` 别名。
  - 问题：违反 §0/§4 的职责-命名对应（资源节点不应叫 `*Service`），造成“读者不知道哪里能 execute”的同名泛滥；别名仅为兼容历史调用而存在。
  - 收敛方向：评估调用方迁移后移除 `CardElementService` 别名，统一只保留 `CardElementResource`。

> 排查方法：`rg -n "pub type .*Service = .*Resource" crates/` 可一次性找出所有此类“同类型两名”残留。

## 8) 改名 review 清单（提交前逐条过一遍）

- 目录/模块路径是否能从类型名推断（至少到 bizTag/project/version/resource）
- `prelude`/re-export 是否引入同名冲突（尤其是 `*Service` 这种泛名；已治理的 `DocsService` 勿再复现）
- `*Client` / `*Service` / `*Resource` 的职责是否清晰，调用方式是否一致
- 版本层是否统一采用 `*V{N}Service`（或 `*V{N}Client`），避免重复 `*Service`；facade 侧 `*V{N}Client` 与执行侧 `*V{N}Service` 版本号是否成对一致
- 同一类型是否同时挂了 `*Resource` 与 `*Service` 两个名字（`rg "pub type .*Service = .*Resource"` 应无新增）
- 事件/回调类是否误被塞进 `openlark-client` 门面（webhook/event 应在独立 P2 crate）
- **改名后跑 `just lint`**：等价于 `cargo clippy --workspace --all-targets --all-features -- -Dwarnings -A missing_docs`（justfile:14），务必带 `--all-targets` 以覆盖 examples/tests/benches，确保改名未漏掉非 lib 目标
- **移动/新建 example 时补 `[[example]] required-features`**：根 `Cargo.toml` 每个 `[[example]]` 需声明所依赖的 feature（如 `required-features = ["communication", "websocket"]`，见 Cargo.toml:248+）；example 引用的类型若位于 feature-gated 模块，缺声明会导致 `--all-features` 之外编译失败
- **测试文件保持 `//!` 在 `#![cfg(feature)]` 之上**：契约测试/集成测试文件开头顺序固定为「文件级文档注释 `//!` 在前、`#![cfg(feature = "...")]` 属性在后」，再 `use`（正例：`crates/openlark-client/tests/docs_feature_contract.rs:1-6`）；改名时勿颠倒顺序，否则 inner attribute 报错
