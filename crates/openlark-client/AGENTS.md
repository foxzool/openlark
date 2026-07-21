# openlark-client Knowledge Base

**Crate**: High-Level Client
**Purpose**: 统一客户端接口、meta 调用链

## OVERVIEW

OpenLark 高级客户端，提供统一的 `Client` 入口点与 meta 链式字段访问。
支持 feature 条件编译和流畅的链式调用体验。业务域字段全部由 `capability` catalog 生成。

> #471 移除了 speculative 的 registry / traits / lazy 半边（零外部消费者）。
> 本 crate 现只保留 catalog 的 Client-construction 半边；不再有 metadata-only registry。

## STRUCTURE

```
src/
├── lib.rs                    # Crate 入口，导出 prelude 和公开 API
├── client.rs                 # 主 Client 实现 + ClientBuilder + catalog 字段投影
├── error.rs                  # 客户端错误处理 + CoreError 适配
├── capability/               # 编译期能力目录（Client 字段单一事实来源）
│   ├── catalog.rs           # 统一声明 + 字段唯一性投影
│   └── unique.rs            # generation-time 字段去重（#423 / #455 / #471）
├── utils.rs                  # env/feature 诊断工具
└── ws_client/                # WebSocket 客户端（可选 feature）
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| 添加业务域 | `src/capability/catalog.rs` | 统一声明生成 Client 字段（feature/field/ty/doc/init） |
| 修改构建器 | `src/client.rs` / `client/builder.rs` | ClientBuilder 与 Client |
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

### Client 配置访问
```rust
// 统一配置入口（inherent 方法，非 trait）
let cfg: &openlark_core::config::Config = client.config();
client.is_configured();
```

## ANTI-PATTERNS

- ❌ 不要直接实例化业务 crate 的服务（使用 meta 链式访问）
- ❌ 不要在 client 中硬编码业务逻辑
- ❌ 不要重新引入 metadata-only registry / lifecycle trait / FeatureLoader 式抽象
  （#471 已证这些是零消费者的 speculative seam）

## NOTES

- **默认 Features**: `default = ["auth", "communication"]`
- **可选 Features**: `docs`, `meeting`, `security`, `cardkit`, `websocket`, `p0-services`
- **Prelude**: `use openlark_client::prelude::*;` 导入常用类型
- **环境变量**: 支持 `OPENLARK_APP_ID`, `OPENLARK_APP_SECRET` 自动配置
- **编译期保证**: 字段唯一 / 禁用 feature 不产 Client 字段由 `openlark-capability-unique`
  trybuild crate（workspace 成员，非本 crate 依赖）覆盖。
