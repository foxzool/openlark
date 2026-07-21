//! 更新组件属性
//!
//! docPath: <https://open.feishu.cn/document/cardkit-v1/card-element/patch>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use super::models::PatchCardElementResponse;
use crate::common::{
    api_utils::serialize_params,
    validation::{validate_card_id, validate_element_id},
};
use crate::endpoints::cardkit_v1_card_element;

/// 更新组件属性请求体（结构以官方文档为准）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PatchCardElementBody {
    /// 卡片 ID。
    pub card_id: String,
    /// 组件 ID。
    pub element_id: String,
    /// 补丁内容。
    pub patch: serde_json::Value,
}

/// 更新组件属性请求
#[derive(Debug, Clone)]
pub struct PatchCardElementRequest {
    config: Config,
    card_id: Option<String>,
    element_id: Option<String>,
    patch: Option<serde_json::Value>,
}

impl PatchCardElementRequest {
    /// 创建新的实例。
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
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card-element/patch>
    pub async fn execute(self, body: PatchCardElementBody) -> SDKResult<PatchCardElementResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（支持自定义选项）
    ///
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card-element/patch>
    pub async fn execute_with_options(
        self,
        body: PatchCardElementBody,
        option: RequestOption,
    ) -> SDKResult<PatchCardElementResponse> {
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

        // url: PATCH:/open-apis/cardkit/v1/cards/:card_id/elements/:element_id
        let req: ApiRequest<PatchCardElementResponse> =
            ApiRequest::patch(cardkit_v1_card_element(&body.card_id, &body.element_id))
                .body(serialize_params(&body, "更新组件属性")?);

        Transport::request_typed(req, &self.config, Some(option), "更新组件属性").await
    }
}

/// 更新组件属性请求构建器
#[derive(Debug, Clone)]
pub struct PatchCardElementRequestBuilder {
    request: PatchCardElementRequest,
    card_id: Option<String>,
    element_id: Option<String>,
    patch: Option<serde_json::Value>,
}

impl PatchCardElementRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: PatchCardElementRequest::new(config),
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

    /// 设置 patch 内容
    pub fn patch(mut self, patch: impl Into<serde_json::Value>) -> Self {
        self.patch = Some(patch.into());
        self
    }

    /// 构建请求
    pub fn build(self) -> PatchCardElementRequest {
        PatchCardElementRequest {
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

    /// 端到端：PATCH .../cards/{card_id}/elements/{element_id} + body 序列化 → PatchCardElementResponse。
    #[tokio::test]
    async fn test_patch_card_element_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
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

        let body = PatchCardElementBody {
            card_id: "card_001".into(),
            element_id: "elem_001".into(),
            patch: json!({ "tag": "column_set" }),
        };
        let resp = PatchCardElementRequest::new(config)
            .execute(body)
            .await
            .expect("更新组件属性应成功");
        assert_eq!(resp.element_id.as_deref(), Some("elem_001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["patch"]["tag"], "column_set");
    }
}
