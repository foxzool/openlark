//! 食品经营许可证识别
//!
//! 识别食品经营许可证信息。
//!
//! docPath: <https://open.feishu.cn/document/document_ai-v1/food_manage_license_recognize>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::serialize_params;
use crate::endpoints::DOCUMENT_AI_FOOD_MANAGE_LICENSE_RECOGNIZE;

/// 食品经营许可证识别请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodManageLicenseRecognizeBody {
    /// 识别的食品经营许可证源文件。
    #[serde(skip_serializing)]
    pub file: Vec<u8>,
    /// multipart 文件名，仅用于设置文件 part 的 filename。
    #[serde(rename = "__file_name", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

impl FoodManageLicenseRecognizeBody {
    /// 校验请求体。
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.file, "file 不能为空");
        Ok(())
    }
}

/// 食品经营许可证识别响应 `data`。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodManageLicenseRecognizeResponse {
    /// 食品经营许可证信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub food_manage_license: Option<FoodManageLicense>,
}

impl openlark_core::api::ApiResponseTrait for FoodManageLicenseRecognizeResponse {}

/// 食品经营许可证信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodManageLicense {
    /// 识别出的实体列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<FoodManageLicenseEntity>>,
}

/// 食品经营许可证识别实体。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodManageLicenseEntity {
    /// 实体类型，可选值有 validity_period、issuer、issuing_authority、complaints_hotline、license_number、domicile、legal_representative、credit_code、operator、premise、daily_supervisor、daily_supervisory_authorities、main_body、operating_item。
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    /// 实体值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 食品经营许可证识别请求
#[derive(Debug, Clone)]
pub struct FoodManageLicenseRecognizeRequest {
    config: Config,
}

impl FoodManageLicenseRecognizeRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(
        self,
        body: FoodManageLicenseRecognizeBody,
    ) -> SDKResult<FoodManageLicenseRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: FoodManageLicenseRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<FoodManageLicenseRecognizeResponse> {
        body.validate()?;

        let req: ApiRequest<FoodManageLicenseRecognizeResponse> =
            ApiRequest::post(DOCUMENT_AI_FOOD_MANAGE_LICENSE_RECOGNIZE)
                .body(serialize_params(&body, "食品经营许可证识别")?)
                .file_content(body.file.clone());

        Transport::request_typed(req, &self.config, Some(option), "食品经营许可证识别").await
    }
}

/// 食品经营许可证识别请求构建器
#[derive(Debug, Clone)]
pub struct FoodManageLicenseRecognizeRequestBuilder {
    request: FoodManageLicenseRecognizeRequest,
    file: Option<Vec<u8>>,
    file_name: Option<String>,
}

impl FoodManageLicenseRecognizeRequestBuilder {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            request: FoodManageLicenseRecognizeRequest::new(config),
            file: None,
            file_name: None,
        }
    }

    /// 设置识别的食品经营许可证源文件。
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
    pub fn body(self) -> FoodManageLicenseRecognizeBody {
        FoodManageLicenseRecognizeBody {
            file: self.file.unwrap_or_default(),
            file_name: self.file_name,
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<FoodManageLicenseRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<FoodManageLicenseRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行食品经营许可证识别。
pub async fn food_manage_license_recognize(
    config: &Config,
    body: FoodManageLicenseRecognizeBody,
) -> SDKResult<FoodManageLicenseRecognizeResponse> {
    food_manage_license_recognize_with_options(config, body, RequestOption::default()).await
}

/// food_manage_license_recognize_with_options。
pub async fn food_manage_license_recognize_with_options(
    config: &Config,
    body: FoodManageLicenseRecognizeBody,
    option: RequestOption,
) -> SDKResult<FoodManageLicenseRecognizeResponse> {
    body.validate()?;

    let req: ApiRequest<FoodManageLicenseRecognizeResponse> =
        ApiRequest::post(DOCUMENT_AI_FOOD_MANAGE_LICENSE_RECOGNIZE)
            .body(serialize_params(&body, "食品经营许可证识别")?)
            .file_content(body.file.clone());

    Transport::request_typed(req, config, Some(option), "食品经营许可证识别").await
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
        let builder = FoodManageLicenseRecognizeRequestBuilder::new(config.clone());
        assert!(builder.file.is_none());
    }

    #[test]
    fn test_body_validation_empty() {
        let body = FoodManageLicenseRecognizeBody {
            file: Vec::new(),
            file_name: None,
        };
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = FoodManageLicenseRecognizeBody {
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
            FoodManageLicenseRecognizeRequestBuilder::new(config.clone()).file_name("file.jpg");
        assert_eq!(builder.file_name, Some("file.jpg".to_string()));
    }

    #[test]
    fn test_builder_body_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let body = FoodManageLicenseRecognizeRequestBuilder::new(config.clone())
            .file(b"file content".to_vec())
            .file_name("file.jpg")
            .body();
        assert_eq!(body.file, b"file content".to_vec());
        assert_eq!(body.file_name, Some("file.jpg".to_string()));
    }

    #[test]
    fn test_body_serialization() {
        let body = FoodManageLicenseRecognizeBody {
            file: b"file content".to_vec(),
            file_name: Some("file.jpg".to_string()),
        };
        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(!json_str.contains("\"file\""));
        assert!(json_str.contains("__file_name"));
        assert!(json_str.contains("file.jpg"));
    }

    #[test]
    fn test_food_manage_license_entity_struct() {
        let entity = FoodManageLicenseEntity {
            entity_type: Some("issuer".to_string()),
            value: Some("张三".to_string()),
        };
        assert_eq!(entity.entity_type, Some("issuer".to_string()));
        assert_eq!(entity.value, Some("张三".to_string()));
    }

    #[test]
    fn test_food_manage_license_entity_serialization() {
        let entity = FoodManageLicenseEntity {
            entity_type: Some("issuer".to_string()),
            value: Some("张三".to_string()),
        };
        let json_str = serde_json::to_string(&entity).expect("序列化失败");
        assert!(json_str.contains("\"type\""));
        assert!(json_str.contains("issuer"));
    }

    #[test]
    fn test_response_struct() {
        let response = FoodManageLicenseRecognizeResponse {
            food_manage_license: None,
        };
        assert!(response.food_manage_license.is_none());

        let response_with_data = FoodManageLicenseRecognizeResponse {
            food_manage_license: Some(FoodManageLicense {
                entities: Some(vec![FoodManageLicenseEntity {
                    entity_type: Some("issuer".to_string()),
                    value: Some("张三".to_string()),
                }]),
            }),
        };
        let value = response_with_data
            .food_manage_license
            .as_ref()
            .and_then(|food_manage_license| food_manage_license.entities.as_ref())
            .and_then(|entities| entities.first())
            .and_then(|entity| entity.value.as_deref());
        assert_eq!(value, Some("张三"));
    }
}
