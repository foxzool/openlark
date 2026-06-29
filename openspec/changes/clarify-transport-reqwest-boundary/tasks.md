# Tasks — clarify-transport-reqwest-boundary

> 闭环 issue #270：删 12 个业务 crate 未用 reqwest 依赖 + 文档化 `Transport<T>` 边界 + webhook 例外 + 中间件层 future 标注。
> 纯依赖声明 + 文档变更，不改 `.rs` 源码逻辑、不改公开 API。

## 1. 依赖清理

- [ ] 1.1 移除 12 个业务 crate 的 `Cargo.toml` 中未用的 `reqwest = { workspace = true }` 依赖声明：`openlark-analytics`、`openlark-auth`、`openlark-bot`、`openlark-application`、`openlark-communication`、`openlark-mail`、`openlark-hr`、`openlark-docs`、`openlark-helpdesk`、`openlark-platform`、`openlark-user`、`openlark-workflow`（全部在 `[dependencies]`，无 dev/build 残留；保留 core/client/webhook 三个例外）。auth 额外从 `oauth = ["reqwest","url"]` feature 移除 `"reqwest"`（→ `["url"]`）。同步清除这 12 个 crate 的 `[package.metadata.cargo-machete] ignored` 列表里的 `"reqwest"` 项（假阴性根因；其余 tokio/tracing/url 等未用债务出范围不动）

## 2. 边界文档化（ARCHITECTURE.md）

- [ ] 2.1 新增/强化「Transport HTTP 边界」小节：明确 `openlark_core::http::Transport<T>` 是边界、业务 crate 经 `Transport::request()` 发请求、不直接依赖 reqwest 类型
- [ ] 2.2 记录 webhook by-design 例外：无鉴权推送器、进程级共享 `reqwest::Client` 复用连接池（引用 #214），不算分层泄漏
- [ ] 2.3 将「Transport 中间件 / 熔断 / 智能重试中间件」草案显式标注为「规划中 / future change / 当前未实现」，与实际（RetryPolicy 配置模式、无中间件链）对齐

## 3. webhook crate 文档注释强化

- [ ] 3.1 强化 `crates/openlark-webhook/src/robot/v1/` 中 `shared_client`/`client` 的文档注释，显式说明「直接 reqwest 是 by-design 边界例外」并指向 ARCHITECTURE.md 边界约定（不改源码逻辑）

## 4. CHANGELOG

- [ ] 4.1 在 CHANGELOG.md v0.18 段补充 hygiene 条目：12 个业务 crate 移除未用 reqwest 依赖声明（非 breaking）

## 5. 验证

- [ ] 5.1 `cargo build --workspace --all-features` exit 0（删依赖后全量构建通过）
- [ ] 5.2 三组 feature clippy（default / `--all-features` / `--no-default-features` + `-D warnings`）均 exit 0
- [ ] 5.3 `cargo test --workspace` 全部通过（0 failed）
- [ ] 5.4 grep 双重确认：12 crate 的 `Cargo.toml` 与 `src/` 中 `reqwest` 命中数均为 0；core/client/webhook 三者 Cargo.toml 仍保留 reqwest
