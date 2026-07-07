//! 更新卡片实体配置
//!
//! docPath: <https://open.feishu.cn/document/cardkit-v1/card/settings>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::{
        api_utils::{extract_response_data, serialize_params},
        validation::validate_card_id,
    },
    endpoints::cardkit_v1_card_settings,
};

/// 更新卡片实体配置请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCardSettingsBody {
    /// 卡片 ID
    pub card_id: String,
    /// 设置内容（结构以官方文档为准）
    pub settings: serde_json::Value,
}

/// 更新卡片实体配置响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCardSettingsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 卡片 ID。
    pub card_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 应用 ID。
    pub app_id: Option<String>,
}

impl openlark_core::api::ApiResponseTrait for UpdateCardSettingsResponse {}

/// 更新卡片实体配置请求
#[derive(Debug, Clone)]
pub struct UpdateCardSettingsRequest {
    config: Config,
    card_id: Option<String>,
    settings: Option<serde_json::Value>,
}

impl UpdateCardSettingsRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            card_id: None,
            settings: None,
        }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card/settings>
    pub async fn execute(
        self,
        body: UpdateCardSettingsBody,
    ) -> SDKResult<UpdateCardSettingsResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（支持自定义选项）
    ///
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card/settings>
    pub async fn execute_with_options(
        self,
        body: UpdateCardSettingsBody,
        option: RequestOption,
    ) -> SDKResult<UpdateCardSettingsResponse> {
        let mut body = body;
        if let Some(card_id) = self.card_id {
            body.card_id = card_id;
        }
        if let Some(settings) = self.settings {
            body.settings = settings;
        }

        validate_card_id(&body.card_id)?;

        // url: PATCH:/open-apis/cardkit/v1/cards/:card_id/settings
        let url = cardkit_v1_card_settings(&body.card_id);
        let req: ApiRequest<UpdateCardSettingsResponse> =
            ApiRequest::patch(url).body(serialize_params(&body, "更新卡片实体配置")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "更新卡片实体配置")
    }
}

/// 更新卡片实体配置请求构建器
#[derive(Debug, Clone)]
pub struct UpdateCardSettingsRequestBuilder {
    request: UpdateCardSettingsRequest,
    card_id: Option<String>,
    settings: Option<serde_json::Value>,
}

impl UpdateCardSettingsRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: UpdateCardSettingsRequest::new(config),
            card_id: None,
            settings: None,
        }
    }

    /// 设置卡片 ID
    pub fn card_id(mut self, card_id: impl Into<String>) -> Self {
        self.card_id = Some(card_id.into());
        self
    }

    /// 设置配置
    pub fn settings(mut self, settings: impl Into<serde_json::Value>) -> Self {
        self.settings = Some(settings.into());
        self
    }

    /// 构建请求
    pub fn build(self) -> UpdateCardSettingsRequest {
        UpdateCardSettingsRequest {
            config: self.request.config,
            card_id: self.card_id,
            settings: self.settings,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PATCH .../cards/{card_id}/settings + body 序列化 → UpdateCardSettingsResponse。
    #[tokio::test]
    async fn test_update_card_settings_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/cardkit/v1/cards/card_001/settings"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "card_id": "card_001", "app_id": "app_001" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let body = UpdateCardSettingsBody {
            card_id: "card_001".into(),
            settings: json!({ "sharing": { "permission": "anyone_can_edit" } }),
        };
        let resp = UpdateCardSettingsRequest::new(config)
            .execute(body)
            .await
            .expect("更新卡片实体配置应成功");
        assert_eq!(resp.app_id.as_deref(), Some("app_001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["settings"]["sharing"]["permission"], "anyone_can_edit");
    }
}
