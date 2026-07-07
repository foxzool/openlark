//! 获取应用反馈列表
//! docPath: <https://open.feishu.cn/document/application-v6/admin/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取应用反馈列表的请求。
#[derive(Debug, Clone)]
pub struct ListApplicationFeedbackRequest {
    config: Arc<Config>,
    app_id: String,
}

/// 获取应用反馈列表的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListApplicationFeedbackResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for ListApplicationFeedbackResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListApplicationFeedbackRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
        }
    }

    /// 执行获取应用反馈列表请求。
    pub async fn execute(self) -> SDKResult<ListApplicationFeedbackResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListApplicationFeedbackResponse> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/feedbacks",
            self.app_id
        );
        let req: ApiRequest<ListApplicationFeedbackResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取应用反馈列表", "响应数据为空")
        })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../applications/{app_id}/feedbacks → 强类型
    /// ListApplicationFeedbackResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_list_feedback_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/application/v6/applications/cli_test_app/feedbacks",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "total": 3 } }
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

        let resp = ListApplicationFeedbackRequest::new(config, "cli_test_app")
            .execute()
            .await
            .expect("获取应用反馈列表应成功");
        assert_eq!(resp.data.unwrap()["total"], 3);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/application/v6/applications/cli_test_app/feedbacks"
        );
    }
}
