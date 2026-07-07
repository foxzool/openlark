//! 更新组件
//!
//! docPath: <https://open.feishu.cn/document/cardkit-v1/card-element/update>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use super::models::UpdateCardElementResponse;
use crate::common::{
    api_utils::{extract_response_data, serialize_params},
    validation::{validate_card_id, validate_element_id},
};
use crate::endpoints::cardkit_v1_card_element;

/// 更新组件属性请求体（结构以官方文档为准）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateCardElementBody {
    /// 卡片 ID
    pub card_id: String,
    /// 组件 ID
    pub element_id: String,
    /// 属性
    pub patch: serde_json::Value,
}

/// 更新组件请求
#[derive(Debug, Clone)]
pub struct UpdateCardElementRequest {
    config: Config,
    card_id: Option<String>,
    element_id: Option<String>,
    patch: Option<serde_json::Value>,
}

impl UpdateCardElementRequest {
    /// 创建新的更新组件请求。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            card_id: None,
            element_id: None,
            patch: None,
        }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card-element/update>
    pub async fn execute(
        self,
        body: UpdateCardElementBody,
    ) -> SDKResult<UpdateCardElementResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（支持自定义选项）
    ///
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card-element/update>
    pub async fn execute_with_options(
        self,
        body: UpdateCardElementBody,
        option: RequestOption,
    ) -> SDKResult<UpdateCardElementResponse> {
        let mut body = body;
        if let Some(card_id) = self.card_id {
            body.card_id = card_id;
        }
        if let Some(element_id) = self.element_id {
            body.element_id = element_id;
        }
        if let Some(patch) = self.patch {
            body.patch = patch;
        }

        validate_card_id(&body.card_id)?;
        validate_element_id(&body.element_id)?;

        // url: PUT:/open-apis/cardkit/v1/cards/:card_id/elements/:element_id
        let req: ApiRequest<UpdateCardElementResponse> =
            ApiRequest::put(cardkit_v1_card_element(&body.card_id, &body.element_id))
                .body(serialize_params(&body, "更新组件属性")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "更新组件属性")
    }
}

/// 更新组件请求构建器
#[derive(Debug, Clone)]
pub struct UpdateCardElementRequestBuilder {
    request: UpdateCardElementRequest,
    card_id: Option<String>,
    element_id: Option<String>,
    patch: Option<serde_json::Value>,
}

impl UpdateCardElementRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: UpdateCardElementRequest::new(config),
            card_id: None,
            element_id: None,
            patch: None,
        }
    }

    /// 设置卡片 ID
    pub fn card_id(mut self, card_id: impl Into<String>) -> Self {
        self.card_id = Some(card_id.into());
        self
    }

    /// 设置组件 ID
    pub fn element_id(mut self, element_id: impl Into<String>) -> Self {
        self.element_id = Some(element_id.into());
        self
    }

    /// 设置属性
    pub fn patch(mut self, patch: impl Into<serde_json::Value>) -> Self {
        self.patch = Some(patch.into());
        self
    }

    /// 构建请求
    pub fn build(self) -> UpdateCardElementRequest {
        UpdateCardElementRequest {
            config: self.request.config,
            card_id: self.card_id,
            element_id: self.element_id,
            patch: self.patch,
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

    /// 端到端：PUT .../cards/{card_id}/elements/{element_id} + body 序列化 → UpdateCardElementResponse。
    #[tokio::test]
    async fn test_update_card_element_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path(
                "/open-apis/cardkit/v1/cards/card_001/elements/elem_001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "card_id": "card_001", "element_id": "elem_001" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let body = UpdateCardElementBody {
            card_id: "card_001".into(),
            element_id: "elem_001".into(),
            patch: json!({ "tag": "div" }),
        };
        let resp = UpdateCardElementRequest::new(config)
            .execute(body)
            .await
            .expect("更新组件属性应成功");
        assert_eq!(resp.card_id.as_deref(), Some("card_001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["patch"]["tag"], "div");
    }
}
