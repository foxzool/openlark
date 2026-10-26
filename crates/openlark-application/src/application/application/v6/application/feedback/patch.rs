//! 更新应用反馈
//! docPath: <https://open.feishu.cn/document/server-docs/application-v6/application/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::CoreError,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 更新应用反馈的请求。
#[derive(Debug, Clone)]
pub struct PatchApplicationFeedbackRequest {
    config: Arc<Config>,
    app_id: String,
    resource_id: String,
    /// 查询参数 `user_id_type`（可选，用户 ID 类型，默认 open_id）。
    user_id_type: Option<String>,
    /// 查询参数 `status`（必填，反馈处理状态：0-未处理 / 1-已处理 / 2-处理中 / 3-已关闭）。
    status: Option<i32>,
    /// 查询参数 `operator_id`（必填，反馈处理人员 id，类型与 user_id_type 对应）。
    operator_id: Option<String>,
}

/// 更新应用反馈的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchApplicationFeedbackResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for PatchApplicationFeedbackResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl PatchApplicationFeedbackRequest {
    /// 创建请求实例。
    pub fn new(
        config: Arc<Config>,
        app_id: impl Into<String>,
        resource_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            app_id: app_id.into(),
            resource_id: resource_id.into(),
            user_id_type: None,
            status: None,
            operator_id: None,
        }
    }

    /// 设置用户 ID 类型（可选，默认 open_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置反馈处理状态（必填：0-未处理 / 1-已处理 / 2-处理中 / 3-已关闭）。
    pub fn status(mut self, status: i32) -> Self {
        self.status = Some(status);
        self
    }

    /// 设置反馈处理人员 id（必填，类型与 user_id_type 对应）。
    pub fn operator_id(mut self, operator_id: impl Into<String>) -> Self {
        self.operator_id = Some(operator_id.into());
        self
    }

    /// 执行更新应用反馈请求。
    pub async fn execute(self) -> SDKResult<PatchApplicationFeedbackResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<PatchApplicationFeedbackResponse> {
        // status / operator_id 官方必填。
        let status = self
            .status
            .ok_or_else(|| CoreError::validation_msg("status 不能为空"))?;
        let operator_id = self
            .operator_id
            .ok_or_else(|| CoreError::validation_msg("operator_id 不能为空"))?;
        validate_required!(operator_id, "operator_id 不能为空");

        let path = format!(
            "/open-apis/application/v6/applications/{}/feedbacks/{}",
            self.app_id, self.resource_id
        );
        let req: ApiRequest<PatchApplicationFeedbackResponse> = ApiRequest::patch(&path)
            .query_opt("user_id_type", self.user_id_type)
            .query("status", status.to_string())
            .query("operator_id", operator_id);

        Transport::request_typed(req, &self.config, Some(option), "更新应用反馈").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：PATCH .../feedbacks/{feedback_id}?status=1&operator_id=ou_op → 强类型
    /// PatchApplicationFeedbackResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_patch_feedback_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path(
                "/open-apis/application/v6/applications/cli_test_app/feedbacks/fb_123",
            ))
            .and(query_param("status", "1"))
            .and(query_param("operator_id", "ou_operator"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "feedback_id": "fb_123" } }
            })))
            .mount(&server)
            .await;

        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let resp = PatchApplicationFeedbackRequest::new(config, "cli_test_app", "fb_123")
            .status(1)
            .operator_id("ou_operator")
            .execute()
            .await
            .expect("更新应用反馈应成功");
        assert_eq!(resp.data.unwrap()["feedback_id"], "fb_123");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/application/v6/applications/cli_test_app/feedbacks/fb_123"
        );
    }
}
