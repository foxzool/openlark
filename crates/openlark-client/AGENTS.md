# openlark-client Knowledge Base

**Crate**: High-Level Client  
**Purpose**: 统一客户端接口、轻量服务注册表、meta 调用链

## OVERVIEW

OpenLark 高级客户端，提供统一的 `Client` 入口点和轻量级 ServiceRegistry 诊断元信息。
支持 feature 条件编译和流畅的链式调用体验。编译能力真相来自 `capability` catalog。

## STRUCTURE

```
src/
├── lib.rs                    # Crate 入口，导出 prelude 和公开 API
├── client.rs                 # 主 Client 实现 + ClientBuilder
├── error.rs                  # 客户端错误处理 + CoreError 适配
├── capability/               # 编译期能力目录（Client 字段 + registry 元数据）
│   ├── catalog.rs           # 统一声明
│   └── macros.rs            # registry / Client 投影
├── registry/                 # 轻量 registry（仅不可变元信息诊断）
│   ├── mod.rs               # DefaultServiceRegistry + types
│   └── bootstrap.rs         # 调用 capability catalog 注册
├── traits/                   # 核心 trait 定义
│   ├── mod.rs
│   ├── client.rs            # LarkClient
│   └── service.rs           # ServiceTrait + ServiceLifecycle
├── utils.rs                  # env/feature 诊断工具
└── ws_client/                # WebSocket 客户端（可选 feature）
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| 添加业务域 | `src/capability/catalog.rs` | 统一声明生成 Client 字段 + registry 元数据 |
| 修改构建器 | `src/client.rs` / `client/builder.rs` | ClientBuilder 与 Client |
| 服务注册查询 | `src/registry/mod.rs` | metadata-only：list/lookup/presence |
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

### ServiceRegistry 使用
```rust
// 检查服务是否可用（编译能力诊断）
if client.registry().has_service("docs") { ... }

// 列出所有已注册的服务元信息
for entry in client.registry().list_services() { ... }
```

## ANTI-PATTERNS

- ❌ 不要直接实例化业务 crate 的服务（使用 meta 链式访问）
- ❌ 不要在 client 中硬编码业务逻辑
- ❌ 不要期望 registry 返回可调用服务实例或生命周期状态（仅元信息）
- ❌ 不要再引入 FeatureLoader 式旁路初始化

## NOTES

- **默认 Features**: `default = ["auth", "communication"]`
- **可选 Features**: `docs`, `meeting`, `security`, `cardkit`, `websocket`, `p0-services`
- **Prelude**: `use openlark_client::prelude::*;` 导入常用类型
- **环境变量**: 支持 `OPENLARK_APP_ID`, `OPENLARK_APP_SECRET` 自动配置
