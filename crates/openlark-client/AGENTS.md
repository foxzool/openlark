# openlark-client Knowledge Base

**Crate**: High-Level Client  
**Purpose**: 统一客户端接口、轻量服务注册表、meta 调用链

## OVERVIEW

OpenLark 高级客户端，提供统一的 `Client` 入口点和轻量级 ServiceRegistry 元信息注册表。支持 feature 条件编译和流畅的链式调用体验。

## STRUCTURE

```
src/
├── lib.rs                    # Crate 入口，导出 prelude 和公开 API
├── client.rs                 # 主 Client 实现 + ClientBuilder
├── client_build_config.rs    # ClientBuilder 内部构建配置（委托生成 openlark-core Config）
├── error.rs                  # 客户端错误处理 + CoreError 适配
├── features.rs               # FeatureLoader（调用 registry::bootstrap）
├── registry/                 # 轻量 registry（仅元信息）
│   ├── mod.rs               # DefaultServiceRegistry + types
│   └── bootstrap.rs         # 编译期服务元信息注册清单
├── traits/                   # 核心 trait 定义（仅 3 个）
│   ├── mod.rs
│   ├── client.rs            # LarkClient
│   └── service.rs           # ServiceTrait + ServiceLifecycle
├── types/                    # 类型定义/别名
├── utils.rs                  # env/feature 诊断工具
└── ws_client/                # WebSocket 客户端（可选 feature）
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| 添加新服务入口 | `src/client.rs` | 在 Client 结构体添加 feature-gated 字段 |
| 修改构建器 | `src/client.rs` | ClientBuilder 与 Client 在同一文件 |
| 服务注册 | `src/registry/mod.rs` | DefaultServiceRegistry 元信息存储 |
| 服务注册清单 | `src/registry/bootstrap.rs` | 编译期按 feature 注册元信息 |
| Feature 加载 | `src/features.rs` | FeatureLoader 入口 |
| Traits | `src/traits/` | LarkClient, ServiceTrait, ServiceLifecycle |
| WebSocket | `src/ws_client/` | 需要 `websocket` feature |
| 错误处理 | `src/error.rs` | 客户端级别错误 + CoreError 扩展 |

## CONVENTIONS

### Meta 调用链
```rust
// 推荐的 API 访问方式
client.docs.ccm.drive.v1().file().upload(...).execute().await?;
client.communication.im.v1().message().send(...).execute().await?;
client.auth.app().access_token().get().execute().await?;
```

### Feature 条件编译
```rust
#[cfg(feature = "docs")]
pub docs: DocsClient,

#[cfg(feature = "communication")]
pub communication: CommunicationClient,
```

### ServiceRegistry 使用
```rust
// 检查服务是否可用（元信息层面）
if client.registry().has_service("docs") { ... }

// 列出所有已注册的服务元信息
for entry in client.registry().list_services() { ... }
```

## ANTI-PATTERNS

- ❌ 不要直接实例化业务 crate 的服务（使用 meta 链式访问）
- ❌ 不要在 client 中硬编码业务逻辑
- ❌ 不要期望 registry 返回可调用服务实例（仅元信息）

## NOTES

- **默认 Features**: `default = ["auth", "communication"]`
- **可选 Features**: `docs`, `meeting`, `security`, `cardkit`, `websocket`, `p0-services`
- **Prelude**: `use openlark_client::prelude::*;` 导入常用类型
- **环境变量**: 支持 `OPENLARK_APP_ID`, `OPENLARK_APP_SECRET` 自动配置
