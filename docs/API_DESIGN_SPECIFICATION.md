# OpenLark API 设计规范

**文档版本**: 1.0.0  
**适用范围**: 所有 openlark-* feature crates  
**关联 Issue**: #41 (方案A)  
**状态**: 正式规范

---

## 1. 概述

本文档定义 OpenLark SDK 中 API 实现的统一设计规范，确保所有业务模块（crates）遵循一致的调用范式、错误处理和参数验证模式。

### 1.1 设计目标

- **一致性**: 同域内禁止出现第二套执行范式
- **可维护性**: 统一代码结构，降低认知负担
- **可扩展性**: 新 API 实现有明确模板可循
- **向后兼容**: 现有代码渐进式迁移，不强制一次性重写

### 1.2 非目标

- 不一次性重写全部历史实现
- 不以新增 endpoint 常量/枚举为本文档目标（见 `openlark-api` skill）
- 不强制要求所有历史代码立即符合规范（允许渐进式改进）

---

## 2. Canonical 范式定义

OpenLark 采用 **"Request 自持 Config + execute() 委托 execute_with_options()"** 作为唯一 canonical 执行范式。

### 2.1 核心结构

```rust
// 1. Request 结构体持有 Config
pub struct CreateXxxRequest {
    config: Config,
    // ... 其他字段
}

// 2. Builder 风格构造函数
impl CreateXxxRequest {
    pub fn new(config: Config) -> Self { ... }
    
    pub fn field(mut self, value: T) -> Self { 
        self.field = value;
        self 
    }
    
    // 3. execute() 委托给 execute_with_options()
    pub async fn execute(self) -> SDKResult<Response> {
        self.execute_with_options(RequestOption::default()).await
    }
    
    // 4. execute_with_options() 是实际执行入口
    pub async fn execute_with_options(
        self, 
        option: RequestOption
    ) -> SDKResult<Response> {
        // 验证 → 构建请求 → 透传 option → 发送
    }
}
```

### 2.2 执行流程

```
用户调用
    ↓
Request::new(config) → Builder 链式设置参数
    ↓
.execute() → 委托 .execute_with_options(RequestOption::default())
    ↓
.execute_with_options(option) → 参数验证 (validate_required!)
    ↓
构建 ApiRequest → Transport::request(..., Some(option))
    ↓
返回 SDKResult<Response>
```

### 2.3 代码模板

```rust
//! API 名称
//!
//! docPath: https://open.feishu.cn/document/...

use openlark_core::{
    api::ApiRequest, 
    config::Config, 
    http::Transport, 
    req_option::RequestOption,
    validate_required, 
    SDKResult,
};
use serde::{Deserialize, Serialize};

// 请求体（如需独立定义）
#[derive(Debug, Serialize, Deserialize)]
pub struct XxxRequestBody {
    pub field1: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field2: Option<String>,
}

// 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct XxxResponse {
    pub data: serde_json::Value,
}

// Request Builder
pub struct XxxRequest {
    config: Config,
    path_param: String,
    query_param: Option<String>,
}

impl XxxRequest {
    pub fn new(config: Config, path_param: impl Into<String>) -> Self {
        Self {
            config,
            path_param: path_param.into(),
            query_param: None,
        }
    }

    pub fn query_param(mut self, v: impl Into<String>) -> Self {
        self.query_param = Some(v.into());
        self
    }

    // Canonical execute: 委托给 execute_with_options
    pub async fn execute(self, body: XxxRequestBody) -> SDKResult<XxxResponse> {
        self.execute_with_options(body, RequestOption::default()).await
    }

    // 实际执行入口
    pub async fn execute_with_options(
        self,
        body: XxxRequestBody,
        option: RequestOption,
    ) -> SDKResult<XxxResponse> {
        // 1. 必填校验
        validate_required!(self.path_param, "path_param 不能为空");
        validate_required!(body.field1, "field1 不能为空");

        // 2. 构建请求
        let mut req = ApiRequest::<XxxResponse>::post(&format!("/open-apis/xxx/{}", self.path_param));
        
        if let Some(ref q) = self.query_param {
            req = req.query("query_param", q);
        }
        
        req = req.body(serde_json::to_string(&body)?);

        // 3. 发送请求（关键：option 必须透传）
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        
        // 4. 提取响应数据
        Ok(resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据", "服务器未返回有效数据")
        })?)
    }
}
```

---

## 3. 必须项 (MUST)

### 3.1 Request 结构

| 必须项 | 说明 | 证据 |
|--------|------|------|
| 持有 `Config` | Request 必须持有 `Config`（非 `Arc<Config>`，Config 内部已用 Arc） | `crates/openlark-communication/src/im/im/v1/message/create.rs:66` |
| Builder 风格 | 使用 `new(config)` + 链式 `field()` 方法 | `crates/openlark-docs/src/base/base/v2/app/role/create.rs:46-87` |
| 消费 self | `execute()` 和 `execute_with_options()` 必须消费 `self` | 全仓库一致 |

### 3.2 执行方法

| 必须项 | 说明 | 证据 |
|--------|------|------|
| 双方法设计 | 必须同时提供 `execute()` 和 `execute_with_options(RequestOption)` | `crates/openlark-communication/src/im/im/v1/message/create.rs:87-116` |
| execute 委托 | `execute()` 必须委托给 `execute_with_options(RequestOption::default())` | `crates/openlark-docs/src/base/base/v2/app/role/create.rs:89-92` |
| option 透传 | `execute_with_options` 必须将 `option` 透传到 `Transport::request(..., Some(option))` | `crates/openlark-communication/src/im/im/v1/message/create.rs:113` |

### 3.3 参数验证

| 必须项 | 说明 | 证据 |
|--------|------|------|
| 使用宏验证 | 必填字段必须使用 `validate_required!` 宏 | `crates/openlark-core/src/lib.rs:50-57` |
| 前置验证 | 所有验证必须在执行逻辑之前完成 | `crates/openlark-docs/src/base/base/v2/app/role/create.rs:98-117` |
| 空白检查 | `validate_required!` 会自动 trim 后检查空白字符串 | `crates/openlark-core/src/validation/validatable.rs` |

### 3.4 请求发送

| 必须项 | 说明 | 证据 |
|--------|------|------|
| Transport 发送 | 必须使用 `Transport::request(req, &self.config, Some(option))` | 全仓库一致 |
| option 包裹 | 第三个参数必须是 `Some(option)`，不能是 `None` | `crates/openlark-cardkit/src/cardkit/cardkit/v1/card/create.rs:98` |

---

## 4. 禁止项 (MUST NOT)

### 4.1 执行范式

| 禁止项 | 说明 | 现有混用点 |
|--------|------|------------|
| 禁止 `send()` | 同域内禁止使用 `send()` 作为执行方法名 | `crates/openlark-security/src/security/acs/v1/users/mod.rs` 等 7 处 |
| 禁止单方法 | 禁止只提供 `execute()` 而不提供 `execute_with_options()` | 部分历史代码 |
| 禁止 option 忽略 | 禁止在 `execute_with_options` 中忽略传入的 option | 无 |

### 4.2 Config 处理

| 禁止项 | 说明 |
|--------|------|
| 禁止 Arc 包裹 | Request 内禁止再包 `Arc<Config>`（Config 内部已是 Arc） |
| 禁止独立 client | Service/Request 禁止持有独立 HTTP client |

### 4.3 验证处理

| 禁止项 | 说明 |
|--------|------|
| 禁止手动 if 检查 | 必填校验禁止手写 `if field.is_empty()`，必须用 `validate_required!` |
| 禁止 unwrap | 库代码禁止对 `Config::build()` 等使用 `.unwrap()` |

---

## 5. 同域禁止第二套执行范式

### 5.1 定义

"同域"指同一个 crate 内的同一个业务子域（如 `openlark-docs` 的 `ccm/drive/v1`）。

### 5.2 规则

- 同一子域内的所有 API 必须遵循相同的执行范式
- 若子域内存在历史混用（如既有 `execute` 又有 `send`），新实现必须遵循该子域的主流范式
- 禁止在同一子域内引入新的执行方法名

### 5.3 现有混用点及收敛方向

| 位置 | 现状 | 收敛方向 |
|------|------|----------|
| `openlark-security/src/security/acs/v1/*/mod.rs` | 使用 `send()` | 保持现状，新 API 沿用 `send()` 直到该子域整体迁移 |
| `openlark-auth/src/auth/oauth/old/` | 使用 `send()` | old 版本保持现状，v3+ 使用 `execute()` |
| `openlark-docs/src/base/base/v2/` | 使用 `execute()` | 保持 canonical 模式 |
| `openlark-communication/src/im/im/v1/` | 使用 `execute()` | 保持 canonical 模式 |

---

## 6. RequestOption 透传要求

### 6.1 目的

`RequestOption` 用于传递用户态 token、request_id、自定义 header 等请求级配置，必须确保从用户调用点透传到 HTTP 层。

### 6.2 透传链

```
用户代码
    ↓
Request::execute_with_options(option)  // 用户传入
    ↓
Transport::request(req, config, Some(option))  // 必须包裹为 Some()
    ↓
HTTP Client 发送请求（携带 option 中的 header/token）
```

### 6.3 必须遵守

- `execute_with_options` 的 `option` 参数必须透传到 `Transport::request` 的第三个参数
- 禁止在中间层丢弃或替换 option
- 禁止将 `Some(option)` 改为 `None`

### 6.4 代码示例

```rust
// ✅ 正确：option 透传
pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<Response> {
    // ... 验证和构建请求 ...
    let resp = Transport::request(req, &self.config, Some(option)).await?;
    // ...
}

// ❌ 错误：option 被丢弃
pub async fn execute_with_options(self, _option: RequestOption) -> SDKResult<Response> {
    let resp = Transport::request(req, &self.config, None).await?;  // 错误！
    // ...
}
```

---

## 7. validate_required! 使用规范

### 7.1 宏定义

```rust
#[macro_export]
macro_rules! validate_required {
    ($field:expr, $error_msg:expr) => {
        if $crate::Validatable::is_empty_trimmed(&$field) {
            return Err($crate::error::CoreError::validation_msg($error_msg));
        }
    };
}
```

### 7.2 使用要求

| 要求 | 说明 |
|------|------|
| 前置调用 | 必须在 `execute_with_options` 开头调用，先于任何网络操作 |
| 字段覆盖 | 所有必填字段（包括 path param、query param、body 字段）都必须验证 |
| 错误消息 | 错误消息必须清晰指明哪个字段不能为空 |

### 7.3 代码示例

```rust
pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<Response> {
    // 1. 验证 path/query 参数
    validate_required!(self.app_token, "app_token 不能为空");
    
    // 2. 验证 body 字段
    validate_required!(self.req.role_name, "role_name 不能为空");
    
    // 3. 复杂验证（长度、范围等）
    if self.req.role_name.chars().count() > 100 {
        return Err(openlark_core::error::validation_error(
            "role_name", "长度不能超过 100 字符"
        ));
    }
    
    // ... 后续逻辑
}
```

### 7.4 列表验证

对于列表字段，使用 `validate_required_list!`：

```rust
validate_required_list!(self.user_ids, 50, "用户 ID 列表不能为空且不能超过 50 个");
```

---

## 8. 导出与测试要求

### 8.1 模块导出

每个 API 文件必须在同级 `mod.rs` 中导出：

```rust
// src/xxx/v1/yyy/mod.rs
pub mod create;
pub mod get;
pub mod models;  // 如果存在
```

### 8.2 测试要求

| 要求 | 说明 |
|------|------|
| Builder 测试 | 必须测试 Builder 链式方法正确设置字段 |
| 序列化测试 | 必须测试请求/响应体的 JSON 序列化/反序列化 |
| 空值测试 | 必须测试空字符串/空列表的验证行为 |

### 8.3 测试模板

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_builder() {
        let config = Config::default();
        let request = CreateRequest::new(config, "token")
            .field1("value1")
            .field2("value2");
        
        assert_eq!(request.path_param, "token");
        assert_eq!(request.field1, Some("value1".to_string()));
    }

    #[test]
    fn test_request_body_serialization() {
        let body = RequestBody {
            field1: "value".to_string(),
            field2: None,
        };
        
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("field1"));
    }

    #[test]
    fn test_empty_field_validation() {
        // 验证空字符串会被 validate_required! 捕获
        let body = RequestBody {
            field1: "".to_string(),  // 空字符串
        };
        // 实际验证在 execute_with_options 中
    }
}
```

---

## 9. 兼容策略

### 9.1 历史代码处理

- **不强制重写**: 现有代码保持现状，除非有功能需求
- **渐进改进**: 新 API 必须遵循本规范
- **同域一致**: 若子域内存在混用，新 API 遵循该子域主流范式

### 9.2 迁移路径（可选）

若决定迁移历史代码：

1. **阶段 1**: 新增 `execute_with_options` 方法，保留原有 `execute`/`send`
2. **阶段 2**: 标记旧方法为 `#[deprecated]`
3. **阶段 3**: 在 major 版本升级时移除旧方法

### 9.3 Breaking Change 控制

- 禁止在 minor/patch 版本中移除或重命名公开方法
- 禁止修改现有方法的签名
- 新增方法不受限制

---

## 10. 参考实现

### 10.1 端点常量风格（推荐默认）

参考: `crates/openlark-communication/src/im/im/v1/message/create.rs`

特点:
- 使用端点常量（`endpoints::IM_V1_MESSAGES`）
- 灵活响应类型（`serde_json::Value` 或具体结构体）
- 适合 IM、通讯等快速迭代的 API

### 10.2 Enum 端点风格

参考: `crates/openlark-docs/src/base/base/v2/app/role/create.rs`

特点:
- 使用 `ApiEndpoint` enum 生成 URL
- 强类型响应（实现 `ApiResponseTrait`）
- 适合 Docs、Bitable 等结构化强的 API

---

## 11. 检查清单

新增 API 时，使用以下清单自查：

- [ ] Request 结构体持有 `Config`
- [ ] 提供 `new(config)` 构造函数
- [ ] Builder 链式方法返回 `Self`
- [ ] 提供 `execute()` 方法
- [ ] 提供 `execute_with_options(RequestOption)` 方法
- [ ] `execute()` 委托给 `execute_with_options(RequestOption::default())`
- [ ] `execute_with_options` 使用 `validate_required!` 验证必填字段
- [ ] `Transport::request` 第三个参数为 `Some(option)`
- [ ] 同级 `mod.rs` 导出该模块
- [ ] 包含 Builder 测试
- [ ] 包含序列化测试

---

## 12. 相关文档

- [标准示例](../.agents/skills/openlark-api/references/standard-example.md) - 代码模板
- [EXECUTE_REFACTORING_ANALYSIS.md](../crates/openlark-docs/docs/EXECUTE_REFACTORING_ANALYSIS.md) - 重构分析
- [PUBLIC_REEXPORT_POLICY.md](PUBLIC_REEXPORT_POLICY.md) - 公开 API 策略
- [AGENTS.md](../AGENTS.md) - 项目知识库

---

**文档维护**: OpenLark 核心团队  
**最后更新**: 2025-04-08
