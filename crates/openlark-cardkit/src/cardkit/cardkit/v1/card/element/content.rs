//! 流式更新文本
//!
//! docPath: <https://open.feishu.cn/document/cardkit-v1/card-element/content>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

use super::models::UpdateCardElementContentResponse;
use crate::common::{
    api_utils::serialize_params,
    validation::{validate_card_id, validate_element_id, validate_sequence, validate_uuid},
};
use crate::endpoints::cardkit_v1_card_element_content;

/// 流式更新文本请求体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateCardElementContentBody {
    /// 卡片 ID（路径参数，不进入 JSON body）
    #[serde(skip_serializing)]
    pub card_id: String,
    /// 组件 ID（路径参数，不进入 JSON body）
    #[serde(skip_serializing)]
    pub element_id: String,
    /// 更新后的文本内容
    pub content: String,
    /// 流式更新序号（必填，严格递增）
    pub sequence: i32,
    /// 幂等 ID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

impl UpdateCardElementContentBody {
    /// 校验请求体。
    pub fn validate(&self) -> SDKResult<()> {
        validate_card_id(&self.card_id)?;
        validate_element_id(&self.element_id)?;
        validate_required!(self.content, "content 不能为空");
        validate_sequence(self.sequence)?;
        validate_uuid(&self.uuid)?;
        Ok(())
    }
}

/// 流式更新文本请求
#[derive(Debug, Clone)]
pub struct UpdateCardElementContentRequest {
    config: Config,
    card_id: Option<String>,
    element_id: Option<String>,
    content: Option<String>,
    sequence: Option<i32>,
    uuid: Option<String>,
}

impl UpdateCardElementContentRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            card_id: None,
            element_id: None,
            content: None,
            sequence: None,
            uuid: None,
        }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card-element/content>
    pub async fn execute(
        self,
        body: UpdateCardElementContentBody,
    ) -> SDKResult<UpdateCardElementContentResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（支持自定义选项）
    ///
    /// docPath: <https://open.feishu.cn/document/cardkit-v1/card-element/content>
    pub async fn execute_with_options(
        self,
        body: UpdateCardElementContentBody,
        option: RequestOption,
    ) -> SDKResult<UpdateCardElementContentResponse> {
        let mut body = body;
        if let Some(card_id) = self.card_id {
            body.card_id = card_id;
        }
        if let Some(element_id) = self.element_id {
            body.element_id = element_id;
        }
        if let Some(content) = self.content {
            body.content = content;
        }
        if let Some(sequence) = self.sequence {
            body.sequence = sequence;
        }
        if let Some(uuid) = self.uuid {
            body.uuid = Some(uuid);
        }

        body.validate()?;

        // url: PUT:/open-apis/cardkit/v1/cards/:card_id/elements/:element_id/content
        let req: ApiRequest<UpdateCardElementContentResponse> = ApiRequest::put(
            cardkit_v1_card_element_content(&body.card_id, &body.element_id),
        )
        .body(serialize_params(&body, "流式更新文本")?);

        Transport::request_typed(req, &self.config, Some(option), "流式更新文本").await
    }
}

/// 流式更新文本请求构建器
#[derive(Debug, Clone)]
pub struct UpdateCardElementContentRequestBuilder {
    request: UpdateCardElementContentRequest,
}

impl UpdateCardElementContentRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: UpdateCardElementContentRequest::new(config),
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

    /// 设置内容
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.request.content = Some(content.into());
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
    pub fn build(self) -> UpdateCardElementContentRequest {
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

    /// 端到端：PUT .../cards/{card_id}/elements/{element_id}/content + body 序列化。
    #[tokio::test]
    async fn test_update_card_element_content_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path(
                "/open-apis/cardkit/v1/cards/card_001/elements/elem_001/content",
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

        let body = UpdateCardElementContentBody {
            card_id: "card_001".into(),
            element_id: "elem_001".into(),
            content: "updated text".into(),
            sequence: 1,
            uuid: Some("a0d69e20-1dd1-458b-k525-dfeca4015204".into()),
        };
        UpdateCardElementContentRequest::new(config)
            .execute(body)
            .await
            .expect("流式更新文本应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert!(sent.get("card_id").is_none());
        assert!(sent.get("element_id").is_none());
        assert_eq!(sent["content"], "updated text");
        assert_eq!(sent["sequence"], 1);
    }
}
