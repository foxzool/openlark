//! 转换 ID
//!
//! docPath: <https://open.feishu.cn/document/historic-version/id_convert>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};
use serde::{Deserialize, Serialize};

use super::models::ConvertCardIdResponse;
use crate::common::{
    api_utils::{extract_response_data, serialize_params},
    validation::{validate_id_list, validate_id_type},
};
use crate::endpoints::CARDKIT_V1_CARD_ID_CONVERT;

/// 转换 ID 请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertCardIdBody {
    /// 源 ID 类型
    pub source_id_type: String,
    /// 目标 ID 类型
    pub target_id_type: String,
    /// 卡片 ID 列表
    pub card_ids: Vec<String>,
}

/// 转换 ID 请求
#[derive(Debug, Clone)]
pub struct ConvertCardIdRequest {
    config: Config,
    source_id_type: Option<String>,
    target_id_type: Option<String>,
    card_ids: Option<Vec<String>>,
}

impl ConvertCardIdRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            source_id_type: None,
            target_id_type: None,
            card_ids: None,
        }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/historic-version/id_convert>
    pub async fn execute(self, body: ConvertCardIdBody) -> SDKResult<ConvertCardIdResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（支持自定义选项）
    ///
    /// docPath: <https://open.feishu.cn/document/historic-version/id_convert>
    pub async fn execute_with_options(
        self,
        body: ConvertCardIdBody,
        option: RequestOption,
    ) -> SDKResult<ConvertCardIdResponse> {
        let mut body = body;
        if let Some(source_id_type) = self.source_id_type {
            body.source_id_type = source_id_type;
        }
        if let Some(target_id_type) = self.target_id_type {
            body.target_id_type = target_id_type;
        }
        if let Some(card_ids) = self.card_ids {
            body.card_ids = card_ids;
        }

        validate_id_type(&body.source_id_type, "source_id_type")?;
        validate_id_type(&body.target_id_type, "target_id_type")?;
        validate_id_list(&body.card_ids, "card_ids")?;

        // url: POST:/open-apis/cardkit/v1/cards/id_convert
        let req: ApiRequest<ConvertCardIdResponse> =
            ApiRequest::post(CARDKIT_V1_CARD_ID_CONVERT).body(serialize_params(&body, "转换 ID")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "转换 ID")
    }
}

/// 转换 ID 请求构建器
#[derive(Debug, Clone)]
pub struct ConvertCardIdRequestBuilder {
    request: ConvertCardIdRequest,
    source_id_type: Option<String>,
    target_id_type: Option<String>,
    card_ids: Option<Vec<String>>,
}

impl ConvertCardIdRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: ConvertCardIdRequest::new(config),
            source_id_type: None,
            target_id_type: None,
            card_ids: None,
        }
    }

    /// 设置源 ID 类型
    pub fn source_id_type(mut self, source_id_type: impl Into<String>) -> Self {
        self.source_id_type = Some(source_id_type.into());
        self
    }

    /// 设置目标 ID 类型
    pub fn target_id_type(mut self, target_id_type: impl Into<String>) -> Self {
        self.target_id_type = Some(target_id_type.into());
        self
    }

    /// 设置卡片 ID 列表
    pub fn card_ids(mut self, card_ids: impl Into<Vec<String>>) -> Self {
        self.card_ids = Some(card_ids.into());
        self
    }

    /// 构建请求
    pub fn build(self) -> ConvertCardIdRequest {
        ConvertCardIdRequest {
            config: self.request.config,
            source_id_type: self.source_id_type,
            target_id_type: self.target_id_type,
            card_ids: self.card_ids,
        }
    }
}

/// 执行请求
///
/// docPath: <https://open.feishu.cn/document/historic-version/id_convert>
pub async fn convert(config: &Config, body: ConvertCardIdBody) -> SDKResult<ConvertCardIdResponse> {
    convert_with_options(config, body, RequestOption::default()).await
}

/// 执行请求（支持自定义选项）
///
/// docPath: <https://open.feishu.cn/document/historic-version/id_convert>
pub async fn convert_with_options(
    config: &Config,
    body: ConvertCardIdBody,
    option: RequestOption,
) -> SDKResult<ConvertCardIdResponse> {
    validate_id_type(&body.source_id_type, "source_id_type")?;
    validate_id_type(&body.target_id_type, "target_id_type")?;
    validate_id_list(&body.card_ids, "card_ids")?;

    // url: POST:/open-apis/cardkit/v1/cards/id_convert
    let req: ApiRequest<ConvertCardIdResponse> =
        ApiRequest::post(CARDKIT_V1_CARD_ID_CONVERT).body(serialize_params(&body, "转换 ID")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "转换 ID")
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
