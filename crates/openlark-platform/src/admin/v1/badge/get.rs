//! 获取勋章详情 API
//!
//! API文档: <https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/get>
//! docPath: <https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 获取勋章详情请求
pub struct GetBadgeRequestBuilder {
    badge_id: String,
    config: Config,
}

impl GetBadgeRequestBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            badge_id: String::new(),
            config,
        }
    }

    /// 设置勋章 ID。
    pub fn badge_id(mut self, badge_id: impl Into<String>) -> Self {
        self.badge_id = badge_id.into();
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<GetBadgeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<GetBadgeResponse> {
        let api_request: ApiRequest<GetBadgeResponse> =
            ApiRequest::get(format!("/open-apis/admin/v1/badges/{}", self.badge_id));

        Transport::request_typed(api_request, &self.config, Some(option), "获取勋章详情").await
    }
}

/// 获取勋章详情响应
#[derive(Debug, Clone, Deserialize, Serialize)]
/// 获取勋章详情的响应。
pub struct GetBadgeResponse {
    /// 勋章 ID。
    pub badge_id: String,
    /// 名称。
    pub name: String,
    /// 描述。
    pub description: Option<String>,
    /// 图标地址。
    pub icon_url: Option<String>,
    /// 创建时间。
    pub create_time: String,
}

impl ApiResponseTrait for GetBadgeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to GetBadgeRequestBuilder, will be removed in v1.0 (#271)")]
pub type GetBadgeBuilder = GetBadgeRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../admin/v1/badges/{badge_id} → 强类型 GetBadgeResponse。
    #[tokio::test]
    async fn test_get_badge_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/admin/v1/badges/badge_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "badge_id": "badge_001",
                    "name": "优秀员工",
                    "description": "季度优秀员工勋章",
                    "icon_url": "https://example.com/icon.png",
                    "create_time": "1717000000"
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

        let resp = GetBadgeRequestBuilder::new(config)
            .badge_id("badge_001")
            .execute()
            .await
            .expect("获取勋章详情应成功");
        assert_eq!(resp.badge_id, "badge_001");
        assert_eq!(resp.name, "优秀员工");
        assert_eq!(resp.create_time, "1717000000");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/admin/v1/badges/badge_001"
        );
    }
}
