//! 名片识别
//!
//! 智能识别名片中的联系信息。
//!
//! docPath: <https://open.feishu.cn/document/document_ai-v1/business_card_recognize>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::serialize_params;
use crate::endpoints::DOCUMENT_AI_BUSINESS_CARD_RECOGNIZE;

/// 名片识别请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessCardRecognizeBody {
    /// 识别名片的源文件（支持 JPG / PNG / PDF）。
    #[serde(skip_serializing)]
    pub file: Vec<u8>,
    /// multipart 文件名，仅用于设置文件 part 的 filename。
    #[serde(rename = "__file_name", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

impl BusinessCardRecognizeBody {
    /// 校验请求体。
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.file, "file 不能为空");
        Ok(())
    }
}

/// 名片识别响应 `data`。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessCardRecognizeResponse {
    /// 名片信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_cards: Option<Vec<BusinessCard>>,
}

impl openlark_core::api::ApiResponseTrait for BusinessCardRecognizeResponse {}

/// 名片信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessCard {
    /// 识别出的实体列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<BusinessCardEntity>>,
}

/// 名片识别实体。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessCardEntity {
    /// 实体类型，识别的字段种类。
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    /// 实体值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 名片识别请求
#[derive(Debug, Clone)]
pub struct BusinessCardRecognizeRequest {
    config: Config,
}

impl BusinessCardRecognizeRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(
        self,
        body: BusinessCardRecognizeBody,
    ) -> SDKResult<BusinessCardRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: BusinessCardRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<BusinessCardRecognizeResponse> {
        body.validate()?;

        let req: ApiRequest<BusinessCardRecognizeResponse> =
            ApiRequest::post(DOCUMENT_AI_BUSINESS_CARD_RECOGNIZE)
                .body(serialize_params(&body, "名片识别")?)
                .file_content(body.file.clone());

        Transport::request_typed(req, &self.config, Some(option), "名片识别").await
    }
}

/// 名片识别请求构建器
#[derive(Debug, Clone)]
pub struct BusinessCardRecognizeRequestBuilder {
    request: BusinessCardRecognizeRequest,
    file: Option<Vec<u8>>,
    file_name: Option<String>,
}

impl BusinessCardRecognizeRequestBuilder {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            request: BusinessCardRecognizeRequest::new(config),
            file: None,
            file_name: None,
        }
    }

    /// 设置识别名片的源文件（支持 JPG / PNG / PDF）。
    pub fn file(mut self, file: impl Into<Vec<u8>>) -> Self {
        self.file = Some(file.into());
        self
    }

    /// 设置 multipart 文件名。
    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = Some(file_name.into());
        self
    }

    /// 构建请求体。
    pub fn body(self) -> BusinessCardRecognizeBody {
        BusinessCardRecognizeBody {
            file: self.file.unwrap_or_default(),
            file_name: self.file_name,
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<BusinessCardRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BusinessCardRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行名片识别。
pub async fn business_card_recognize(
    config: &Config,
    body: BusinessCardRecognizeBody,
) -> SDKResult<BusinessCardRecognizeResponse> {
    business_card_recognize_with_options(config, body, RequestOption::default()).await
}

/// business_card_recognize_with_options。
pub async fn business_card_recognize_with_options(
    config: &Config,
    body: BusinessCardRecognizeBody,
    option: RequestOption,
) -> SDKResult<BusinessCardRecognizeResponse> {
    body.validate()?;

    let req: ApiRequest<BusinessCardRecognizeResponse> =
        ApiRequest::post(DOCUMENT_AI_BUSINESS_CARD_RECOGNIZE)
            .body(serialize_params(&body, "名片识别")?)
            .file_content(body.file.clone());

    Transport::request_typed(req, config, Some(option), "名片识别").await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_default_state() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = BusinessCardRecognizeRequestBuilder::new(config.clone());
        assert!(builder.file.is_none());
    }

    #[test]
    fn test_body_validation_empty() {
        let body = BusinessCardRecognizeBody {
            file: Vec::new(),
            file_name: None,
        };
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = BusinessCardRecognizeBody {
            file: b"file content".to_vec(),
            file_name: None,
        };
        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_builder_file_name() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder =
            BusinessCardRecognizeRequestBuilder::new(config.clone()).file_name("file.jpg");
        assert_eq!(builder.file_name, Some("file.jpg".to_string()));
    }

    #[test]
    fn test_builder_body_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let body = BusinessCardRecognizeRequestBuilder::new(config.clone())
            .file(b"file content".to_vec())
            .file_name("file.jpg")
            .body();
        assert_eq!(body.file, b"file content".to_vec());
        assert_eq!(body.file_name, Some("file.jpg".to_string()));
    }

    #[test]
    fn test_body_serialization() {
        let body = BusinessCardRecognizeBody {
            file: b"file content".to_vec(),
            file_name: Some("file.jpg".to_string()),
        };
        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(!json_str.contains("\"file\""));
        assert!(json_str.contains("__file_name"));
        assert!(json_str.contains("file.jpg"));
    }

    #[test]
    fn test_business_card_entity_struct() {
        let entity = BusinessCardEntity {
            entity_type: Some("mobile".to_string()),
            value: Some("13800138000".to_string()),
        };
        assert_eq!(entity.entity_type, Some("mobile".to_string()));
        assert_eq!(entity.value, Some("13800138000".to_string()));
    }

    #[test]
    fn test_business_card_entity_serialization() {
        let entity = BusinessCardEntity {
            entity_type: Some("mobile".to_string()),
            value: Some("13800138000".to_string()),
        };
        let json_str = serde_json::to_string(&entity).expect("序列化失败");
        assert!(json_str.contains("\"type\""));
        assert!(json_str.contains("mobile"));
    }

    #[test]
    fn test_response_struct() {
        let response = BusinessCardRecognizeResponse {
            business_cards: None,
        };
        assert!(response.business_cards.is_none());

        let response_with_data = BusinessCardRecognizeResponse {
            business_cards: Some(vec![BusinessCard {
                entities: Some(vec![BusinessCardEntity {
                    entity_type: Some("mobile".to_string()),
                    value: Some("13800138000".to_string()),
                }]),
            }]),
        };
        let value = response_with_data
            .business_cards
            .as_ref()
            .and_then(|items| items.first())
            .and_then(|business_card| business_card.entities.as_ref())
            .and_then(|entities| entities.first())
            .and_then(|entity| entity.value.as_deref());
        assert_eq!(value, Some("13800138000"));
    }
}
