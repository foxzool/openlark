//! 更新组件属性
//!
//! docPath: <https://open.feishu.cn/document/cardkit-v1/card-element/patch>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

use super::models::PatchCardElementResponse;
use crate::common::{
    api_utils::serialize_params,
    validation::{validate_card_id, validate_element_id, validate_sequence, validate_uuid},
};
use crate::endpoints::cardkit_v1_card_element;

/// 更新组件属性请求体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PatchCardElementBody {
    /// 卡片 ID（路径参数，不进入 JSON body）
    #[serde(skip_serializing)]
    pub card_id: String,
    /// 组件 ID（路径参数，不进入 JSON body）
    #[serde(skip_serializing)]
    pub element_id: String,
    /// 局部更新内容（JSON 序列化字符串；不支持修改 tag）
    pub partial_element: String,
    /// 流式更新序号（必填，严格递增）
    pub sequence: i32,
    /// 幂等 ID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

impl PatchCardElementBody {
    /// 校验请求体。
    pub fn validate(&self) -> SDKResult<()> {
        validate_card_id(&self.card_id)?;
        validate_element_id(&self.element_id)?;
        validate_required!(self.partial_element, "partial_element 不能为空");
        validate_sequence(self.sequence)?;
        validate_uuid(&self.uuid)?;
        Ok(())
    }
}

/// 更新组件属性请求
#[derive(Debug, Clone)]
pub struct PatchCardElementRequest {
    config: Config,
    card_id: Option<String>,
    element_id: Option<String>,
    partial_element: Option<String>,
    sequence: Option<i32>,
    uuid: Option<String>,
}

impl PatchCardElementRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            card_id: None,
            element_id: None,
            partial_element: None,
            sequence: None,
            uuid: None,
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
        if let Some(partial_element) = self.partial_element {
            body.partial_element = partial_element;
        }
        if let Some(sequence) = self.sequence {
            body.sequence = sequence;
        }
        if let Some(uuid) = self.uuid {
            body.uuid = Some(uuid);
        }

        body.validate()?;

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
}

impl PatchCardElementRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: PatchCardElementRequest::new(config),
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

    /// 设置局部更新内容
    pub fn partial_element(mut self, partial_element: impl Into<String>) -> Self {
        self.request.partial_element = Some(partial_element.into());
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
    pub fn build(self) -> PatchCardElementRequest {
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

    /// 端到端：PATCH .../cards/{card_id}/elements/{element_id} + body 序列化。
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

        let body = PatchCardElementBody {
            card_id: "card_001".into(),
            element_id: "elem_001".into(),
            partial_element: r#"{"content":"Updated text"}"#.into(),
            sequence: 1,
            uuid: None,
        };
        PatchCardElementRequest::new(config)
            .execute(body)
            .await
            .expect("更新组件属性应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert!(sent.get("patch").is_none());
        assert_eq!(sent["partial_element"], r#"{"content":"Updated text"}"#);
        assert_eq!(sent["sequence"], 1);
    }
}
