## 任务（open 阶段草案，design brainstorming 后定稿）

> 标注 ⚠️ 的任务依赖 design 阶段未定决策（见 design.md「留待 design 阶段解决的分叉」），定稿后可能拆分或调整。

- [x] T1: core `ConfigInner` 加 `allow_custom_base_url: bool` 字段，同步 `Default`/`Debug`/`with_token_provider`/`build` 等所有构造点
- [x] T2: core `Config::validate()` + `is_known_base_url()` + base_url 白名单 SSRF 防护上移（从 `client/config.rs` 迁移；core 用 `CoreError`；`builder().build()` 不校验，分叉 1）
- [x] T3: core `Config::from_env()` / `load_from_env()` 从 client 迁移，适配 Arc<ConfigInner> 封装；保留 `OPENLARK_*` 环境变量语义；`OPENLARK_TIMEOUT` → `req_timeout(Some(Duration))`（分叉 5）；内部调 validate 不阻塞
- [x] T4: core `ConfigSummary` + `Config::summary()` 从 client 迁移
- [x] T5: core `ConfigBuilder` 加 `allow_custom_base_url()`；`build()` 保持不校验（分叉 1）
- [x] T6: 字段命名按分叉 2 决策：core 用 req_timeout/header 单数，不保留 client 的 timeout/headers 别名（别名随 client::Config 移除消失）
- [x] T7: client 移除 deprecated `client::Config`/`ConfigBuilder`/`ConfigSummary`（plan 遗漏面已补全）：迁移 `client.rs`(with_config) / `utils.rs`(create_config_from_env+get_config_summary) / `builder.rs`(From\<Config\>) / `client_build_config.rs`(From\<Config\> + is_known_base_url→core) / `ws_client/client.rs`(Arc\<client::Config\>→core + 字段直访→accessor) / `lib.rs`(3 测试)；删 `config.rs` + `config.rs.backup` + `pub use config::Config`。core `is_known_base_url` 改 pub。分叉 4
- [x] T8: 根 crate `src/lib.rs:31` re-export 改 `openlark_core::config::Config`
- [x] T9: examples 迁移（test_debug 简化为 core::Config；websocket_echo_bot timeout→req_timeout + build 返回 Config 非 Result）。cargo check --workspace --all-targets --all-features 全绿
- [x] T10: 文档 + CHANGELOG：breaking 迁移指引 + client::Config → core::Config 字段/方法对应表
- [ ] T11: `cargo test` + `cargo clippy --all-targets` + `cargo check --workspace --all-targets` 全绿
