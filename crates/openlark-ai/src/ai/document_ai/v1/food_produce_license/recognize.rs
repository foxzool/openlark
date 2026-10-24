//! 食品生产许可证识别
//!
//! 识别食品生产许可证信息。
//!
//! docPath: <https://open.feishu.cn/document/document_ai-v1/food_produce_license_recognize>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::serialize_params;
use crate::endpoints::DOCUMENT_AI_FOOD_PRODUCE_LICENSE_RECOGNIZE;

/// 食品生产许可证识别请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodProduceLicenseRecognizeBody {
    /// 识别的食品生产许可证源文件。
    #[serde(skip_serializing)]
    pub file: Vec<u8>,
    /// multipart 文件名，仅用于设置文件 part 的 filename。
    #[serde(rename = "__file_name", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

impl FoodProduceLicenseRecognizeBody {
    /// 校验请求体。
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.file, "file 不能为空");
        Ok(())
    }
}

/// 食品生产许可证识别响应 `data`。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodProduceLicenseRecognizeResponse {
    /// 食品生产许可证信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub food_produce_license: Option<FoodProduceLicense>,
}

impl openlark_core::api::ApiResponseTrait for FoodProduceLicenseRecognizeResponse {}

/// 食品生产许可证信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodProduceLicense {
    /// 识别出的实体列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<FoodProduceLicenseEntity>>,
}

/// 食品生产许可证识别实体。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodProduceLicenseEntity {
    /// 实体类型，可选值有 validity_period、issuer、issuing_authority、complaints_hotline、food_category、production_address、license_number、domicile、legal_representative、credit_code、producer、daily_supervisory_authorities、daily_supervisor。
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    /// 实体值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 食品生产许可证识别请求
#[derive(Debug, Clone)]
pub struct FoodProduceLicenseRecognizeRequest {
    config: Config,
}

impl FoodProduceLicenseRecognizeRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(
        self,
        body: FoodProduceLicenseRecognizeBody,
    ) -> SDKResult<FoodProduceLicenseRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: FoodProduceLicenseRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<FoodProduceLicenseRecognizeResponse> {
        body.validate()?;

        let req: ApiRequest<FoodProduceLicenseRecognizeResponse> =
            ApiRequest::post(DOCUMENT_AI_FOOD_PRODUCE_LICENSE_RECOGNIZE)
                .body(serialize_params(&body, "食品生产许可证识别")?)
                .file_content(body.file.clone());

        Transport::request_typed(req, &self.config, Some(option), "食品生产许可证识别").await
    }
}

/// 食品生产许可证识别请求构建器
#[derive(Debug, Clone)]
pub struct FoodProduceLicenseRecognizeRequestBuilder {
    request: FoodProduceLicenseRecognizeRequest,
    file: Option<Vec<u8>>,
    file_name: Option<String>,
}

impl FoodProduceLicenseRecognizeRequestBuilder {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            request: FoodProduceLicenseRecognizeRequest::new(config),
            file: None,
            file_name: None,
        }
    }

    /// 设置识别的食品生产许可证源文件。
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
    pub fn body(self) -> FoodProduceLicenseRecognizeBody {
        FoodProduceLicenseRecognizeBody {
            file: self.file.unwrap_or_default(),
            file_name: self.file_name,
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<FoodProduceLicenseRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<FoodProduceLicenseRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行食品生产许可证识别。
pub async fn food_produce_license_recognize(
    config: &Config,
    body: FoodProduceLicenseRecognizeBody,
) -> SDKResult<FoodProduceLicenseRecognizeResponse> {
    food_produce_license_recognize_with_options(config, body, RequestOption::default()).await
}

/// food_produce_license_recognize_with_options。
pub async fn food_produce_license_recognize_with_options(
    config: &Config,
    body: FoodProduceLicenseRecognizeBody,
    option: RequestOption,
) -> SDKResult<FoodProduceLicenseRecognizeResponse> {
    body.validate()?;

    let req: ApiRequest<FoodProduceLicenseRecognizeResponse> =
        ApiRequest::post(DOCUMENT_AI_FOOD_PRODUCE_LICENSE_RECOGNIZE)
            .body(serialize_params(&body, "食品生产许可证识别")?)
            .file_content(body.file.clone());

    Transport::request_typed(req, config, Some(option), "食品生产许可证识别").await
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
        let builder = FoodProduceLicenseRecognizeRequestBuilder::new(config.clone());
        assert!(builder.file.is_none());
    }

    #[test]
    fn test_body_validation_empty() {
        let body = FoodProduceLicenseRecognizeBody {
            file: Vec::new(),
            file_name: None,
        };
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = FoodProduceLicenseRecognizeBody {
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
            FoodProduceLicenseRecognizeRequestBuilder::new(config.clone()).file_name("file.jpg");
        assert_eq!(builder.file_name, Some("file.jpg".to_string()));
    }

    #[test]
    fn test_builder_body_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let body = FoodProduceLicenseRecognizeRequestBuilder::new(config.clone())
            .file(b"file content".to_vec())
            .file_name("file.jpg")
            .body();
        assert_eq!(body.file, b"file content".to_vec());
        assert_eq!(body.file_name, Some("file.jpg".to_string()));
    }

    #[test]
    fn test_body_serialization() {
        let body = FoodProduceLicenseRecognizeBody {
            file: b"file content".to_vec(),
            file_name: Some("file.jpg".to_string()),
        };
        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(!json_str.contains("\"file\""));
        assert!(json_str.contains("__file_name"));
        assert!(json_str.contains("file.jpg"));
    }

    #[test]
    fn test_food_produce_license_entity_struct() {
        let entity = FoodProduceLicenseEntity {
            entity_type: Some("license_number".to_string()),
            value: Some("SC12345678901234".to_string()),
        };
        assert_eq!(entity.entity_type, Some("license_number".to_string()));
        assert_eq!(entity.value, Some("SC12345678901234".to_string()));
    }

    #[test]
    fn test_food_produce_license_entity_serialization() {
        let entity = FoodProduceLicenseEntity {
            entity_type: Some("license_number".to_string()),
            value: Some("SC12345678901234".to_string()),
        };
        let json_str = serde_json::to_string(&entity).expect("序列化失败");
        assert!(json_str.contains("\"type\""));
        assert!(json_str.contains("license_number"));
    }

    #[test]
    fn test_response_struct() {
        let response = FoodProduceLicenseRecognizeResponse {
            food_produce_license: None,
        };
        assert!(response.food_produce_license.is_none());

        let response_with_data = FoodProduceLicenseRecognizeResponse {
            food_produce_license: Some(FoodProduceLicense {
                entities: Some(vec![FoodProduceLicenseEntity {
                    entity_type: Some("license_number".to_string()),
                    value: Some("SC12345678901234".to_string()),
                }]),
            }),
        };
        let value = response_with_data
            .food_produce_license
            .as_ref()
            .and_then(|food_produce_license| food_produce_license.entities.as_ref())
            .and_then(|entities| entities.first())
            .and_then(|entity| entity.value.as_deref());
        assert_eq!(value, Some("SC12345678901234"));
    }
}
