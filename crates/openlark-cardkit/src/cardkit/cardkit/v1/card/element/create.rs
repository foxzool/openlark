//! 新增组件
//!
//! docPath: <https://open.feishu.cn/document/cardkit-v1/card-element/create>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

use super::models::CreateCardElementResponse;
use crate::common::{
    api_utils::serialize_params,
    validation::{validate_card_id, validate_sequence, validate_uuid},
};
use crate::endpoints::cardkit_v1_card_elements;

/// 新增组件请求体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateCardElementBody {
    /// 卡片 ID（路径参数，不进入 JSON body）
    #[serde(skip_serializing)]
    pub card_id: String,
    /// 添加方式：`insert_before` / `insert_after` / `append`
    #[serde(rename = "type")]
    pub type_: String,
    /// 目标组件 ID（定位用，可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_element_id: Option<String>,
    /// 幂等 ID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    /// 流式更新序号（必填，严格递增）
    pub sequence: i32,
    /// 组件列表（JSON 数组序列化字符串）
    pub elements: String,
}

impl CreateCardElementBody {
    /// 校验请求体。
    pub fn validate(&self) -> SDKResult<()> {
        validate_card_id(&self.card_id)?;
        validate_required!(self.type_, "type 不能为空");
        validate_required!(self.elements, "elements 不能为空");
        validate_sequence(self.sequence)?;
        validate_uuid(&self.uuid)?;
        Ok(())
    }
}

/// 新增组件请求
#[derive(Debug, Clone)]
pub struct CreateCardElementRequest {
    config: Config,
    card_id: Option<String>,
    type_: Option<String>,
    target_element_id: Option<String>,
    uuid: Option<String>,
    sequence: Option<i32>,
    elements: Option<String>,
}

impl CreateCardElementRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            card_id: None,
            type_: None,
            target_element_id: None,
            uuid: None,
            sequence: None,
            elements: None,
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
        if let Some(type_) = self.type_ {
            body.type_ = type_;
        }
        if let Some(target_element_id) = self.target_element_id {
            body.target_element_id = Some(target_element_id);
        }
        if let Some(uuid) = self.uuid {
            body.uuid = Some(uuid);
        }
        if let Some(sequence) = self.sequence {
            body.sequence = sequence;
        }
        if let Some(elements) = self.elements {
            body.elements = elements;
        }

        body.validate()?;

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
}

impl CreateCardElementRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: CreateCardElementRequest::new(config),
        }
    }

    /// 设置卡片 ID
    pub fn card_id(mut self, card_id: impl Into<String>) -> Self {
        self.request.card_id = Some(card_id.into());
        self
    }

    /// 设置添加方式
    pub fn type_(mut self, type_: impl Into<String>) -> Self {
        self.request.type_ = Some(type_.into());
        self
    }

    /// 设置目标组件 ID
    pub fn target_element_id(mut self, target_element_id: impl Into<String>) -> Self {
        self.request.target_element_id = Some(target_element_id.into());
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

    /// 设置组件列表
    pub fn elements(mut self, elements: impl Into<String>) -> Self {
        self.request.elements = Some(elements.into());
        self
    }

    /// 构建请求
    pub fn build(self) -> CreateCardElementRequest {
        self.request
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
    body.validate()?;

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

    /// 端到端：POST .../cards/{card_id}/elements + body 序列化。
    #[tokio::test]
    async fn test_create_card_element_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/cardkit/v1/cards/card_001/elements"))
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

        let body = CreateCardElementBody {
            card_id: "card_001".into(),
            type_: "insert_before".into(),
            target_element_id: Some("elem_63529372".into()),
            uuid: None,
            sequence: 1,
            elements: r#"[{"tag":"markdown","id":"md_1","content":"示例文本"}]"#.into(),
        };
        CreateCardElementRequest::new(config)
            .execute(body)
            .await
            .expect("新增组件应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert!(sent.get("card_id").is_none());
        assert!(sent.get("element").is_none());
        assert_eq!(sent["type"], "insert_before");
        assert_eq!(sent["sequence"], 1);
        assert!(sent["elements"].as_str().unwrap().contains("markdown"));
    }
}
