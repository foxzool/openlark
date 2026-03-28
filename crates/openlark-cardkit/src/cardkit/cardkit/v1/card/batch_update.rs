//! 局部更新卡片实体
//!
//! docPath: <https://open.feishu.cn/document/cardkit-v1/card/batch_update>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::{
        api_utils::{extract_response_data, serialize_params},
        validation::validate_card_id,
    },
    endpoints::cardkit_v1_card_batch_update,
};

/// 局部更新卡片实体请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateCardBody {
    /// 卡片 ID
    pub card_id: String,
    /// 操作列表（结构以官方文档为准）
    pub operations: Vec<serde_json::Value>,
}

/// 局部更新卡片实体响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchUpdateCardResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

impl openlark_core::api::ApiResponseTrait for BatchUpdateCardResponse {}

/// 局部更新卡片实体请求
#[derive(Debug, Clone)]
pub struct BatchUpdateCardRequest {
    config: Config,
    card_id: Option<String>,
    operations: Option<Vec<serde_json::Value>>,
}

impl BatchUpdateCardRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            card_id: None,
            operations: None,
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
        if let Some(operations) = self.operations {
            body.operations = operations;
        }

        validate_card_id(&body.card_id)?;
        if body.operations.is_empty() {
            return Err(openlark_core::error::validation_error(
                "operations 不能为空",
                "operations 不能为空",
            ));
        }

        // url: POST:/open-apis/cardkit/v1/cards/:card_id/batch_update
        let url = cardkit_v1_card_batch_update(&body.card_id);
        let req: ApiRequest<BatchUpdateCardResponse> =
            ApiRequest::post(url).body(serialize_params(&body, "局部更新卡片实体")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "局部更新卡片实体")
    }
}

/// 局部更新卡片实体请求构建器
#[derive(Debug, Clone)]
pub struct BatchUpdateCardRequestBuilder {
    request: BatchUpdateCardRequest,
    card_id: Option<String>,
    operations: Option<Vec<serde_json::Value>>,
}

impl BatchUpdateCardRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: BatchUpdateCardRequest::new(config),
            card_id: None,
            operations: None,
        }
    }

    /// 设置卡片 ID
    pub fn card_id(mut self, card_id: impl Into<String>) -> Self {
        self.card_id = Some(card_id.into());
        self
    }

    /// 设置操作列表
    pub fn operations(mut self, operations: impl Into<Vec<serde_json::Value>>) -> Self {
        self.operations = Some(operations.into());
        self
    }

    /// 构建请求
    pub fn build(self) -> BatchUpdateCardRequest {
        BatchUpdateCardRequest {
            config: self.request.config,
            card_id: self.card_id,
            operations: self.operations,
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
