## Why

issue #270（2026-06 架构审计 finding）指出「14 个业务 crate 直接依赖 reqwest，存在分层泄漏与配置不一致风险」。但本次 open 阶段已实证核实：**业务 crate 早已通过 core 的 `Transport<T>` 收口、源码层 0 处直接使用 reqwest**（core 62 处、webhook 10 处有意、client 3 处，其余 12 个业务 crate 全部 0 处）。issue 的「分层泄漏」「配置跨 crate 不一致」前提已不成立——真正的遗留是：

1. 12 个业务 crate 的 `Cargo.toml` 仍声明 `reqwest = { workspace = true }` 却从不引用（cargo-machete 因不遍历 `crates/` 给出假阴性），属 hygiene 问题；
2. webhook crate 直接用 reqwest（连接池复用，引用 #214）这一 by-design 例外未显式文档化；
3. ARCHITECTURE.md 的「Transport 中间件/熔断/重试」层是「🚧 规划中」的未实现草案，与 reqwest 依赖问题混为一谈。

本 change 闭环该审计 finding：做廉价且高确定性的 hygiene 清理 + 把已存在且在工作的 `Transport<T>` 边界显式文档化，不引入新抽象（trait / 中间件层推迟为独立 future change）。

## What Changes

- 删除 12 个业务 crate 的未用 `reqwest = { workspace = true }` 依赖声明：`analytics / auth / bot / application / communication / mail / hr / docs / helpdesk / platform / user / workflow`。
- **ARCHITECTURE.md** 新增/强化一节，明确「`openlark_core::http::Transport<T>` 是 HTTP 边界：业务 crate 经 `Transport::request()` 发请求、不直接依赖 reqwest 类型」。
- **ARCHITECTURE.md** 显式记录 webhook 的 by-design 例外（无鉴权推送器，进程级共享 `reqwest::Client` 复用连接池，见 #214）。
- **ARCHITECTURE.md** 将「Transport 中间件/熔断/重试」草案标注为 future change（不删除设计记录，仅澄清当前未实现、不属本次范围）。
- **CHANGELOG.md** 记一笔 v0.18 hygiene 清理（删 12 个未用 reqwest 依赖，非 breaking）。
- **webhook crate 文档注释**：强化「直接 reqwest 是 by-design 例外」说明，指向本边界约定。

## Capabilities

### New Capabilities
- `transport-reqwest-boundary`: openlark 业务 crate SHALL 通过 core 的 `Transport<T>` 抽象发起 HTTP 请求、SHALL 不在 `Cargo.toml` 中直接声明 `reqwest` 依赖（core / client / webhook 三个 crate 为显式例外，其中 webhook 为 by-design 性能优化例外并需文档化）。

### Modified Capabilities
<!-- 无（无既有 transport/http 相关 main spec） -->

## Impact

- **12 个业务 crate 的 `Cargo.toml`**：各删 1 行 `reqwest = { workspace = true }`（及其在 `[dependencies]` / `[dev-dependencies]` 中的归属）。不动任何 `.rs` 源码。
- **ARCHITECTURE.md**：新增边界约定小节 + webhook 例外 + 中间件层 future 标注。
- **CHANGELOG.md**：v0.18 hygiene 条目。
- **crates/openlark-webhook**：仅强化文档注释，不改源码逻辑。
- **破坏性**：无。删除未用依赖声明不改公开 API（业务 crate 0 处引用 reqwest）。理论上若下游依赖某业务 crate 传递出的 reqwest 可用性会受影响，但 workspace dep 的 features 集中管理、且无任何业务 crate re-export reqwest，属不支持的隐式用法；CHANGELOG 记录兜底。
- **非目标**：不把 `Transport<T>` 升级为 trait、不实现中间件/熔断/重试链、不改造 webhook 到 Transport、不动 core `Transport` 实现、不动 client。
