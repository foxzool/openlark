//! 更新员工信息
//!
//! 文档: <https://open.feishu.cn/document/directory-v1/employee/patch>
//! docPath: <https://open.feishu.cn/document/directory-v1/employee/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 更新员工信息 Builder
#[derive(Debug, Clone)]
pub struct EmployeePatchRequestBuilder {
    config: Config,
    /// 员工 ID
    employee_id: String,
    /// 员工名称
    name: Option<String>,
    /// 邮箱
    email: Option<String>,
    /// 部门 ID 列表
    department_ids: Vec<String>,
}

impl EmployeePatchRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, employee_id: impl Into<String>) -> Self {
        Self {
            config,
            employee_id: employee_id.into(),
            name: None,
            email: None,
            department_ids: Vec::new(),
        }
    }

    /// 设置员工名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置邮箱
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    /// 添加部门 ID
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.department_ids.push(department_id.into());
        self
    }

    /// 添加多个部门 ID
    pub fn department_ids(
        mut self,
        department_ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.department_ids
            .extend(department_ids.into_iter().map(Into::into));
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EmployeePatchResponse> {
        let url = format!("/open-apis/directory/v1/employees/{}", self.employee_id);

        let request = EmployeePatchRequest {
            name: self.name,
            email: self.email,
            department_ids: self.department_ids,
        };

        let req: ApiRequest<EmployeePatchResponse> =
            ApiRequest::patch(&url).body(serde_json::to_value(&request)?);
        Transport::request_typed(
            req,
            &self.config,
            Some(RequestOption::default()),
            "Operation",
        )
        .await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<EmployeePatchResponse> {
        let url = format!("/open-apis/directory/v1/employees/{}", self.employee_id);

        let request = EmployeePatchRequest {
            name: self.name,
            email: self.email,
            department_ids: self.department_ids,
        };

        let req: ApiRequest<EmployeePatchResponse> =
            ApiRequest::patch(&url).body(serde_json::to_value(&request)?);
        Transport::request_typed(req, &self.config, Some(option), "Operation").await
    }
}

/// 更新员工信息请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct EmployeePatchRequest {
    /// 员工名称
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// 邮箱
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    /// 部门 ID 列表
    #[serde(rename = "department_ids", skip_serializing_if = "Vec::is_empty")]
    department_ids: Vec<String>,
}

/// 更新员工信息响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeePatchResponse {
    /// 员工 ID
    #[serde(rename = "employee_id")]
    pub employee_id: String,
    /// 更新后的名称
    #[serde(rename = "name")]
    pub name: String,
    /// 更新时间
    #[serde(rename = "updated_at")]
    pub updated_at: i64,
}

impl ApiResponseTrait for EmployeePatchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to EmployeePatchRequestBuilder, will be removed in v1.0 (#271)")]
pub type EmployeePatchBuilder = EmployeePatchRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：PATCH .../employees/{id} → 强类型 EmployeePatchResponse。
    #[tokio::test]
    async fn test_patch_employee_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/directory/v1/employees/emp_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "employee_id": "emp_001",
                    "name": "alice_updated",
                    "updated_at": 1700000000
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

        let resp = EmployeePatchRequestBuilder::new(config, "emp_001")
            .name("alice_updated")
            .email("alice@example.com")
            .execute()
            .await
            .expect("更新员工信息应成功");
        assert_eq!(resp.employee_id, "emp_001");
        assert_eq!(resp.name, "alice_updated");
        assert_eq!(resp.updated_at, 1700000000);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/directory/v1/employees/emp_001"
        );
        assert_eq!(received[0].method, "PATCH");
    }
}
