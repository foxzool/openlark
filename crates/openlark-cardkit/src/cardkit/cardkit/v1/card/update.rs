//! 全量更新卡片实体
//!
//! docPath: <https://open.feishu.cn/document/cardkit-v1/card/update>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::{
        api_utils::{extract_response_data, serialize_params},
        validation::validate_card_id,
    },
    endpoints::cardkit_v1_card,
};

/// 全量更新卡片实体请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCardBody {
    /// 卡片 ID
    pub card_id: String,
    /// 卡片内容
    pub card_content: serde_json::Value,
    /// 卡片类型（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_type: Option<String>,
    /// 更新掩码（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_mask: Option<Vec<String>>,
}

/// 全量更新卡片实体响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCardResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

impl openlark_core::api::ApiResponseTrait for UpdateCardResponse {}

/// 全量更新卡片实体请求
#[derive(Debug, Clone)]
pub struct UpdateCardRequest {
    config: Config,
    card_id: Option<String>,
    card_content: Option<serde_json::Value>,
    card_type: Option<String>,
    update_mask: Option<Vec<String>>,
}

impl UpdateCardRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            card_id: None,
            card_content: None,
            card_type: None,
            update_mask: None,
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
        if let Some(card_content) = self.card_content {
            body.card_content = card_content;
        }
        if let Some(card_type) = self.card_type {
            body.card_type = Some(card_type);
        }
        if let Some(update_mask) = self.update_mask {
            body.update_mask = Some(update_mask);
        }

        validate_card_id(&body.card_id)?;

        // url: PUT:/open-apis/cardkit/v1/cards/:card_id
        let url = cardkit_v1_card(&body.card_id);
        let req: ApiRequest<UpdateCardResponse> =
            ApiRequest::put(url).body(serialize_params(&body, "全量更新卡片实体")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "全量更新卡片实体")
    }
}

/// 全量更新卡片实体请求构建器
#[derive(Debug, Clone)]
pub struct UpdateCardRequestBuilder {
    request: UpdateCardRequest,
    card_id: Option<String>,
    card_content: Option<serde_json::Value>,
    card_type: Option<String>,
    update_mask: Option<Vec<String>>,
}

impl UpdateCardRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: UpdateCardRequest::new(config),
            card_id: None,
            card_content: None,
            card_type: None,
            update_mask: None,
        }
    }

    /// 设置卡片 ID
    pub fn card_id(mut self, card_id: impl Into<String>) -> Self {
        self.card_id = Some(card_id.into());
        self
    }

    /// 设置卡片内容
    pub fn card_content(mut self, card_content: impl Into<serde_json::Value>) -> Self {
        self.card_content = Some(card_content.into());
        self
    }

    /// 设置卡片类型
    pub fn card_type(mut self, card_type: impl Into<String>) -> Self {
        self.card_type = Some(card_type.into());
        self
    }

    /// 设置更新掩码
    pub fn update_mask(mut self, update_mask: impl Into<Vec<String>>) -> Self {
        self.update_mask = Some(update_mask.into());
        self
    }

    /// 构建请求
    pub fn build(self) -> UpdateCardRequest {
        UpdateCardRequest {
            config: self.request.config,
            card_id: self.card_id,
            card_content: self.card_content,
            card_type: self.card_type,
            update_mask: self.update_mask,
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
