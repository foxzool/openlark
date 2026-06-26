## 任务（open 阶段草案，design brainstorming 后定稿）

> 标注 ⚠️ 的任务依赖 design 阶段未定决策（见 design.md「留待 design 阶段解决的分叉」），定稿后可能拆分或调整。

- [x] T1: core `ConfigInner` 加 `allow_custom_base_url: bool` 字段，同步 `Default`/`Debug`/`with_token_provider`/`build` 等所有构造点
- [ ] T2: core `Config::from_env()` / `load_from_env()` 从 client 迁移，适配 Arc<ConfigInner> 封装；保留 `OPENLARK_*` 环境变量语义 ⚠️（timeout 语义对齐见分叉 5）
- [ ] T3: core `Config::validate()` + `is_known_base_url()` + base_url 白名单 SSRF 防护上移
- [ ] T4: core `ConfigSummary` + `Config::summary()` 从 client 迁移
- [x] T5: core `ConfigBuilder` 加 `allow_custom_base_url()`；`build()` 保持不校验（分叉 1）
- [ ] T6: ⚠️ 字段命名兼容按 design 决策（分叉 2）：core 是否保留 `timeout()`/`headers()` 等迁移别名
- [ ] T7: client 移除 deprecated `Config`/`ConfigBuilder`/`ConfigSummary` 本体（`client/config.rs`）；`Client::new`/`builder` 改用 core::Config（分叉 4）
- [ ] T8: 根 crate `src/lib.rs:31` re-export 改 `openlark_core::config::Config`
- [ ] T9: examples 迁移到 core::Config
- [ ] T10: 文档 + CHANGELOG：breaking 迁移指引 + client::Config → core::Config 字段/方法对应表
- [ ] T11: `cargo test` + `cargo clippy --all-targets` + `cargo check --workspace --all-targets` 全绿
