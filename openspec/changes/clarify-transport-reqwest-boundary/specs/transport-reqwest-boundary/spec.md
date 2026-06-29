## ADDED Requirements

### Requirement: 业务 crate 不直接依赖 reqwest
openlark 业务 crate SHALL 通过 core 的 `openlark_core::http::Transport<T>` 抽象（`Transport::request()`）发起 HTTP 请求，SHALL 不在各自 `Cargo.toml` 中声明 `reqwest` 依赖。仅 `openlark-core`（抽象本体）、`openlark-client`（客户端装配）与 `openlark-webhook`（by-design 性能例外，见下条）三个 crate 允许直接依赖 reqwest。

#### Scenario: 12 个业务 crate 的 Cargo.toml 不含 reqwest
- **WHEN** 在 `analytics / auth / bot / application / communication / mail / hr / docs / helpdesk / platform / user / workflow` 这 12 个 crate 的 `Cargo.toml` 中 grep `reqwest`
- **THEN** 命中数为 0（未用依赖声明全部移除）

#### Scenario: 业务 crate 源码不出现 reqwest 类型
- **WHEN** 在上述 12 个 crate 的 `src/` 中递归 grep `reqwest`
- **THEN** 命中数为 0（业务 crate 经 core `Transport` 发请求，不直接碰 reqwest）

#### Scenario: 允许的三个例外 crate 仍可声明 reqwest
- **WHEN** 检查 `openlark-core`、`openlark-client`、`openlark-webhook` 的 `Cargo.toml`
- **THEN** 三者保留 `reqwest = { workspace = true }`（core=抽象本体，client=装配，webhook=by-design 例外）

### Requirement: Transport 边界显式文档化
ARCHITECTURE.md SHALL 明确记录「`openlark_core::http::Transport<T>` 是 HTTP 边界：业务 crate 经 `Transport::request()` 发请求、不直接依赖 reqwest 类型」这一架构约定。

#### Scenario: ARCHITECTURE.md 含边界约定
- **WHEN** 在 `ARCHITECTURE.md` 中 grep `Transport` 与边界相关表述
- **THEN** 能定位到明确说明业务 crate 经 core `Transport` 发请求、不直接依赖 reqwest 的段落

### Requirement: webhook 直接 reqwest 作为 by-design 例外文档化
`openlark-webhook` 直接使用 `reqwest::Client`（进程级共享、连接池复用，见 GitHub issue #214）SHALL 作为 by-design 例外在 ARCHITECTURE.md 与 webhook crate 文档注释中显式记录，不被视为分层泄漏。

#### Scenario: ARCHITECTURE.md 记录 webhook 例外
- **WHEN** 在 `ARCHITECTURE.md` 中 grep `webhook`
- **THEN** 能定位到说明 webhook 直接 reqwest 是 by-design 例外（连接池复用 / 无鉴权推送器）的段落

#### Scenario: webhook crate 文档注释说明例外
- **WHEN** 在 `crates/openlark-webhook/src/robot/v1/` 中检查 `shared_client`/`client` 的文档注释
- **THEN** 注释显式说明「直接 reqwest 是 by-design 边界例外」并指向架构约定

### Requirement: Transport 边界由守卫脚本机器检验
「业务 crate 不直接依赖 reqwest」这一边界 SHALL 由 `tools/check_reqwest_boundary.sh` 守卫脚本机器检验（匹配 `check_no_dead_code_allows.sh` 先例），并接入 justfile 与 CI lint job，防止未来 PR 重新引入违规依赖。白名单仅 `openlark-core`、`openlark-client`、`openlark-webhook`。

#### Scenario: 守卫脚本存在
- **WHEN** 检查 `tools/check_reqwest_boundary.sh` 是否存在且可执行
- **THEN** 文件存在、含 `set -euo pipefail`、白名单含 core/client/webhook

#### Scenario: 清理后守卫通过
- **WHEN** 12 个业务 crate 的 Cargo.toml 移除 reqwest 后运行 `bash tools/check_reqwest_boundary.sh`
- **THEN** exit 0（业务 crate 无 reqwest 直接依赖）

#### Scenario: 守卫能抓回归
- **WHEN** 任一业务 crate 的 Cargo.toml 重新声明 `reqwest` 后运行守卫
- **THEN** exit 1 并列出违规的 Cargo.toml 路径（防止边界被未来 PR 破坏）

#### Scenario: 守卫接入 CI 与 justfile
- **WHEN** 检查 `justfile` 与 `.github/workflows/ci.yml`
- **THEN** 分别存在 `reqwest-boundary` recipe 与 CI lint job 的一步调用 `tools/check_reqwest_boundary.sh`

### Requirement: ARCHITECTURE.md 中间件层标注为 future
ARCHITECTURE.md 中「Transport 中间件 / 熔断 / 智能重试中间件」相关设计 SHALL 显式标注为「规划中 / future change、当前未实现」，与实际实现（RetryPolicy 配置模式、无中间件链）对齐，消除文档与现实的偏差。

#### Scenario: 中间件草案标注未实现状态
- **WHEN** 在 `ARCHITECTURE.md` 中定位中间件/熔断/重试中间件相关段落
- **THEN** 段落显式标注为规划中 / future / 当前未实现（不再以已实现口吻描述）

### Requirement: 清理不破坏构建、lint 与测试
本次移除 12 个未用 reqwest 依赖 SHALL 不导致 workspace 构建、clippy 或测试失败，SHALL 不改变任何公开 API。

#### Scenario: 全 feature 构建通过
- **WHEN** 运行 `cargo build --workspace --all-features`
- **THEN** exit 0（删依赖后全量构建成功）

#### Scenario: 三组 feature clippy 通过
- **WHEN** 运行 `cargo clippy --workspace --all-targets` 分别以 default、`--all-features`、`--no-default-features` + `-D warnings`
- **THEN** 三组均 exit 0

#### Scenario: 测试通过
- **WHEN** 运行 `cargo test --workspace`
- **THEN** 全部通过（0 failed；本次为纯依赖声明 + 文档变更，无行为影响）
