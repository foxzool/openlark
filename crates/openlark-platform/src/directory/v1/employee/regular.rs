//! 更新待离职成员为在职
//!
//! 文档: <https://open.feishu.cn/document/directory-v1/employee/regular>
//! docPath: <https://open.feishu.cn/document/directory-v1/employee/regular>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 更新待离职成员为在职 Builder
#[derive(Debug, Clone)]
pub struct EmployeeRegularRequestBuilder {
    config: Config,
    /// 员工 ID
    employee_id: String,
}

impl EmployeeRegularRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, employee_id: impl Into<String>) -> Self {
        Self {
            config,
            employee_id: employee_id.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EmployeeRegularResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<EmployeeRegularResponse> {
        let url = format!(
            "/open-apis/directory/v1/employees/{}/regular",
            self.employee_id
        );

        let req: ApiRequest<EmployeeRegularResponse> = ApiRequest::patch(&url);
        Transport::request_typed(req, &self.config, Some(option), "更新待离职成员为在职").await
    }
}

/// 更新待离职成员为在职响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeRegularResponse {
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

impl ApiResponseTrait for EmployeeRegularResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to EmployeeRegularRequestBuilder, will be removed in v1.0 (#271)")]
pub type EmployeeRegularBuilder = EmployeeRegularRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：PATCH .../employees/{id}/regular → 强类型 EmployeeRegularResponse。
    #[tokio::test]
    async fn test_regular_employee_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/directory/v1/employees/emp_001/regular"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "employee_id": "emp_001",
                    "status": "regular",
                    "message": "updated to regular"
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

        let resp = EmployeeRegularRequestBuilder::new(config, "emp_001")
            .execute()
            .await
            .expect("更新待离职成员为在职应成功");
        assert_eq!(resp.employee_id, "emp_001");
        assert_eq!(resp.status, "regular");
        assert_eq!(resp.message, "updated to regular");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/directory/v1/employees/emp_001/regular"
        );
        assert_eq!(received[0].method, "PATCH");
    }
}
