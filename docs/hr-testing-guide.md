# OpenLark HR 测试指南

本指南旨在为 `openlark-hr` 模块建立统一的测试开发标准和规范。由于 HR 模块涉及大量 API（153+），测试开发应遵循“核心路径优先”策略，确保 Builder 逻辑正确、API 请求构造符合预期、以及对服务端响应的正确解析。

## 1. 目录结构说明

所有的 HR 单元测试应存放在项目根目录的 `tests/unit/hr/` 下。测试文件的组织应与 SDK 内部的模块结构保持一致：

```text
tests/unit/hr/
├── mod.rs                  # 模块声明
├── corehr/                 # CoreHR 模块测试
│   ├── v1/
│   │   ├── employee_tests.rs
│   │   └── department_tests.rs
│   └── mod.rs
├── attendance/             # 出勤模块测试
│   ├── v1/
│   │   └── mod.rs
│   └── mod.rs
└── builder_tests.rs        # 通用 Builder 链式调用测试
```

### 对应关系
- 测试文件命名：`{module}_tests.rs`
- 测试函数命名：`test_{api_name}_{scenario}`
- 测试模块内部：使用 `mod {api_name}_tests` 进行二级组织

## 2. 测试命名规范

### 文件命名
文件应反映其测试的业务资源。例如测试 `core_hr.v1.employee` 相关的接口，文件应命名为 `employee_tests.rs`。

### 函数命名
采用 `test_{操作}_{场景}` 格式：
- `test_create_employee_success`: 测试创建员工成功的场景。
- `test_get_employee_invalid_id`: 测试获取员工时传入无效 ID 的场景。

## 3. Builder 测试模式

Builder 测试是 SDK 健壮性的基础，需验证所有 setter 方法能否正确影响生成的请求参数。

### 关键验证点
1. **默认状态**: 验证 `default()` 或 `builder()` 初始状态下的参数。
2. **Setter 覆盖**: 验证多次调用同一个 setter 是否正确覆盖旧值。
3. **类型安全**: 确保复杂类型（如 Enum, Struct）能正确序列化到请求体或查询参数。
4. **链式调用**: 验证 Builder 链式调用的连贯性。

### 代码示例 (参考模式)

```rust
#[cfg(test)]
mod create_employee_builder_tests {
    use super::*;
    use open_lark::service::hr::corehr::v1::employee::*;

    #[test]
    fn test_builder_query_params() {
        let request = CreateEmployeeRequest::builder()
            .client_token("test-token")
            .user_id_type("open_id")
            .build();
            
        assert_eq!(request.api_req.query_params.get("client_token"), Some(&"test-token".to_string()));
        assert_eq!(request.api_req.query_params.get("user_id_type"), Some(&"open_id".to_string()));
    }

    #[test]
    fn test_builder_body_serialization() {
        let body = CreateEmployeeRequestBody::builder()
            .name("张三")
            .mobile("13800138000")
            .build();
            
        let request = CreateEmployeeRequest::builder()
            .request_body(body)
            .build();
            
        let parsed_body: CreateEmployeeRequestBody = serde_json::from_slice(&request.api_req.body).unwrap();
        assert_eq!(parsed_body.name, "张三");
    }
}
```

## 4. Fixture 使用指南

为了避免在测试代码中硬编码大量的长 JSON 字符串，应使用 Fixture 文件。

### 管理方式
- **位置**: `tests/fixtures/hr/{module}/{version}/{api_name}_{scenario}.json`
- **读取**: 使用 `include_str!` 宏在编译时载入。

### 静态 JSON vs 动态生成
- **静态**: 复杂的响应报文推荐使用文件存放。
- **动态**: 简单的请求体推荐使用 `serde_json::json!` 宏动态生成。

## 5. 代码审查 Checklist

提交 HR 模块测试 PR 时，请检查：
- [ ] 测试覆盖了核心 API 的成功场景。
- [ ] 边界测试：空字符串、特殊字符、极大/极小数值。
- [ ] Builder 验证：验证了所有暴露的 setter 方法。
- [ ] 错误处理：模拟 4xx/5xx 响应，验证 SDK 的 Error 类型转换。
- [ ] 无 Panic：测试代码在任何情况下均不得产生 panic。

## 6. 可运行代码示例

以下展示一个结合 `rstest` 和 `wiremock` 的完整测试示例。

```rust
use rstest::*;
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};
use open_lark::prelude::*;
use open_lark::service::hr::corehr::v1::employee::*;

#[tokio::test]
async fn test_get_employee_integration() {
    // 1. 启动 Mock Server
    let mock_server = MockServer::start().await;
    
    // 2. 配置预期行为
    let response_body = serde_json::json!({
        "code": 0,
        "msg": "success",
        "data": {
            "employee": {
                "employee_id": "emp_123",
                "name": "Test User"
            }
        }
    });
    
    Mock::given(method("GET"))
        .and(path("/open-apis/corehr/v1/employees/emp_123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .mount(&mock_server)
        .await;

    // 3. 构造 Client 并执行
    let config = Config::builder()
        .app_id("id")
        .app_secret("secret")
        .base_url(mock_server.uri())
        .build();
    let client = Client::with_core_config(config).unwrap();
    
    let resp = client.hr.corehr.v1().employee().get("emp_123").execute().await.unwrap();
    
    // 4. 断言结果
    assert_eq!(resp.employee.employee_id, "emp_123");
    assert_eq!(resp.employee.name, "Test User");
}

#[rstest]
#[case("open_id")]
#[case("user_id")]
fn test_employee_id_types(#[case] id_type: &str) {
    let request = GetEmployeeRequest::builder()
        .user_id_type(id_type)
        .build();
    assert_eq!(request.api_req.query_params.get("user_id_type"), Some(&id_type.to_string()));
}
```

---
**注意**: 编写测试时应优先参考 `tests/unit/im/builder_tests.rs` 中的 Builder 测试模式。
