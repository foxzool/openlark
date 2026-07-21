//! 转换 ID
//!
//! docPath: <https://open.feishu.cn/document/historic-version/id_convert>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

use super::models::ConvertCardIdResponse;
use crate::common::{
    api_utils::serialize_params,
    validation::{validate_id_list, validate_id_type},
};
use crate::endpoints::CARDKIT_V1_CARD_ID_CONVERT;

/// 转换 ID 请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertCardIdBody {
    /// 源 ID 类型
    pub source_id_type: String,
    /// 目标 ID 类型
    pub target_id_type: String,
    /// 卡片 ID 列表
    pub card_ids: Vec<String>,
}

/// 转换 ID 请求
#[derive(Debug, Clone)]
pub struct ConvertCardIdRequest {
    config: Config,
    source_id_type: Option<String>,
    target_id_type: Option<String>,
    card_ids: Option<Vec<String>>,
}

impl ConvertCardIdRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            source_id_type: None,
            target_id_type: None,
            card_ids: None,
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
        if let Some(source_id_type) = self.source_id_type {
            body.source_id_type = source_id_type;
        }
        if let Some(target_id_type) = self.target_id_type {
            body.target_id_type = target_id_type;
        }
        if let Some(card_ids) = self.card_ids {
            body.card_ids = card_ids;
        }

        validate_id_type(&body.source_id_type, "source_id_type")?;
        validate_id_type(&body.target_id_type, "target_id_type")?;
        validate_id_list(&body.card_ids, "card_ids")?;

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
    source_id_type: Option<String>,
    target_id_type: Option<String>,
    card_ids: Option<Vec<String>>,
}

impl ConvertCardIdRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: ConvertCardIdRequest::new(config),
            source_id_type: None,
            target_id_type: None,
            card_ids: None,
        }
    }

    /// 设置源 ID 类型
    pub fn source_id_type(mut self, source_id_type: impl Into<String>) -> Self {
        self.source_id_type = Some(source_id_type.into());
        self
    }

    /// 设置目标 ID 类型
    pub fn target_id_type(mut self, target_id_type: impl Into<String>) -> Self {
        self.target_id_type = Some(target_id_type.into());
        self
    }

    /// 设置卡片 ID 列表
    pub fn card_ids(mut self, card_ids: impl Into<Vec<String>>) -> Self {
        self.card_ids = Some(card_ids.into());
        self
    }

    /// 构建请求
    pub fn build(self) -> ConvertCardIdRequest {
        ConvertCardIdRequest {
            config: self.request.config,
            source_id_type: self.source_id_type,
            target_id_type: self.target_id_type,
            card_ids: self.card_ids,
        }
    }
}

/// 执行请求
///
/// docPath: <https://open.feishu.cn/document/historic-version/id_convert>
pub async fn convert(config: &Config, body: ConvertCardIdBody) -> SDKResult<ConvertCardIdResponse> {
    convert_with_options(config, body, RequestOption::default()).await
}

/// 执行请求（支持自定义选项）
///
/// docPath: <https://open.feishu.cn/document/historic-version/id_convert>
pub async fn convert_with_options(
    config: &Config,
    body: ConvertCardIdBody,
    option: RequestOption,
) -> SDKResult<ConvertCardIdResponse> {
    validate_id_type(&body.source_id_type, "source_id_type")?;
    validate_id_type(&body.target_id_type, "target_id_type")?;
    validate_id_list(&body.card_ids, "card_ids")?;

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
                "data": { "card_id": "open_card_001" }
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
            source_id_type: "card_id".into(),
            target_id_type: "open_id".into(),
            card_ids: vec!["card_001".into()],
        };
        let resp = ConvertCardIdRequest::new(config)
            .execute(body)
            .await
            .expect("转换 ID 应成功");
        assert_eq!(resp.card_id.as_deref(), Some("open_card_001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["source_id_type"], "card_id");
        assert_eq!(sent["card_ids"][0], "card_001");
    }
}
