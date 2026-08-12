//! 更新组件
//!
//! docPath: <https://open.feishu.cn/document/cardkit-v1/card-element/update>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

use super::models::UpdateCardElementResponse;
use crate::common::{
    api_utils::serialize_params,
    validation::{validate_card_id, validate_element_id, validate_sequence, validate_uuid},
};
use crate::endpoints::cardkit_v1_card_element;

/// 更新组件请求体（全量替换组件）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateCardElementBody {
    /// 卡片 ID（路径参数，不进入 JSON body）
    #[serde(skip_serializing)]
    pub card_id: String,
    /// 组件 ID（路径参数，不进入 JSON body）
    #[serde(skip_serializing)]
    pub element_id: String,
    /// 新组件定义（JSON 序列化字符串）
    pub element: String,
    /// 流式更新序号（必填，严格递增）
    pub sequence: i32,
    /// 幂等 ID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

impl UpdateCardElementBody {
    /// 校验请求体。
    pub fn validate(&self) -> SDKResult<()> {
        validate_card_id(&self.card_id)?;
        validate_element_id(&self.element_id)?;
        validate_required!(self.element, "element 不能为空");
        validate_sequence(self.sequence)?;
        validate_uuid(&self.uuid)?;
        Ok(())
    }
}

/// 更新组件请求
#[derive(Debug, Clone)]
pub struct UpdateCardElementRequest {
    config: Config,
    card_id: Option<String>,
    element_id: Option<String>,
    element: Option<String>,
    sequence: Option<i32>,
    uuid: Option<String>,
}

impl UpdateCardElementRequest {
    /// 创建新的更新组件请求。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            card_id: None,
            element_id: None,
            element: None,
            sequence: None,
            uuid: None,
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
        if let Some(element) = self.element {
            body.element = element;
        }
        if let Some(sequence) = self.sequence {
            body.sequence = sequence;
        }
        if let Some(uuid) = self.uuid {
            body.uuid = Some(uuid);
        }

        body.validate()?;

        // url: PUT:/open-apis/cardkit/v1/cards/:card_id/elements/:element_id
        let req: ApiRequest<UpdateCardElementResponse> =
            ApiRequest::put(cardkit_v1_card_element(&body.card_id, &body.element_id))
                .body(serialize_params(&body, "更新组件")?);

        Transport::request_typed(req, &self.config, Some(option), "更新组件").await
    }
}

/// 更新组件请求构建器
#[derive(Debug, Clone)]
pub struct UpdateCardElementRequestBuilder {
    request: UpdateCardElementRequest,
}

impl UpdateCardElementRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: UpdateCardElementRequest::new(config),
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

    /// 设置新组件定义（JSON 序列化字符串）
    pub fn element(mut self, element: impl Into<String>) -> Self {
        self.request.element = Some(element.into());
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
    pub fn build(self) -> UpdateCardElementRequest {
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

    /// 端到端：PUT .../cards/{card_id}/elements/{element_id} + body 序列化。
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

        let body = UpdateCardElementBody {
            card_id: "card_001".into(),
            element_id: "elem_001".into(),
            element: r#"{"tag":"markdown","id":"md_1","content":"普通文本"}"#.into(),
            sequence: 1,
            uuid: None,
        };
        UpdateCardElementRequest::new(config)
            .execute(body)
            .await
            .expect("更新组件应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert!(sent.get("patch").is_none());
        assert!(sent["element"].as_str().unwrap().contains("markdown"));
        assert_eq!(sent["sequence"], 1);
    }
}
