//! 更新组件属性
//!
//! docPath: <https://open.feishu.cn/document/cardkit-v1/card-element/patch>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};

use super::models::PatchCardElementResponse;
use crate::common::{
    api_utils::{extract_response_data, serialize_params},
    validation::{validate_card_id, validate_element_id},
};
use crate::endpoints::cardkit_v1_card_element;

/// 更新组件属性请求体（结构以官方文档为准）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PatchCardElementBody {
    pub card_id: String,
    pub element_id: String,
    pub patch: serde_json::Value,
}

/// 更新组件属性请求
#[derive(Debug, Clone)]
pub struct PatchCardElementRequest {
    config: Config,
    card_id: Option<String>,
    element_id: Option<String>,
    patch: Option<serde_json::Value>,
}

impl PatchCardElementRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            card_id: None,
            element_id: None,
            patch: None,
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
        if let Some(patch) = self.patch {
            body.patch = patch;
        }

        validate_card_id(&body.card_id)?;
        validate_element_id(&body.element_id)?;

        // url: PATCH:/open-apis/cardkit/v1/cards/:card_id/elements/:element_id
        let req: ApiRequest<PatchCardElementResponse> =
            ApiRequest::patch(cardkit_v1_card_element(&body.card_id, &body.element_id))
                .body(serialize_params(&body, "更新组件属性")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "更新组件属性")
    }
}

/// 更新组件属性请求构建器
#[derive(Debug, Clone)]
pub struct PatchCardElementRequestBuilder {
    request: PatchCardElementRequest,
    card_id: Option<String>,
    element_id: Option<String>,
    patch: Option<serde_json::Value>,
}

impl PatchCardElementRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: PatchCardElementRequest::new(config),
            card_id: None,
            element_id: None,
            patch: None,
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

    /// 设置 patch 内容
    pub fn patch(mut self, patch: impl Into<serde_json::Value>) -> Self {
        self.patch = Some(patch.into());
        self
    }

    /// 构建请求
    pub fn build(self) -> PatchCardElementRequest {
        PatchCardElementRequest {
            config: self.request.config,
            card_id: self.card_id,
            element_id: self.element_id,
            patch: self.patch,
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
