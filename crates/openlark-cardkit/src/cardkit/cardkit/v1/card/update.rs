//! 全量更新卡片实体
//!
//! docPath: <https://open.feishu.cn/document/cardkit-v1/card/update>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::{
        api_utils::serialize_params,
        validation::{validate_card_id, validate_sequence, validate_uuid},
    },
    endpoints::cardkit_v1_card,
};

/// 全量更新卡片内容（`card` 对象）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCardPayload {
    /// 卡片数据类型，固定值 `card_json`
    #[serde(rename = "type")]
    pub type_: String,
    /// 卡片 JSON 数据（序列化字符串，仅支持 schema 2.0）
    pub data: String,
}

/// 全量更新卡片实体请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCardBody {
    /// 卡片 ID（路径参数，不进入 JSON body）
    #[serde(skip_serializing)]
    pub card_id: String,
    /// 更新后的卡片内容
    pub card: UpdateCardPayload,
    /// 幂等 ID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    /// 流式更新序号（必填，严格递增）
    pub sequence: i32,
}

impl UpdateCardBody {
    /// 校验请求体。
    pub fn validate(&self) -> SDKResult<()> {
        validate_card_id(&self.card_id)?;
        validate_required!(self.card.type_, "card.type 不能为空");
        validate_required!(self.card.data, "card.data 不能为空");
        validate_uuid(&self.uuid)?;
        validate_sequence(self.sequence)?;
        Ok(())
    }
}

/// 全量更新卡片实体响应（官方 `data` 为空对象）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCardResponse {}

impl openlark_core::api::ApiResponseTrait for UpdateCardResponse {}

/// 全量更新卡片实体请求
#[derive(Debug, Clone)]
pub struct UpdateCardRequest {
    config: Config,
    card_id: Option<String>,
    card: Option<UpdateCardPayload>,
    uuid: Option<String>,
    sequence: Option<i32>,
}

impl UpdateCardRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            card_id: None,
            card: None,
            uuid: None,
            sequence: None,
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
        if let Some(card) = self.card {
            body.card = card;
        }
        if let Some(uuid) = self.uuid {
            body.uuid = Some(uuid);
        }
        if let Some(sequence) = self.sequence {
            body.sequence = sequence;
        }

        body.validate()?;

        // url: PUT:/open-apis/cardkit/v1/cards/:card_id
        let url = cardkit_v1_card(&body.card_id);
        let req: ApiRequest<UpdateCardResponse> =
            ApiRequest::put(url).body(serialize_params(&body, "全量更新卡片实体")?);

        Transport::request_typed(req, &self.config, Some(option), "全量更新卡片实体").await
    }
}

/// 全量更新卡片实体请求构建器
#[derive(Debug, Clone)]
pub struct UpdateCardRequestBuilder {
    request: UpdateCardRequest,
}

impl UpdateCardRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: UpdateCardRequest::new(config),
        }
    }

    /// 设置卡片 ID
    pub fn card_id(mut self, card_id: impl Into<String>) -> Self {
        self.request.card_id = Some(card_id.into());
        self
    }

    /// 设置卡片内容
    pub fn card(mut self, card: UpdateCardPayload) -> Self {
        self.request.card = Some(card);
        self
    }

    /// 设置幂等 ID
    pub fn uuid(mut self, uuid: impl Into<String>) -> Self {
        self.request.uuid = Some(uuid.into());
        self
    }

    /// 设置流式更新序号
    pub fn sequence(mut self, sequence: i32) -> Self {
        self.request.sequence = Some(sequence);
        self
    }

    /// 构建请求
    pub fn build(self) -> UpdateCardRequest {
        self.request
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
                "data": {}
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
            card: UpdateCardPayload {
                type_: "card_json".into(),
                data: r#"{"schema":"2.0"}"#.into(),
            },
            uuid: Some("a0d69e20-1dd1-458b-k525-dfeca4015204".into()),
            sequence: 1,
        };
        UpdateCardRequest::new(config)
            .execute(body)
            .await
            .expect("全量更新卡片实体应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert!(sent.get("card_id").is_none());
        assert_eq!(sent["card"]["type"], "card_json");
        assert_eq!(sent["card"]["data"], r#"{"schema":"2.0"}"#);
        assert_eq!(sent["sequence"], 1);
        assert_eq!(sent["uuid"], "a0d69e20-1dd1-458b-k525-dfeca4015204");
    }
}
