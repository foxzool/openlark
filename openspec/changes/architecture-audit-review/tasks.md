## 1. 快速修复（P2 级，低风险）

- [x] 1.1 移除 `openlark-core/Cargo.toml` 中 `default = ["testing"]`，改为 `default = []`
- [x] 1.2 移除 `openlark-client/src/lib.rs` 中的 `#![allow(async_fn_in_trait)]`
- [x] 1.3 在根 `Cargo.toml` 添加 `[package.metadata.docs.rs]` 配置，启用 `all-features`
- [x] 1.4 运行 `cargo check --workspace` 确认所有修改无编译错误

## 2. 根 crate deprecated 清理

- [x] 2.1 移除 `src/lib.rs` 中所有 `#[deprecated(since = "0.15.0")]` 的 crate re-export（`pub use openlark_xxx`）
- [x] 2.2 移除所有 `#[deprecated(since = "0.15.0")]` 的 `*Client` 类型别名 re-export
- [x] 2.3 清理 `prelude` 模块，移除 `#[allow(deprecated)]` 和对应的 deprecated 导出
- [x] 2.4 保留命名空间 re-export（`pub use openlark_auth as auth` 等）和新版公共类型
- [x] 2.5 更新 `src/lib.rs` 模块文档，反映清理后的导出结构
- [x] 2.6 运行 `cargo test --workspace` 确认无破坏

## 3. 业务 crate Client 类型命名统一

- [x] 3.1 制定命名规范文档：所有业务 crate 导出 `XxxClient` struct（非 type alias、非 Arc 包装）
- [x] 3.2 将 `SecurityClient` 从 `Arc<SecurityServices>` 改为独立 struct（内部持有 Arc）
- [x] 3.3 统一 `WorkflowClient`、`PlatformClient` 等从 type alias 改为正式 struct
- [x] 3.4 更新 `openlark-client/src/lib.rs` 中的对应 re-export
- [x] 3.5 运行 `cargo test --workspace` 确认所有业务 crate 测试通过

## 4. Config 统一（Phase 1: Deprecated）

- [x] 4.1 将 `openlark_client::Config` 标记为 `#[deprecated(since = "0.17.0")]`
- [x] 4.2 扩展 `openlark_core::config::Config::builder()` 支持 `enable_log`、`retry_count` 等原属 client Config 的选项
- [x] 4.3 修改 `Client::builder()` 内部直接构建 `CoreConfig`，不再依赖 client Config
- [x] 4.4 移除 Client 中 `config: Arc<Config>` 字段，仅保留 `core_config`
- [x] 4.5 更新 `Client::config()` 返回 `&openlark_core::config::Config`
- [x] 4.6 运行 `cargo test --workspace` 确认无破坏

## 5. ServiceRegistry 宏化改造 <!-- DEFERRED: v0.18, separate PR -->

- [ ] 5.1 设计声明宏 `register_service!` 的接口（name, feature, client_type, constructor）
- [ ] 5.2 在 `openlark-client/src/client.rs` 中定义宏注册表，替代手动 `#[cfg(feature)]` 块
- [ ] 5.3 使用宏重新生成 `Client` 结构体的 feature-gated 字段
- [ ] 5.4 使用宏重新生成 `Client::with_config()` 中的初始化代码
- [ ] 5.5 简化 `registry/bootstrap.rs`，由宏自动生成注册调用
- [ ] 5.6 确认所有 feature 组合（essential, enterprise, full）编译通过
- [ ] 5.7 运行 `cargo test --workspace` 确认所有测试通过

## 6. HTTP 中间件层（v0.18 延迟实施）<!-- DEFERRED: v0.18, separate PR -->

- [ ] 6.1 评估 `tower` vs 自定义 trait 中间件方案的编译时间和运行时开销
- [ ] 6.2 定义 `Middleware` trait（async fn process）
- [ ] 6.3 实现 `RetryMiddleware`（指数退避，覆盖当前 Transport 中的硬编码逻辑）
- [ ] 6.4 实现 `RateLimitMiddleware`（429 Retry-After 感知）
- [ ] 6.5 实现 `LoggingMiddleware`（tracing span 集成）
- [ ] 6.6 改造 Transport 支持中间件管道注入
- [ ] 6.7 添加性能基准测试（criterion），对比中间件 vs 直接调用的开销
- [ ] 6.8 运行 `cargo test --workspace` 确认所有测试通过

## 7. 文档与迁移

- [x] 7.1 编写 CHANGELOG 条目，记录所有破坏性变更和迁移路径
- [ ] 7.2 更新 `examples/` 中的代码，确保使用新 API
- [ ] 7.3 更新 `README.md` 中的快速开始代码示例
- [x] 7.4 更新 `AGENTS.md` 中的架构说明
- [x] 7.5 运行 `cargo doc --workspace --all-features` 确认文档生成无警告
