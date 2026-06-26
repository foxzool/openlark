## Why

OpenLark v0.16.1 已发展到 18 个 workspace crates、335K 行代码、1,560+ API 的规模。架构审核发现了若干结构性问题：ServiceRegistry 仅存元信息而非真正的服务容器、存在双重 Config 体系导致认知负担、根 crate re-export 层过于厚重、业务 crate 类型命名不一致。这些问题在项目接近 v1.0 之前需要解决，以确保长期可维护性和一致的 API 体验。

## What Changes

- **升级 ServiceRegistry 为真正的服务容器**：当前 registry 仅注册 `ServiceMetadata`（描述性字段），不持有实际服务实例。所有服务在 `Client::with_config()` 中通过 `#[cfg(feature)]` 硬编码初始化。需要改造为运行时可查询的服务定位器，或使用宏自动生成 feature-gated 初始化代码。
- **统一双重 Config 体系**：消除 `openlark_client::Config` 和 `openlark_core::config::Config` 的二元性。将 `openlark_client::Config` 改造为 `openlark_core::config::Config` 的 Builder wrapper，或合并为单一类型。
- **清理根 crate deprecated re-export 层**：移除 v0.15 之前遗留的 `#[deprecated]` crate re-export 和 `*Client` 类型别名，简化 `prelude` 模块。
- **统一业务 crate 的 Client 类型命名**：制定统一规范，消除 `XxxClient` / `XxxService` / `Arc<XxxServices>` 的混用。
- **移除 `openlark-core` testing feature 的默认启用**：将 `default = ["testing"]` 改为 `default = []`。
- **记录 HTTP 中间件层后续路线**：Transport 中间件化影响面较大，当前 change 只记录 v0.18+ 后续评估方向，不引入请求链路行为变化。
- **记录 `ApiRequest<R>` 类型约束复核**：将 request/response 关联关系的类型安全收敛列入后续 core API 设计项。
- **移除 `#![allow(async_fn_in_trait)]`**：MSRV 已为 1.88，不再需要此 allow。
- **添加 `docs.rs` metadata**：确保 `full` feature 下的文档能正确生成。

## Capabilities

### New Capabilities
- `service-registry-upgrade`: 将 ServiceRegistry 从元信息注册升级为真正的服务容器，支持运行时服务查询和延迟初始化

> 归档清理说明：原计划的 `unified-config` 能力已由后续 change `merge-deprecated-config`（v0.18 breaking）交付，并在主 spec 以 `config` 能力落地，本 change 不再单独归档该 delta；`http-middleware-layer` 仅作为 v0.18+ 后续评估方向记录（见 What Changes），未产出 spec，归档时清理其空目录。

### Modified Capabilities
<!-- 无现有 spec 需要修改 -->

## Impact

- **openlark-core**: Config builder 扩展、testing feature 默认值变更；Transport 中间件与 `ApiRequest<R>` 类型约束仅记录为后续事项
- **openlark-client**: Client 初始化逻辑重构、Registry 升级、deprecated re-export 清理、`async_fn_in_trait` allow 移除
- **所有业务 crate**: Client 类型命名统一化
- **根 crate (openlark)**: re-export 层简化、prelude 收窄
- **Cargo.toml**: `docs.rs` metadata 添加
- **破坏性变更**: 移除 deprecated 别名（已标记 `#[deprecated(since = "0.15.0")]` 超过一个次版本）
- **依赖**: 当前 change 不新增依赖；`tower` / `tower-http` 是否引入留到 v0.18+ 评估
