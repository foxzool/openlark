//! 恢复离职员工
//!
//! 文档: <https://open.feishu.cn/document/directory-v1/employee/resurrect>
//! docPath: <https://open.feishu.cn/document/directory-v1/employee/resurrect>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 恢复离职员工 Builder
#[derive(Debug, Clone)]
pub struct EmployeeResurrectRequestBuilder {
    config: Config,
    /// 员工 ID
    employee_id: String,
}

impl EmployeeResurrectRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, employee_id: impl Into<String>) -> Self {
        Self {
            config,
            employee_id: employee_id.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EmployeeResurrectResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<EmployeeResurrectResponse> {
        let url = format!(
            "/open-apis/directory/v1/employees/{}/resurrect",
            self.employee_id
        );

        let req: ApiRequest<EmployeeResurrectResponse> = ApiRequest::post(&url);
        Transport::request_typed(req, &self.config, Some(option), "恢复离职员工").await
    }
}

/// 恢复离职员工响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeResurrectResponse {
    /// 员工 ID
    #[serde(rename = "employee_id")]
    pub employee_id: String,
    /// 结果消息
    #[serde(rename = "message")]
    pub message: String,
}

impl ApiResponseTrait for EmployeeResurrectResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to EmployeeResurrectRequestBuilder, will be removed in v1.0 (#271)")]
pub type EmployeeResurrectBuilder = EmployeeResurrectRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../employees/{id}/resurrect → 强类型 EmployeeResurrectResponse。
    #[tokio::test]
    async fn test_resurrect_employee_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/directory/v1/employees/emp_001/resurrect"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "employee_id": "emp_001",
                    "message": "resurrected"
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

        let resp = EmployeeResurrectRequestBuilder::new(config, "emp_001")
            .execute()
            .await
            .expect("恢复离职员工应成功");
        assert_eq!(resp.employee_id, "emp_001");
        assert_eq!(resp.message, "resurrected");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/directory/v1/employees/emp_001/resurrect"
        );
        assert_eq!(received[0].method, "POST");
    }
}
