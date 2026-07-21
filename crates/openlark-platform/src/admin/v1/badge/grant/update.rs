//! 修改勋章授予名单 API
//! docPath: <https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/update>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 修改勋章授予名单的请求构建器。
pub struct UpdateBadgeGrantRequestBuilder {
    badge_id: String,
    grant_id: String,
    user_ids: Vec<String>,
    config: Config,
}

impl UpdateBadgeGrantRequestBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            badge_id: String::new(),
            grant_id: String::new(),
            user_ids: Vec::new(),
            config,
        }
    }

    /// 设置勋章 ID。
    pub fn badge_id(mut self, badge_id: impl Into<String>) -> Self {
        self.badge_id = badge_id.into();
        self
    }

    /// 设置授予记录 ID。
    pub fn grant_id(mut self, grant_id: impl Into<String>) -> Self {
        self.grant_id = grant_id.into();
        self
    }

    /// 设置用户 ID 列表。
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = user_ids;
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<UpdateBadgeGrantResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UpdateBadgeGrantResponse> {
        let url = format!(
            "/open-apis/admin/v1/badges/{}/grants/{}",
            self.badge_id, self.grant_id
        );
        let request_body = UpdateBadgeGrantRequest {
            user_ids: self.user_ids,
        };
        let api_request: ApiRequest<UpdateBadgeGrantResponse> =
            ApiRequest::put(url).body(serde_json::to_value(&request_body)?);

        Transport::request_typed(api_request, &self.config, Some(option), "修改勋章授予名单").await
    }
}

#[derive(Debug, Serialize)]
struct UpdateBadgeGrantRequest {
    user_ids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// 修改勋章授予名单的响应。
pub struct UpdateBadgeGrantResponse {
    /// 授予记录 ID。
    pub grant_id: String,
    /// 勋章 ID。
    pub badge_id: String,
    /// 用户 ID 列表。
    pub user_ids: Vec<String>,
    /// 更新时间。
    pub update_time: String,
}

impl ApiResponseTrait for UpdateBadgeGrantResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to UpdateBadgeGrantRequestBuilder, will be removed in v1.0 (#271)")]
pub type UpdateBadgeGrantBuilder = UpdateBadgeGrantRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：PUT .../badges/{badge_id}/grants/{grant_id} → 强类型 UpdateBadgeGrantResponse。
    #[tokio::test]
    async fn test_update_badge_grant_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path(
                "/open-apis/admin/v1/badges/badge_001/grants/grant_001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "grant_id": "grant_001",
                    "badge_id": "badge_001",
                    "user_ids": ["u_001", "u_002"],
                    "update_time": "1717000000"
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

        let resp = UpdateBadgeGrantRequestBuilder::new(config)
            .badge_id("badge_001")
            .grant_id("grant_001")
            .user_ids(vec!["u_001".to_string(), "u_002".to_string()])
            .execute()
            .await
            .expect("修改勋章授予名单应成功");
        assert_eq!(resp.grant_id, "grant_001");
        assert_eq!(resp.badge_id, "badge_001");
        assert_eq!(
            resp.user_ids,
            vec!["u_001".to_string(), "u_002".to_string()]
        );
        assert_eq!(resp.update_time, "1717000000");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].method, "PUT");
        assert_eq!(
            received[0].url.path(),
            "/open-apis/admin/v1/badges/badge_001/grants/grant_001"
        );
    }
}
