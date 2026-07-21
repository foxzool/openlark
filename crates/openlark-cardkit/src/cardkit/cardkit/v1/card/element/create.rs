//! 新增组件
//!
//! docPath: <https://open.feishu.cn/document/cardkit-v1/card-element/create>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use super::models::CreateCardElementResponse;
use crate::common::{api_utils::serialize_params, validation::validate_card_id};
use crate::endpoints::cardkit_v1_card_elements;

/// 新增组件请求体（结构以官方文档为准）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateCardElementBody {
    /// 卡片 ID
    pub card_id: String,
    /// 组件定义
    pub element: serde_json::Value,
}

/// 新增组件请求
#[derive(Debug, Clone)]
pub struct CreateCardElementRequest {
    config: Config,
    card_id: Option<String>,
}

impl CreateCardElementRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            card_id: None,
        }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card-element/create>
    pub async fn execute(
        self,
        body: CreateCardElementBody,
    ) -> SDKResult<CreateCardElementResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（支持自定义选项）
    ///
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card-element/create>
    pub async fn execute_with_options(
        self,
        body: CreateCardElementBody,
        option: RequestOption,
    ) -> SDKResult<CreateCardElementResponse> {
        let mut body = body;
        if let Some(card_id) = self.card_id {
            body.card_id = card_id;
        }

        validate_card_id(&body.card_id)?;

        // url: POST:/open-apis/cardkit/v1/cards/:card_id/elements
        let req: ApiRequest<CreateCardElementResponse> =
            ApiRequest::post(cardkit_v1_card_elements(&body.card_id))
                .body(serialize_params(&body, "新增组件")?);

        Transport::request_typed(req, &self.config, Some(option), "新增组件").await
    }
}

/// 新增组件请求构建器
#[derive(Debug, Clone)]
pub struct CreateCardElementRequestBuilder {
    request: CreateCardElementRequest,
    card_id: Option<String>,
    element: Option<serde_json::Value>,
}

impl CreateCardElementRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: CreateCardElementRequest::new(config),
            card_id: None,
            element: None,
        }
    }

    /// 设置卡片 ID
    pub fn card_id(mut self, card_id: impl Into<String>) -> Self {
        self.card_id = Some(card_id.into());
        self
    }

    /// 设置组件定义
    pub fn element(mut self, element: impl Into<serde_json::Value>) -> Self {
        self.element = Some(element.into());
        self
    }

    /// 构建请求
    pub fn build(self) -> CreateCardElementRequest {
        CreateCardElementRequest {
            config: self.request.config,
            card_id: self.card_id,
        }
    }
}

/// 执行请求
///
/// docPath: <https://open.feishu.cn/document/cardkit-v1/card-element/create>
pub async fn create(
    config: &Config,
    body: CreateCardElementBody,
) -> SDKResult<CreateCardElementResponse> {
    create_with_options(config, body, RequestOption::default()).await
}

/// 执行请求（支持自定义选项）
///
/// docPath: <https://open.feishu.cn/document/cardkit-v1/card-element/create>
pub async fn create_with_options(
    config: &Config,
    body: CreateCardElementBody,
    option: RequestOption,
) -> SDKResult<CreateCardElementResponse> {
    validate_card_id(&body.card_id)?;

    // url: POST:/open-apis/cardkit/v1/cards/:card_id/elements
    let req: ApiRequest<CreateCardElementResponse> =
        ApiRequest::post(cardkit_v1_card_elements(&body.card_id))
            .body(serialize_params(&body, "新增组件")?);

    Transport::request_typed(req, config, Some(option), "新增组件").await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST .../cards/{card_id}/elements + body 序列化 → CreateCardElementResponse。
    #[tokio::test]
    async fn test_create_card_element_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/cardkit/v1/cards/card_001/elements"))
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

        let body = CreateCardElementBody {
            card_id: "card_001".into(),
            element: json!({ "tag": "div" }),
        };
        let resp = CreateCardElementRequest::new(config)
            .execute(body)
            .await
            .expect("新增组件应成功");
        assert_eq!(resp.element_id.as_deref(), Some("elem_001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["element"]["tag"], "div");
    }
}
