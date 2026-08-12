//! 创建卡片实体
//!
//! docPath: <https://open.feishu.cn/document/cardkit-v1/card/create>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::{common::api_utils::serialize_params, endpoints::CARDKIT_V1_CARDS};

/// 创建卡片实体请求体
///
/// 官方字段：`type`（`card_json` / `template`）+ `data`（JSON 序列化字符串）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCardBody {
    /// 卡片类型：`card_json` 或 `template`
    #[serde(rename = "type")]
    pub type_: String,
    /// 卡片数据（与 `type_` 对应的 JSON 序列化字符串）
    pub data: String,
}

impl CreateCardBody {
    /// 校验请求体。
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.type_, "type 不能为空");
        validate_required!(self.data, "data 不能为空");
        Ok(())
    }
}

/// 创建卡片实体响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateCardResponse {
    /// 卡片实体 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
}

impl openlark_core::api::ApiResponseTrait for CreateCardResponse {}

/// 创建卡片实体请求
#[derive(Debug, Clone)]
pub struct CreateCardRequest {
    config: Config,
}

impl CreateCardRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card/create>
    pub async fn execute(self, body: CreateCardBody) -> SDKResult<CreateCardResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（支持自定义选项）
    ///
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card/create>
    pub async fn execute_with_options(
        self,
        body: CreateCardBody,
        option: RequestOption,
    ) -> SDKResult<CreateCardResponse> {
        body.validate()?;

        // url: POST:/open-apis/cardkit/v1/cards
        let req: ApiRequest<CreateCardResponse> =
            ApiRequest::post(CARDKIT_V1_CARDS).body(serialize_params(&body, "创建卡片实体")?);

        Transport::request_typed(req, &self.config, Some(option), "创建卡片实体").await
    }
}

/// 创建卡片实体请求构建器
#[derive(Debug, Clone)]
pub struct CreateCardRequestBuilder {
    request: CreateCardRequest,
}

impl CreateCardRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: CreateCardRequest::new(config),
        }
    }

    /// 构建请求
    pub fn build(self) -> CreateCardRequest {
        self.request
    }
}

/// 执行创建卡片实体请求
///
/// docPath: <https://open.feishu.cn/document/cardkit-v1/card/create>
pub async fn create(config: &Config, body: CreateCardBody) -> SDKResult<CreateCardResponse> {
    create_with_options(config, body, RequestOption::default()).await
}

/// 执行创建卡片实体请求（支持自定义选项）
///
/// docPath: <https://open.feishu.cn/document/cardkit-v1/card/create>
pub async fn create_with_options(
    config: &Config,
    body: CreateCardBody,
    option: RequestOption,
) -> SDKResult<CreateCardResponse> {
    body.validate()?;

    // url: POST:/open-apis/cardkit/v1/cards
    let req: ApiRequest<CreateCardResponse> =
        ApiRequest::post(CARDKIT_V1_CARDS).body(serialize_params(&body, "创建卡片实体")?);

    Transport::request_typed(req, config, Some(option), "创建卡片实体").await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/cardkit/v1/cards + body 序列化 → 强类型 CreateCardResponse。
    #[tokio::test]
    async fn test_create_card_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/cardkit/v1/cards"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "card_id": "card_001" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let body = CreateCardBody {
            type_: "card_json".into(),
            data: r#"{"schema":"2.0"}"#.into(),
        };
        let resp = CreateCardRequest::new(config)
            .execute(body)
            .await
            .expect("创建卡片实体应成功");
        assert_eq!(resp.card_id.as_deref(), Some("card_001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["type"], "card_json");
        assert_eq!(sent["data"], r#"{"schema":"2.0"}"#);
        assert!(sent.get("card_content").is_none());
    }
}
