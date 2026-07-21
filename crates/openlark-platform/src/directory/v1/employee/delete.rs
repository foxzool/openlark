//! 离职员工
//!
//! 文档: <https://open.feishu.cn/document/directory-v1/employee/delete>
//! docPath: <https://open.feishu.cn/document/directory-v1/employee/delete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 离职员工 Builder
#[derive(Debug, Clone)]
pub struct EmployeeDeleteRequestBuilder {
    config: Config,
    /// 员工 ID
    employee_id: String,
}

impl EmployeeDeleteRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, employee_id: impl Into<String>) -> Self {
        Self {
            config,
            employee_id: employee_id.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EmployeeDeleteResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<EmployeeDeleteResponse> {
        let url = format!("/open-apis/directory/v1/employees/{}", self.employee_id);

        let req: ApiRequest<EmployeeDeleteResponse> = ApiRequest::delete(&url);
        Transport::request_typed(req, &self.config, Some(option), "离职员工").await
    }
}

/// 离职员工响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeDeleteResponse {
    /// 员工 ID
    #[serde(rename = "employee_id")]
    pub employee_id: String,
    /// 结果消息
    #[serde(rename = "message")]
    pub message: String,
}

impl ApiResponseTrait for EmployeeDeleteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to EmployeeDeleteRequestBuilder, will be removed in v1.0 (#271)")]
pub type EmployeeDeleteBuilder = EmployeeDeleteRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：DELETE .../employees/{id} → 强类型 EmployeeDeleteResponse。
    #[tokio::test]
    async fn test_delete_employee_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/directory/v1/employees/emp_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "employee_id": "emp_001",
                    "message": "deleted"
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

        let resp = EmployeeDeleteRequestBuilder::new(config, "emp_001")
            .execute()
            .await
            .expect("离职员工应成功");
        assert_eq!(resp.employee_id, "emp_001");
        assert_eq!(resp.message, "deleted");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/directory/v1/employees/emp_001"
        );
        assert_eq!(received[0].method, "DELETE");
    }
}
