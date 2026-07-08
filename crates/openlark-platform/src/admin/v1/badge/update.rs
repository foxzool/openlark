//! 修改勋章信息 API
//! docPath: <https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/update>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 修改勋章信息的请求构建器。
pub struct UpdateBadgeRequestBuilder {
    badge_id: String,
    name: Option<String>,
    description: Option<String>,
    config: Config,
}

impl UpdateBadgeRequestBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            badge_id: String::new(),
            name: None,
            description: None,
            config,
        }
    }

    /// 设置勋章 ID。
    pub fn badge_id(mut self, badge_id: impl Into<String>) -> Self {
        self.badge_id = badge_id.into();
        self
    }

    /// 设置名称。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置描述。
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<UpdateBadgeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UpdateBadgeResponse> {
        let url = format!("/open-apis/admin/v1/badges/{}", self.badge_id);
        let request_body = UpdateBadgeRequest {
            name: self.name,
            description: self.description,
        };
        let api_request: ApiRequest<UpdateBadgeResponse> =
            ApiRequest::put(url).body(serde_json::to_value(&request_body)?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("修改勋章信息", "响应数据为空"))
    }
}

#[derive(Debug, Serialize)]
struct UpdateBadgeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// 修改勋章信息的响应。
pub struct UpdateBadgeResponse {
    /// 勋章 ID。
    pub badge_id: String,
    /// 名称。
    pub name: String,
    /// 描述。
    pub description: Option<String>,
    /// 更新时间。
    pub update_time: String,
}

impl ApiResponseTrait for UpdateBadgeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to UpdateBadgeRequestBuilder, will be removed in v1.0 (#271)")]
pub type UpdateBadgeBuilder = UpdateBadgeRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：PUT .../admin/v1/badges/{badge_id} → 强类型 UpdateBadgeResponse。
    #[tokio::test]
    async fn test_update_badge_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/open-apis/admin/v1/badges/badge_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "badge_id": "badge_001",
                    "name": "优秀员工",
                    "description": "更新后的描述",
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

        let resp = UpdateBadgeRequestBuilder::new(config)
            .badge_id("badge_001")
            .name("优秀员工")
            .description("更新后的描述")
            .execute()
            .await
            .expect("修改勋章信息应成功");
        assert_eq!(resp.badge_id, "badge_001");
        assert_eq!(resp.name, "优秀员工");
        assert_eq!(resp.update_time, "1717000000");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].method, "PUT");
        assert_eq!(
            received[0].url.path(),
            "/open-apis/admin/v1/badges/badge_001"
        );
    }
}
