//! 删除组件
//!
//! docPath: <https://open.feishu.cn/document/cardkit-v1/card-element/delete>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};

use super::models::DeleteCardElementResponse;
use crate::common::{
    api_utils::extract_response_data,
    validation::{validate_card_id, validate_element_id},
};
use crate::endpoints::cardkit_v1_card_element;

/// 删除组件请求体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeleteCardElementBody {
    pub card_id: String,
    pub element_id: String,
}

/// 删除组件请求
#[derive(Debug, Clone)]
pub struct DeleteCardElementRequest {
    config: Config,
    card_id: Option<String>,
    element_id: Option<String>,
}

impl DeleteCardElementRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            card_id: None,
            element_id: None,
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

        validate_card_id(&body.card_id)?;
        validate_element_id(&body.element_id)?;

        // url: DELETE:/open-apis/cardkit/v1/cards/:card_id/elements/:element_id
        let req: ApiRequest<DeleteCardElementResponse> =
            ApiRequest::delete(cardkit_v1_card_element(&body.card_id, &body.element_id));

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "删除组件")
    }
}

/// 删除组件请求构建器
#[derive(Debug, Clone)]
pub struct DeleteCardElementRequestBuilder {
    request: DeleteCardElementRequest,
    card_id: Option<String>,
    element_id: Option<String>,
}

impl DeleteCardElementRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: DeleteCardElementRequest::new(config),
            card_id: None,
            element_id: None,
        }
    }

    /// 设置卡片 ID
    pub fn card_id(mut self, card_id: impl Into<String>) -> Self {
        self.card_id = Some(card_id.into());
        self
    }

    /// 设置组件 ID
    pub fn element_id(mut self, element_id: impl Into<String>) -> Self {
        self.element_id = Some(element_id.into());
        self
    }

    /// 构建请求
    pub fn build(self) -> DeleteCardElementRequest {
        DeleteCardElementRequest {
            config: self.request.config,
            card_id: self.card_id,
            element_id: self.element_id,
        }
    }
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
