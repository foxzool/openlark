//! 转换 ID
//!
//! docPath: <https://open.feishu.cn/document/historic-version/id_convert>
//!
//! 注意：官方文档标注该接口已废弃，推荐先创建卡片实体再发送消息。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use super::models::ConvertCardIdResponse;
use crate::{common::api_utils::serialize_params, endpoints::CARDKIT_V1_CARD_ID_CONVERT};

/// 转换 ID 请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertCardIdBody {
    /// 消息 ID（由发送消息等接口返回）
    pub message_id: String,
}

impl ConvertCardIdBody {
    /// 校验请求体。
    pub fn validate(&self) -> SDKResult<()> {
        validate_required!(self.message_id, "message_id 不能为空");
        Ok(())
    }
}

/// 转换 ID 请求
#[derive(Debug, Clone)]
pub struct ConvertCardIdRequest {
    config: Config,
    message_id: Option<String>,
}

impl ConvertCardIdRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            message_id: None,
        }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/historic-version/id_convert>
    pub async fn execute(self, body: ConvertCardIdBody) -> SDKResult<ConvertCardIdResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（支持自定义选项）
    ///
    /// docPath: <https://open.feishu.cn/document/historic-version/id_convert>
    pub async fn execute_with_options(
        self,
        body: ConvertCardIdBody,
        option: RequestOption,
    ) -> SDKResult<ConvertCardIdResponse> {
        let mut body = body;
        if let Some(message_id) = self.message_id {
            body.message_id = message_id;
        }

        body.validate()?;

        // url: POST:/open-apis/cardkit/v1/cards/id_convert
        let req: ApiRequest<ConvertCardIdResponse> =
            ApiRequest::post(CARDKIT_V1_CARD_ID_CONVERT).body(serialize_params(&body, "转换 ID")?);

        Transport::request_typed(req, &self.config, Some(option), "转换 ID").await
    }
}

/// 转换 ID 请求构建器
#[derive(Debug, Clone)]
pub struct ConvertCardIdRequestBuilder {
    request: ConvertCardIdRequest,
}

impl ConvertCardIdRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: ConvertCardIdRequest::new(config),
        }
    }

    /// 设置消息 ID
    pub fn message_id(mut self, message_id: impl Into<String>) -> Self {
        self.request.message_id = Some(message_id.into());
        self
    }

    /// 构建请求
    pub fn build(self) -> ConvertCardIdRequest {
        self.request
    }
}

/// 执行转换 ID 请求
///
/// docPath: <https://open.feishu.cn/document/historic-version/id_convert>
pub async fn id_convert(
    config: &Config,
    body: ConvertCardIdBody,
) -> SDKResult<ConvertCardIdResponse> {
    id_convert_with_options(config, body, RequestOption::default()).await
}

/// 执行转换 ID 请求（支持自定义选项）
///
/// docPath: <https://open.feishu.cn/document/historic-version/id_convert>
pub async fn id_convert_with_options(
    config: &Config,
    body: ConvertCardIdBody,
    option: RequestOption,
) -> SDKResult<ConvertCardIdResponse> {
    body.validate()?;

    // url: POST:/open-apis/cardkit/v1/cards/id_convert
    let req: ApiRequest<ConvertCardIdResponse> =
        ApiRequest::post(CARDKIT_V1_CARD_ID_CONVERT).body(serialize_params(&body, "转换 ID")?);

    Transport::request_typed(req, config, Some(option), "转换 ID").await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/cardkit/v1/cards/id_convert + body 序列化 → ConvertCardIdResponse。
    #[tokio::test]
    async fn test_convert_card_id_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/cardkit/v1/cards/id_convert"))
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

        let body = ConvertCardIdBody {
            message_id: "om_fbdf6ed2e17f1d98e78fb26c1370186e".into(),
        };
        let resp = ConvertCardIdRequest::new(config)
            .execute(body)
            .await
            .expect("转换 ID 应成功");
        assert_eq!(resp.card_id.as_deref(), Some("card_001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["message_id"], "om_fbdf6ed2e17f1d98e78fb26c1370186e");
        assert!(sent.get("card_ids").is_none());
        assert!(sent.get("source_id_type").is_none());
    }
}
