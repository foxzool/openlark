//! 更新应用红点
//! docPath: <https://open.feishu.cn/document/server-docs/application-v6/app_badge/set>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 更新应用红点的请求。
#[derive(Debug, Clone)]
pub struct SetAppBadgeRequest {
    config: Arc<Config>,
    body: SetAppBadgeBody,
}

/// 更新应用红点的请求体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetAppBadgeBody {
    /// 应用 ID。
    pub app_id: String,
    /// 徽标。
    pub badge: i32,
}

/// 更新应用红点的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetAppBadgeResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for SetAppBadgeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SetAppBadgeRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: SetAppBadgeBody::default(),
        }
    }

    /// 设置应用 ID。
    pub fn app_id(mut self, id: impl Into<String>) -> Self {
        self.body.app_id = id.into();
        self
    }

    /// 设置徽标。
    pub fn badge(mut self, badge: i32) -> Self {
        self.body.badge = badge;
        self
    }

    /// 执行更新应用红点请求。
    pub async fn execute(self) -> SDKResult<SetAppBadgeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SetAppBadgeResponse> {
        let path = "/open-apis/application/v6/app_badge/set";
        let body = serde_json::to_value(&self.body)?;
        let req: ApiRequest<SetAppBadgeResponse> = ApiRequest::post(path).body(body);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        Ok(resp.data.unwrap_or(SetAppBadgeResponse { data: None }))
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：POST .../app_badge/set → 强类型 SetAppBadgeResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_set_app_badge_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/application/v6/app_badge/set"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "app_id": "cli_test_app", "badge": 5 } }
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

        let resp = SetAppBadgeRequest::new(config)
            .app_id("cli_test_app")
            .badge(5)
            .execute()
            .await
            .expect("更新应用红点应成功");
        assert_eq!(resp.data.unwrap()["app_id"], "cli_test_app");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/application/v6/app_badge/set"
        );
    }
}
