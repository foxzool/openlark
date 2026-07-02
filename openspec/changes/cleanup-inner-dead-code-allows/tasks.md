## 1. 前置安全核查（D2 风险缓解）

- [ ] 1.1 核查 workspace 内无 crate 直接启用 `openlark-core` 的 `tracing-init`/`otel`（design 探查已证：仅 hr/docs 启用 `testing`，保留；无 crate 启用 tracing-init/otel）。`grep -rn 'openlark-core' crates/ --include=Cargo.toml` 复核。
- [ ] 1.2 核查无测试/示例引用已删的 `otel`/`tracing-init` feature 符号（design 探查已证无）；`pub mod testing`（被 hr/docs 大量用）保留。

## 2. openlark-core：删 observability + 移除 feature/依赖（D2）

- [x] 2.1 **重写** `crates/openlark-core/src/observability.rs`：仅保留 `ResponseTracker`（+ 4 测试），删死 tracker/trace 函数/5 宏/tracing-init/otel 门控块 + 文件顶 `#![allow(dead_code)]`（build 实测修正：`response_handler` 用 `ResponseTracker`，非全文件死）。**保留 `lib.rs` 的 `pub(crate) mod observability;`**。
- [x] 2.2 从 `crates/openlark-core/Cargo.toml` 移除 `tracing-init`/`otel` feature（含注释）；**`testing` 解耦为 `testing = []`（保留，去掉 `["tracing-init"]`）**。另：删根 `Cargo.toml` 的 `otel = ["openlark-core/otel"]` 转发 feature（build 实测发现，Task 1 crates/-only grep 漏检）。
- [x] 2.3 从 `[dependencies]` 移除 4 个 optional 依赖及其 `[dependencies.X]` 表：`opentelemetry`、`opentelemetry_sdk`、`opentelemetry-otlp`、`tracing-opentelemetry`；从 `[dependencies]` + `[dev-dependencies]` 移除 `tracing-subscriber`（两处）；根 `Cargo.toml` `[workspace.dependencies]` 同步删这 5 项（保留 `tracing` 本体）。
- [x] 2.4 确认 `pub mod testing` 完整保留、hr/docs 的 `features = ["testing"]` 仍工作（解耦后 testing 不再拉 tracing-init）。
- [ ] 2.5 同步更新 `.github/msrv/Cargo.lock`（删依赖必须）—— **移到 Task 8 Step 5 与全量验证一起做**。
- [x] 2.6 `cargo check -p openlark-core`（default / `--all-features` / `--no-default-features`）三组均编译通过 ✓。

## 3. openlark-core：删 query_params / header_builder 死项（D3）

- [x] 3.1 **整文件删** `query_params.rs`（1085 行，顶层仅 QueryParams/QueryParamsBuilder 两死 struct，0 外部 use）+ `lib.rs` 的 `pub(crate) mod query_params;` 声明。
- [x] 3.2 **项级删** `request_builder/header_builder.rs`：删 `add_headers` 函数 + 3 个 `add_headers` 测试 + 文件顶 `#![allow(dead_code)]`；保留 `HeaderBuilder`/`build_headers`/`add_header`（活于 `request_builder/mod.rs:46,48`）。

## 4. openlark-hr：删废弃 endpoints 模块（D1）

- [x] 4.1 删除 `crates/openlark-hr/src/endpoints/` 整个目录（仅 mod.rs）。
- [x] 4.2 移除 `lib.rs:67-69` 的「端点保留（已废弃…）」注释 + `#[allow(deprecated)]` + `mod endpoints;`。
- [x] 4.3 `cargo check -p openlark-hr` 绿、0 dead_code；`cargo test --lib` 1250 passed（注：`--features testing` 是无效 flag，testing 乃 core feature，hr 经 dev-dep 拉取）。

## 5. openlark-mail：删孤儿字段 + User.config 显式处理（D4）

- [x] 5.1 删除 6 处孤儿字段 `delete_id` / `patch_id`（alias、folder、mail_contact、rule 的 delete+patch）+ 各 `new()` 初始化（sed 按行删，已复核 6 文件 0 残留）。
- [x] 5.2 `mail/v1/user/mod.rs` 的 `User.config` 字段加 `#[expect(dead_code)]` + 注释「导航 struct，accessor 待补（#274/#275 范式），本 change 不接线」。
- [x] 5.3 移除 `crates/openlark-mail/src/lib.rs:1` 的 `#![allow(dead_code)]`（保留 `clippy::module_inception`）。
- [x] 5.4 `cargo check -p openlark-mail` 绿、`clippy --all-targets` 0 dead_code ✓。

## 6. openlark-bot / openlark-docs：删 stale allow（D5）

- [ ] 6.1 移除 `crates/openlark-bot/src/lib.rs:1` 的 `#![allow(dead_code)]`。
- [ ] 6.2 移除 `crates/openlark-docs/src/ccm/explorer/explorer/mod.rs:1` 的 `#![allow(dead_code)]`。

## 7. CI 脚本收口（D6）

- [ ] 7.1 编辑 `tools/check_no_dead_code_allows.sh`：清空 `KNOWN_INNER_DEBT` heredoc 内容并更新脚本尾部文案（inner-attribute 不再享受豁免）。
- [ ] 7.2 运行 `bash tools/check_no_dead_code_allows.sh` 确认 PASS。

## 8. 全量验证（spec 验收场景）

- [ ] 8.1 `cargo fmt --check`（CI lint 第一步，避免重蹈 #270/#280 漏 fmt 致 lint fail）。
- [ ] 8.2 `cargo clippy --workspace --all-targets` 三组——default / `--all-features` / `--no-default-features`——均 0 dead_code 警告、0 `#![allow(dead_code)]` 残留（对应 spec「全 workspace 内外层均无 cruft 残留」+「废弃模块被删除而非抑制」scenario）。
- [ ] 8.3 `cargo test --workspace` 全绿（删除项均 0 引用，无行为回归）。
- [ ] 8.4 `cargo build --workspace --all-features` 与 `--no-default-features` 均通过（feature 移除后矩阵仍绿）。
- [ ] 8.5 更新 CHANGELOG v0.18 breaking 区：记录移除的 3 feature + 5 依赖，附迁移指引「若启用过这些 feature，直接移除，无行为变化」。
