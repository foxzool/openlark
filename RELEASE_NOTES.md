# OpenLark 0.15.0-rc.2 发布说明

**发布日期：** 2026-03-26  
**版本类型：** Release Candidate  
**适用版本：** `0.15.0-rc.2`

## 概览

`0.15` 的核心目标不是继续堆叠 API 数量，而是把 SDK 的对外使用方式收敛成一个清晰、稳定、可解释的模型。

这一版完成了三件关键事情：

- 将 `openlark` 明确为唯一官方入口 crate
- 将根 crate feature 收敛为面向用户理解的业务能力模型
- 将 README、示例和公开导出统一到同一套使用路径

这意味着新用户不再需要先区分 “到底应该依赖 `openlark` 还是 `openlark-client`”，也不需要理解 `client`、`protocol` 这类内部实现细节才能开始接入。

## 这版最重要的变化

### 1. `openlark` 成为唯一官方入口

普通用户现在应直接依赖根 crate：

```toml
[dependencies]
openlark = "0.15.0-rc.2"
```

根 crate 现在直接导出以下高频入口：

- `Client`
- `ClientBuilder`
- `Config`
- `CoreConfig`
- `RequestOption`
- `Result`
- `CoreError`

同时，业务模块也统一从根 crate 暴露：

- `open_lark::auth`
- `open_lark::communication`
- `open_lark::docs`
- `open_lark::workflow`
- `open_lark::security`
- 以及其它业务域模块

### 2. `openlark-client` 降级为高级入口

`openlark-client` 仍然保留，但它不再是普通用户的默认入口。

它现在的定位是：

- 给高级用户直接复用统一 `Client` 实现
- 给需要更细粒度控制客户端层 feature 的场景使用
- 作为根 crate 的底层客户端实现层

### 3. feature 模型按用户视角重构

根 crate 不再将 `client`、`protocol` 作为公开 feature 暴露。

对外建议只关注三类 feature：

- 业务 feature：`auth`、`communication`、`docs`、`security`、`hr`、`workflow`、`meeting`、`ai`、`cardkit`、`webhook` 等
- 技术 feature：`websocket`、`otel`
- 组合 feature：`essential`、`enterprise`、`full`

当前组合建议如下：

```toml
[dependencies]
openlark = "0.15.0-rc.2"                                # 默认: auth
openlark = { version = "0.15.0-rc.2", features = ["essential"] }
openlark = { version = "0.15.0-rc.2", features = ["enterprise"] }
openlark = { version = "0.15.0-rc.2", features = ["full"] }
```

## 推荐安装方式

### 最小接入

```toml
[dependencies]
openlark = "0.15.0-rc.2"
```

默认提供统一入口和认证能力，适合先完成应用配置、鉴权和最小化接入。

### 推荐业务开发

```toml
[dependencies]
openlark = { version = "0.15.0-rc.2", features = ["essential"] }
```

`essential` 包含：

- `auth`
- `communication`
- `docs`

### 企业级组合

```toml
[dependencies]
openlark = { version = "0.15.0-rc.2", features = ["enterprise"] }
```

`enterprise` 包含：

- `essential`
- `security`
- `hr`
- `workflow`

### 全量组合

```toml
[dependencies]
openlark = { version = "0.15.0-rc.2", features = ["full"] }
```

## 示例入口统一

主 README 和根 examples 现在统一使用 `openlark` 根路径。

例如：

```rust
use open_lark::prelude::*;
use open_lark::communication::endpoints::IM_V1_MESSAGES;

let client = Client::builder()
    .app_id("your_app_id")
    .app_secret("your_app_secret")
    .build()?;
```

工作流示例也统一到了根 crate，而不是直接要求用户引入 `openlark-workflow`。

## 升级说明

如果你之前直接使用根 crate，并且把它当成“功能聚合层”，这次升级主要需要注意两点：

1. 不再依赖根 crate 的 `client` / `protocol` 这类实现细节 feature
2. README 和示例应切换到 `openlark` 根入口路径

如果你之前直接依赖 `openlark-client`，代码仍可继续使用，但文档推荐路径已经调整为 `openlark`。

## 兼容性与验证

本次 RC 已完成以下发布前验证：

- `cargo check -p openlark`
- `cargo check -p openlark --no-default-features --features docs`
- `cargo check -p openlark --example simple_api_call --features "auth,communication"`
- `cargo check --workspace --all-features`
- `cargo test --workspace --all-features`
- `cargo doc --workspace --all-features --no-deps`

另外，`workspace.lints` 已经真正落到所有成员 crate。当前仍存在一批历史 `missing_docs` 告警，RC 阶段仍以功能正确性、发布链路可通过和公开入口收敛为优先；与此同时，本次已补齐 rustdoc 裸链接格式，确保发布流程中的 `cargo doc` 严格校验不再被 `bare_urls` 阻断。

当前覆盖率口径统一为：**最新工作区覆盖率约 ~47%，CI/main 分支发布门禁为 40%**。这意味着 RC.2 以“可稳定发布”为首要目标，后续正式版继续提升覆盖率基线。

## 已知限制

- 仍有部分公开类型缺少完整 rustdoc，主要表现为 `missing_docs` 存量告警。
- 覆盖率虽已超过当前 CI 基线，但尚未达到更激进的高覆盖率目标。
- WebSocket 与部分高级业务模块的示例仍会在后续版本继续补充。

## 对用户意味着什么

这版最直接的价值是：

- 安装路径更清晰：默认只需要记住 `openlark`
- feature 更可理解：表达的是“我要什么业务能力”，不是“我要内部实现层”
- 示例更一致：README、examples、crate 导出不再混用多套入口
- 后续版本更稳：根 crate 已经具备长期作为官方门面的结构基础

## 后续计划

`0.15` 正式版后，接下来的重点会放在两类工作：

- 继续补齐公开 API 文档，消化这次 lint 落地后暴露的 `missing_docs`
- 在统一入口稳定的前提下，继续推进业务模块覆盖率和示例质量
