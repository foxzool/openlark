//! 删除组件
//!
//! docPath: <https://open.feishu.cn/document/cardkit-v1/card-element/delete>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use super::models::DeleteCardElementResponse;
use crate::common::{
    api_utils::serialize_params,
    validation::{validate_card_id, validate_element_id, validate_sequence, validate_uuid},
};
use crate::endpoints::cardkit_v1_card_element;

/// 删除组件请求体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeleteCardElementBody {
    /// 卡片 ID（路径参数，不进入 JSON body）
    #[serde(skip_serializing)]
    pub card_id: String,
    /// 组件 ID（路径参数，不进入 JSON body）
    #[serde(skip_serializing)]
    pub element_id: String,
    /// 流式更新序号（必填，严格递增）
    pub sequence: i32,
    /// 幂等 ID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

impl DeleteCardElementBody {
    /// 校验请求体。
    pub fn validate(&self) -> SDKResult<()> {
        validate_card_id(&self.card_id)?;
        validate_element_id(&self.element_id)?;
        validate_sequence(self.sequence)?;
        validate_uuid(&self.uuid)?;
        Ok(())
    }
}

/// 删除组件请求
#[derive(Debug, Clone)]
pub struct DeleteCardElementRequest {
    config: Config,
    card_id: Option<String>,
    element_id: Option<String>,
    sequence: Option<i32>,
    uuid: Option<String>,
}

impl DeleteCardElementRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            card_id: None,
            element_id: None,
            sequence: None,
            uuid: None,
        }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card-element/delete>
    pub async fn execute(
        self,
        body: DeleteCardElementBody,
    ) -> SDKResult<DeleteCardElementResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（支持自定义选项）
    ///
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card-element/delete>
    pub async fn execute_with_options(
        self,
        body: DeleteCardElementBody,
        option: RequestOption,
    ) -> SDKResult<DeleteCardElementResponse> {
        let mut body = body;
        if let Some(card_id) = self.card_id {
            body.card_id = card_id;
        }
        if let Some(element_id) = self.element_id {
            body.element_id = element_id;
        }
        if let Some(sequence) = self.sequence {
            body.sequence = sequence;
        }
        if let Some(uuid) = self.uuid {
            body.uuid = Some(uuid);
        }

        body.validate()?;

        // url: DELETE:/open-apis/cardkit/v1/cards/:card_id/elements/:element_id
        let req: ApiRequest<DeleteCardElementResponse> =
            ApiRequest::delete(cardkit_v1_card_element(&body.card_id, &body.element_id))
                .body(serialize_params(&body, "删除组件")?);

        Transport::request_typed(req, &self.config, Some(option), "删除组件").await
    }
}

/// 删除组件请求构建器
#[derive(Debug, Clone)]
pub struct DeleteCardElementRequestBuilder {
    request: DeleteCardElementRequest,
}

impl DeleteCardElementRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: DeleteCardElementRequest::new(config),
        }
    }

    /// 设置卡片 ID
    pub fn card_id(mut self, card_id: impl Into<String>) -> Self {
        self.request.card_id = Some(card_id.into());
        self
    }

    /// 设置组件 ID
    pub fn element_id(mut self, element_id: impl Into<String>) -> Self {
        self.request.element_id = Some(element_id.into());
        self
    }

    /// 设置流式更新序号
    pub fn sequence(mut self, sequence: i32) -> Self {
        self.request.sequence = Some(sequence);
        self
    }

    /// 设置幂等 ID
    pub fn uuid(mut self, uuid: impl Into<String>) -> Self {
        self.request.uuid = Some(uuid.into());
        self
    }

    /// 构建请求
    pub fn build(self) -> DeleteCardElementRequest {
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

    /// 端到端：DELETE .../elements/{element_id} + body（含 sequence）。
    #[tokio::test]
    async fn test_delete_card_element_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/open-apis/cardkit/v1/cards/card_001/elements/elem_001",
            ))
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

        let body = DeleteCardElementBody {
            card_id: "card_001".into(),
            element_id: "elem_001".into(),
            sequence: 1,
            uuid: Some("a0d69e20-1dd1-458b-k525-dfeca4015204".into()),
        };
        DeleteCardElementRequest::new(config)
            .execute(body)
            .await
            .expect("删除组件应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/cardkit/v1/cards/card_001/elements/elem_001"
        );
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert!(sent.get("card_id").is_none());
        assert_eq!(sent["sequence"], 1);
        assert_eq!(sent["uuid"], "a0d69e20-1dd1-458b-k525-dfeca4015204");
    }
}
