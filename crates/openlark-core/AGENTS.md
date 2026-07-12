# openlark-core Knowledge Base

**Crate**: Core Infrastructure  
**Purpose**: HTTP 客户端、错误处理、认证和核心工具

## OVERVIEW

OpenLark 核心基础设施 crate，提供所有业务模块共享的底层能力。包含企业级错误处理系统、HTTP 客户端抽象、令牌管理和验证工具。

## STRUCTURE

```
src/
├── api/                      # API 基础类型和 trait
│   ├── mod.rs               # ApiRequest, ApiResponse 定义
│   ├── prelude.rs           # API 模块预置导入
│   ├── responses.rs         # Response<T>, RawResponse 定义
│   └── traits.rs            # ApiResponseTrait 等 trait
├── auth/                     # 认证相关（令牌抽象 - 具体实现已迁移到 openlark-auth）
│   ├── mod.rs               # 认证模块入口
│   ├── app_ticket.rs        # App Ticket 管理（保留在 core 因依赖 http 模块）
│   └── token_provider.rs    # 令牌提供者 trait（抽象接口）
├── config.rs                 # 客户端配置
│   ├── mod.rs               # 认证模块入口
│   ├── app_ticket.rs        # App Ticket 管理（保留在 core 因依赖 http 模块）
│   └── token_provider.rs    # 令牌提供者 trait（抽象接口）
│   ├── mod.rs               # 认证模块入口
│   ├── app_ticket.rs        # App Ticket 管理
│   ├── cache.rs             # 令牌缓存
│   ├── refresh.rs           # 令牌刷新
│   ├── token.rs             # AccessToken/RefreshToken
│   ├── token_provider.rs    # 令牌提供者
│   └── validator.rs         # 令牌验证
├── config.rs                 # 客户端配置
├── constants.rs              # 全局常量（AppType、AccessTokenType 等）
├── error/                    # CoreError 错误系统
│   ├── core.rs              # 核心错误类型（CoreError 枚举）
│   ├── codes.rs             # 飞书错误码映射
│   ├── context.rs           # 错误上下文
│   ├── traits.rs            # ErrorType 枚举
│   ├── prelude.rs           # 错误系统预置导入
│   └── mod.rs               # 错误模块入口
├── http.rs                   # Transport 公开 seam（HTTP 边界）
├── req_option.rs            # 请求选项
├── request_execution/       # pub(crate) 深请求执行（#422）
│   ├── mod.rs               # UnifiedRequestBuilder 协调
│   ├── auth_handler.rs      # TokenProvider 双 adapter 认证 seam
│   ├── multipart_builder.rs # multipart 组合规则
│   └── decode.rs            # 类型化响应解码（Data/Flatten/Text/Binary/Custom）
├── validation/              # 参数验证
│   ├── mod.rs               # 模块入口
│   ├── core.rs              # 核心验证函数 + ValidateBuilder
│   └── validatable.rs       # Validatable trait
└── (内部模块)
    ├── content_disposition.rs
    ├── observability.rs      # pub(crate) - 可观测性
    ├── performance/          # 性能监控
    ├── testing/              # 测试工具
    │   ├── mod.rs
    │   ├── assertions.rs
    │   ├── fixtures.rs
    │   └── mock_context.rs
    └── utils.rs
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| 添加错误码 | `src/error/codes.rs` | 飞书通用错误码映射 |
| 修改 HTTP 行为 | `src/http.rs` | reqwest 客户端配置 |
| 添加验证规则 | `src/validation/core.rs` | 复用 validate_required! 宏 |
| 令牌管理 | `crates/openlark-auth/src/common/token.rs` | AccessToken/RefreshToken (已迁移) |
| 常量定义 | `src/constants.rs` | BASE_URL, 默认超时等 |
| 请求选项 | `src/req_option.rs` | RequestOption（字段为 Option<String>） |

## CONVENTIONS

### 错误处理
```rust
// 统一错误类型（LarkAPIError 别名已删除，统一使用 CoreError）
pub type SDKResult<T> = Result<T, CoreError>;

// 验证宏（validate_required 函数已删除，仅保留宏）
validate_required!(self.field, "字段不能为空");
validate_required_list!(self.items, 50, "列表不能为空且不能超过 50 项");
```

### RequestOption 字段语义
```rust
// 字段为 Option<String>，None 表示未设置，Some("") 表示显式设置为空
pub struct RequestOption {
    pub(crate) tenant_key: Option<String>,
    pub(crate) user_access_token: Option<String>,
    // ...
}
// 判断是否设置：option.field.is_some() / option.field.is_none()
// 获取值：option.field.as_deref().unwrap_or("")
```

### 公开 API 设计
- **最小化原则**: 只导出必要的类型（KISS）
- **内部模块**: observability/query_params/request_builder 为 pub(crate)
- **Re-export**: 常用类型从 crate root 重新导出
- **Prelude**: 不 re-export 第三方类型（serde/HashMap 等）

### 错误码优先级
1. 飞书通用 `code`（如 99991671 Token 无效）
2. HTTP `status`
3. 内部业务码

## ANTI-PATTERNS

- ❌ 不要把 core 变成"全家桶"（避免暴露过多内部模块）
- ❌ 不要在 core 引入业务逻辑
- ❌ 不要破坏 Validatable trait 的语义一致性
- ❌ 不要使用 LarkAPIError（已删除，统一使用 CoreError）
- ❌ 不要使用 validate_required 函数（已删除，使用 validate_required! 宏）
- ❌ 不要在 validation/ 下添加业务验证模块（file/pagination/password/uuid 已删除）

## COMMANDS

```bash
# 测试 core
cargo test -p openlark-core

# 检查 core 独立编译
cargo check -p openlark-core
```
