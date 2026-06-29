# Tasks — clarify-transport-reqwest-boundary

> 闭环 issue #270：删 12 个业务 crate 未用 reqwest 依赖 + 清 cargo-machete ignore 债务 + 文档化 `Transport<T>` 边界 + webhook 例外 + 中间件层 future 标注 + 新增防回归守卫脚本。
> 纯依赖声明 + 文档 + 一个 shell 守卫脚本，不改 `.rs` 业务源码逻辑、不改公开 API（唯一允许改 .rs 的是 webhook doc comment）。
> 详细执行步骤见 plan: `docs/superpowers/plans/2026-06-29-clarify-transport-reqwest-boundary.md`

## 1. 依赖清理

- [x] 1.1 移除 12 个业务 crate 的 `Cargo.toml` 中未用的 `reqwest = { workspace = true }` 依赖声明（全部在 `[dependencies]`，无 dev/build 残留；保留 core/client/webhook 三个例外）。auth 额外从 `oauth = ["reqwest","url"]` feature 移除 `"reqwest"`（→ `["url"]`）。同步清除这 12 个 crate 的 `[package.metadata.cargo-machete] ignored` 列表里的 `"reqwest"` 项（假阴性根因；其余 tokio/tracing/url 等未用债务出范围不动）。验证：`cargo build --workspace --all-features` + grep 双重确认

## 2. 防回归守卫脚本

- [x] 2.1 新增 `tools/check_reqwest_boundary.sh`：遍历 `crates/openlark-*/`，白名单 {core,client,webhook}，业务 crate Cargo.toml 出现 reqwest 则 exit 1 + 列违规项；`set -euo pipefail` + ✅/❌ 风格对齐 `check_no_dead_code_allows.sh`。验证：清理后 exit 0 + 构造违规能抓回归（exit 1）

## 3. 守卫接入 justfile 与 CI

- [x] 3.1 justfile 加 `reqwest-boundary` recipe（平行 `no-dead-code-allows`，line 17-19 后）；ci.yml lint job 加一步「Check Transport/reqwest boundary」（line 115-116 后）。验证：`just reqwest-boundary` 通过 + ci.yml YAML 合法

## 4. 边界文档化（ARCHITECTURE.md）

- [x] 4.1 在 `## 模块详细设计`（line 106）下新增「Transport HTTP 边界」小节：边界定义 + 调用路径图 + Cargo 依赖边界表（core/client/webhook 允许，业务禁止）+ webhook by-design 例外（#214）+ 中间件/熔断/重试层标注「规划中 future、当前未实现」

## 5. webhook crate 文档注释强化

- [x] 5.1 `crates/openlark-webhook/src/robot/v1/` 的 `shared_client`/`client` doc 注释补一行指向 ARCHITECTURE.md「Transport HTTP 边界」约定（不改源码逻辑/签名）。验证：`cargo build -p openlark-webhook --all-features` + grep

## 6. CHANGELOG

- [ ] 6.1 在 CHANGELOG.md `[Unreleased]`（v0.18 待发段）`### Changed` 下补 hygiene 条目：移除 12 业务 crate 未用 reqwest 依赖 + 清 machete ignore 债务 + 新增 `check_reqwest_boundary.sh` 守卫（非 breaking）

## 7. 全量验证

- [ ] 7.1 `cargo build --workspace --all-features` exit 0
- [ ] 7.2 三组 feature clippy（default / `--all-features` / `--no-default-features` + `-D warnings`）均 exit 0
- [ ] 7.3 `cargo test --workspace` 全部通过（0 failed）
- [ ] 7.4 `bash tools/check_reqwest_boundary.sh` exit 0 + grep 双重确认（12 crate Cargo.toml 与 src `reqwest` 命中均 0；core/client/webhook 保留；justfile+ci.yml 接线存在）
