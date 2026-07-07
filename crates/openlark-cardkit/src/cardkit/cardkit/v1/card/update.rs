//! 全量更新卡片实体
//!
//! docPath: <https://open.feishu.cn/document/cardkit-v1/card/update>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::{
        api_utils::{extract_response_data, serialize_params},
        validation::validate_card_id,
    },
    endpoints::cardkit_v1_card,
};

/// 全量更新卡片实体请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCardBody {
    /// 卡片 ID
    pub card_id: String,
    /// 卡片内容
    pub card_content: serde_json::Value,
    /// 卡片类型（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 卡片类型。
    pub card_type: Option<String>,
    /// 更新掩码（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 更新字段掩码。
    pub update_mask: Option<Vec<String>>,
}

/// 全量更新卡片实体响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCardResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 卡片 ID。
    pub card_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 应用 ID。
    pub app_id: Option<String>,
}

impl openlark_core::api::ApiResponseTrait for UpdateCardResponse {}

/// 全量更新卡片实体请求
#[derive(Debug, Clone)]
pub struct UpdateCardRequest {
    config: Config,
    card_id: Option<String>,
    card_content: Option<serde_json::Value>,
    card_type: Option<String>,
    update_mask: Option<Vec<String>>,
}

impl UpdateCardRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            card_id: None,
            card_content: None,
            card_type: None,
            update_mask: None,
        }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card/update>
    pub async fn execute(self, body: UpdateCardBody) -> SDKResult<UpdateCardResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（支持自定义选项）
    ///
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card/update>
    pub async fn execute_with_options(
        self,
        body: UpdateCardBody,
        option: RequestOption,
    ) -> SDKResult<UpdateCardResponse> {
        let mut body = body;
        if let Some(card_id) = self.card_id {
            body.card_id = card_id;
        }
        if let Some(card_content) = self.card_content {
            body.card_content = card_content;
        }
        if let Some(card_type) = self.card_type {
            body.card_type = Some(card_type);
        }
        if let Some(update_mask) = self.update_mask {
            body.update_mask = Some(update_mask);
        }

        validate_card_id(&body.card_id)?;

        // url: PUT:/open-apis/cardkit/v1/cards/:card_id
        let url = cardkit_v1_card(&body.card_id);
        let req: ApiRequest<UpdateCardResponse> =
            ApiRequest::put(url).body(serialize_params(&body, "全量更新卡片实体")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "全量更新卡片实体")
    }
}

/// 全量更新卡片实体请求构建器
#[derive(Debug, Clone)]
pub struct UpdateCardRequestBuilder {
    request: UpdateCardRequest,
    card_id: Option<String>,
    card_content: Option<serde_json::Value>,
    card_type: Option<String>,
    update_mask: Option<Vec<String>>,
}

impl UpdateCardRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: UpdateCardRequest::new(config),
            card_id: None,
            card_content: None,
            card_type: None,
            update_mask: None,
        }
    }

    /// 设置卡片 ID
    pub fn card_id(mut self, card_id: impl Into<String>) -> Self {
        self.card_id = Some(card_id.into());
        self
    }

    /// 设置卡片内容
    pub fn card_content(mut self, card_content: impl Into<serde_json::Value>) -> Self {
        self.card_content = Some(card_content.into());
        self
    }

    /// 设置卡片类型
    pub fn card_type(mut self, card_type: impl Into<String>) -> Self {
        self.card_type = Some(card_type.into());
        self
    }

    /// 设置更新掩码
    pub fn update_mask(mut self, update_mask: impl Into<Vec<String>>) -> Self {
        self.update_mask = Some(update_mask.into());
        self
    }

    /// 构建请求
    pub fn build(self) -> UpdateCardRequest {
        UpdateCardRequest {
            config: self.request.config,
            card_id: self.card_id,
            card_content: self.card_content,
            card_type: self.card_type,
            update_mask: self.update_mask,
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

    /// 端到端：PUT .../cards/{card_id} + body 序列化 → UpdateCardResponse。
    #[tokio::test]
    async fn test_update_card_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/open-apis/cardkit/v1/cards/card_001"))
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

        let body = UpdateCardBody {
            card_id: "card_001".into(),
            card_content: json!({ "schema": "2.0" }),
            card_type: None,
            update_mask: None,
        };
        let resp = UpdateCardRequest::new(config)
            .execute(body)
            .await
            .expect("全量更新卡片实体应成功");
        assert_eq!(resp.card_id.as_deref(), Some("card_001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["card_content"]["schema"], "2.0");
    }
}
