//! 更新在职员工为待离职
//!
//! 文档: <https://open.feishu.cn/document/directory-v1/employee/to_be_resigned>
//! docPath: <https://open.feishu.cn/document/directory-v1/employee/to_be_resigned>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 更新在职员工为待离职 Builder
#[derive(Debug, Clone)]
pub struct EmployeeToBeResignedRequestBuilder {
    config: Config,
    /// 员工 ID
    employee_id: String,
    /// 离职原因
    reason: Option<String>,
}

impl EmployeeToBeResignedRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, employee_id: impl Into<String>) -> Self {
        Self {
            config,
            employee_id: employee_id.into(),
            reason: None,
        }
    }

    /// 设置离职原因
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EmployeeToBeResignedResponse> {
        let url = format!(
            "/open-apis/directory/v1/employees/{}/to_be_resigned",
            self.employee_id
        );

        let request = EmployeeToBeResignedRequest {
            reason: self.reason,
        };

        let req: ApiRequest<EmployeeToBeResignedResponse> =
            ApiRequest::patch(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(RequestOption::default())).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<EmployeeToBeResignedResponse> {
        let url = format!(
            "/open-apis/directory/v1/employees/{}/to_be_resigned",
            self.employee_id
        );

        let request = EmployeeToBeResignedRequest {
            reason: self.reason,
        };

        let req: ApiRequest<EmployeeToBeResignedResponse> =
            ApiRequest::patch(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 更新在职员工为待离职请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct EmployeeToBeResignedRequest {
    /// 离职原因
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
}

/// 更新在职员工为待离职响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeToBeResignedResponse {
    /// 员工 ID
    #[serde(rename = "employee_id")]
    pub employee_id: String,
    /// 状态
    #[serde(rename = "status")]
    pub status: String,
    /// 结果消息
    #[serde(rename = "message")]
    pub message: String,
}

impl ApiResponseTrait for EmployeeToBeResignedResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(
    note = "renamed to EmployeeToBeResignedRequestBuilder, will be removed in v1.0 (#271)"
)]
pub type EmployeeToBeResignedBuilder = EmployeeToBeResignedRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：PATCH .../employees/{id}/to_be_resigned → 强类型 EmployeeToBeResignedResponse。
    #[tokio::test]
    async fn test_to_be_resigned_employee_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path(
                "/open-apis/directory/v1/employees/emp_001/to_be_resigned",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "employee_id": "emp_001",
                    "status": "to_be_resigned",
                    "message": "updated to to_be_resigned"
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

        let resp = EmployeeToBeResignedRequestBuilder::new(config, "emp_001")
            .reason("contract ended")
            .execute()
            .await
            .expect("更新在职员工为待离职应成功");
        assert_eq!(resp.employee_id, "emp_001");
        assert_eq!(resp.status, "to_be_resigned");
        assert_eq!(resp.message, "updated to to_be_resigned");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/directory/v1/employees/emp_001/to_be_resigned"
        );
        assert_eq!(received[0].method, "PATCH");
    }
}
