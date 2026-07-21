//! 批量获取员工信息
//!
//! 文档: <https://open.feishu.cn/document/directory-v1/employee/mget>
//! docPath: <https://open.feishu.cn/document/directory-v1/employee/mget>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 批量获取员工信息 Builder
#[derive(Debug, Clone)]
pub struct EmployeeMgetRequestBuilder {
    config: Config,
    /// 员工 ID 列表
    employee_ids: Vec<String>,
}

impl EmployeeMgetRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            employee_ids: Vec::new(),
        }
    }

    /// 添加员工 ID
    pub fn employee_id(mut self, employee_id: impl Into<String>) -> Self {
        self.employee_ids.push(employee_id.into());
        self
    }

    /// 添加多个员工 ID
    pub fn employee_ids(
        mut self,
        employee_ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.employee_ids
            .extend(employee_ids.into_iter().map(Into::into));
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EmployeeMgetResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<EmployeeMgetResponse> {
        let url = "/open-apis/directory/v1/employees/mget".to_string();

        let request = EmployeeMgetRequest {
            employee_ids: self.employee_ids,
        };

        let req: ApiRequest<EmployeeMgetResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        Transport::request_typed(req, &self.config, Some(option), "Operation").await
    }
}

/// 批量获取员工信息请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct EmployeeMgetRequest {
    /// 员工 ID 列表
    #[serde(rename = "employee_ids")]
    employee_ids: Vec<String>,
}

/// 员工信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeInfo {
    /// 员工 ID
    #[serde(rename = "employee_id")]
    employee_id: String,
    /// 员工名称
    #[serde(rename = "name")]
    name: String,
    /// 手机号
    #[serde(rename = "mobile")]
    mobile: String,
    /// 邮箱
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    /// 部门 ID 列表
    #[serde(rename = "department_ids")]
    department_ids: Vec<String>,
    /// 状态
    #[serde(rename = "status")]
    status: String,
}

/// 批量获取员工信息响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeMgetResponse {
    /// 员工信息列表
    #[serde(rename = "items")]
    pub items: Vec<EmployeeInfo>,
}

impl ApiResponseTrait for EmployeeMgetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to EmployeeMgetRequestBuilder, will be removed in v1.0 (#271)")]
pub type EmployeeMgetBuilder = EmployeeMgetRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../employees/mget → 强类型 EmployeeMgetResponse。
    #[tokio::test]
    async fn test_mget_employees_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/directory/v1/employees/mget"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "employee_id": "emp_001",
                            "name": "alice",
                            "mobile": "13800000001",
                            "email": "alice@example.com",
                            "department_ids": ["dept_001"],
                            "status": "active"
                        },
                        {
                            "employee_id": "emp_002",
                            "name": "bob",
                            "mobile": "13800000002",
                            "department_ids": ["dept_002"],
                            "status": "active"
                        }
                    ]
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = EmployeeMgetRequestBuilder::new(config)
            .employee_ids(["emp_001", "emp_002"])
            .execute()
            .await
            .expect("批量获取员工信息应成功");
        assert_eq!(resp.items.len(), 2);
        assert_eq!(resp.items[0].employee_id, "emp_001");
        assert_eq!(resp.items[0].name, "alice");
        assert_eq!(resp.items[1].employee_id, "emp_002");
        assert_eq!(resp.items[1].email, None);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/directory/v1/employees/mget"
        );
        assert_eq!(received[0].method, "POST");
    }
}
