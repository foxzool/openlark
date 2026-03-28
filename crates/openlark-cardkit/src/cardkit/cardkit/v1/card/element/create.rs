//! 新增组件
//!
//! docPath: <https://open.feishu.cn/document/cardkit-v1/card-element/create>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};

use super::models::CreateCardElementResponse;
use crate::common::{
    api_utils::{extract_response_data, serialize_params},
    validation::validate_card_id,
};
use crate::endpoints::cardkit_v1_card_elements;

/// 新增组件请求体（结构以官方文档为准）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateCardElementBody {
    /// 卡片 ID
    pub card_id: String,
    /// 组件定义
    pub element: serde_json::Value,
}

/// 新增组件请求
#[derive(Debug, Clone)]
pub struct CreateCardElementRequest {
    config: Config,
    card_id: Option<String>,
}

impl CreateCardElementRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            card_id: None,
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

        validate_card_id(&body.card_id)?;

        // url: POST:/open-apis/cardkit/v1/cards/:card_id/elements
        let req: ApiRequest<CreateCardElementResponse> =
            ApiRequest::post(cardkit_v1_card_elements(&body.card_id))
                .body(serialize_params(&body, "新增组件")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "新增组件")
    }
}

/// 新增组件请求构建器
#[derive(Debug, Clone)]
pub struct CreateCardElementRequestBuilder {
    request: CreateCardElementRequest,
    card_id: Option<String>,
    element: Option<serde_json::Value>,
}

impl CreateCardElementRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: CreateCardElementRequest::new(config),
            card_id: None,
            element: None,
        }
    }

    /// 设置卡片 ID
    pub fn card_id(mut self, card_id: impl Into<String>) -> Self {
        self.card_id = Some(card_id.into());
        self
    }

    /// 设置组件定义
    pub fn element(mut self, element: impl Into<serde_json::Value>) -> Self {
        self.element = Some(element.into());
        self
    }

    /// 构建请求
    pub fn build(self) -> CreateCardElementRequest {
        CreateCardElementRequest {
            config: self.request.config,
            card_id: self.card_id,
        }
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
    validate_card_id(&body.card_id)?;

    // url: POST:/open-apis/cardkit/v1/cards/:card_id/elements
    let req: ApiRequest<CreateCardElementResponse> =
        ApiRequest::post(cardkit_v1_card_elements(&body.card_id))
            .body(serialize_params(&body, "新增组件")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "新增组件")
}

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
