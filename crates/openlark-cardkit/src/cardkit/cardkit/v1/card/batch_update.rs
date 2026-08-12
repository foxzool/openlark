//! 局部更新卡片实体
//!
//! docPath: <https://open.feishu.cn/document/cardkit-v1/card/batch_update>

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
    endpoints::cardkit_v1_card_batch_update,
};

/// 局部更新卡片实体请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateCardBody {
    /// 卡片 ID（路径参数，不进入 JSON body）
    #[serde(skip_serializing)]
    pub card_id: String,
    /// 操作列表（JSON 数组的序列化字符串）
    pub actions: String,
    /// 幂等 ID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    /// 流式更新序号（必填，严格递增）
    pub sequence: i32,
}

impl BatchUpdateCardBody {
    /// 校验请求体。
    pub fn validate(&self) -> SDKResult<()> {
        validate_card_id(&self.card_id)?;
        validate_required!(self.actions, "actions 不能为空");
        validate_uuid(&self.uuid)?;
        validate_sequence(self.sequence)?;
        Ok(())
    }
}

/// 局部更新卡片实体响应（官方 `data` 为空对象）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchUpdateCardResponse {}

impl openlark_core::api::ApiResponseTrait for BatchUpdateCardResponse {}

/// 局部更新卡片实体请求
#[derive(Debug, Clone)]
pub struct BatchUpdateCardRequest {
    config: Config,
    card_id: Option<String>,
    actions: Option<String>,
    uuid: Option<String>,
    sequence: Option<i32>,
}

impl BatchUpdateCardRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            card_id: None,
            actions: None,
            uuid: None,
            sequence: None,
        }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card/batch_update>
    pub async fn execute(self, body: BatchUpdateCardBody) -> SDKResult<BatchUpdateCardResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（支持自定义选项）
    ///
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card/batch_update>
    pub async fn execute_with_options(
        self,
        body: BatchUpdateCardBody,
        option: RequestOption,
    ) -> SDKResult<BatchUpdateCardResponse> {
        let mut body = body;
        if let Some(card_id) = self.card_id {
            body.card_id = card_id;
        }
        if let Some(actions) = self.actions {
            body.actions = actions;
        }
        if let Some(uuid) = self.uuid {
            body.uuid = Some(uuid);
        }
        if let Some(sequence) = self.sequence {
            body.sequence = sequence;
        }

        body.validate()?;

        // url: POST:/open-apis/cardkit/v1/cards/:card_id/batch_update
        let url = cardkit_v1_card_batch_update(&body.card_id);
        let req: ApiRequest<BatchUpdateCardResponse> =
            ApiRequest::post(url).body(serialize_params(&body, "局部更新卡片实体")?);

        Transport::request_typed(req, &self.config, Some(option), "局部更新卡片实体").await
    }
}

/// 局部更新卡片实体请求构建器
#[derive(Debug, Clone)]
pub struct BatchUpdateCardRequestBuilder {
    request: BatchUpdateCardRequest,
}

impl BatchUpdateCardRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: BatchUpdateCardRequest::new(config),
        }
    }

    /// 设置卡片 ID
    pub fn card_id(mut self, card_id: impl Into<String>) -> Self {
        self.request.card_id = Some(card_id.into());
        self
    }

    /// 设置操作列表（JSON 数组序列化字符串）
    pub fn actions(mut self, actions: impl Into<String>) -> Self {
        self.request.actions = Some(actions.into());
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
    pub fn build(self) -> BatchUpdateCardRequest {
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

    /// 端到端：POST .../cards/{card_id}/batch_update + body 序列化 → BatchUpdateCardResponse。
    #[tokio::test]
    async fn test_batch_update_card_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/cardkit/v1/cards/card_001/batch_update"))
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

        let body = BatchUpdateCardBody {
            card_id: "card_001".into(),
            actions: r#"[{"action":"partial_update_setting","params":{"settings":{"config":{"streaming_mode":true}}}}]"#.into(),
            uuid: None,
            sequence: 1,
        };
        BatchUpdateCardRequest::new(config)
            .execute(body)
            .await
            .expect("局部更新卡片实体应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert!(sent.get("card_id").is_none());
        assert!(sent.get("operations").is_none());
        assert!(
            sent["actions"]
                .as_str()
                .unwrap()
                .contains("partial_update_setting")
        );
        assert_eq!(sent["sequence"], 1);
    }
}
