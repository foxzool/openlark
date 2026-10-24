//! 行驶证识别
//!
//! 识别行驶证信息。
//!
//! docPath: <https://open.feishu.cn/document/document_ai-v1/vehicle_license_recognize>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::serialize_params;
use crate::endpoints::DOCUMENT_AI_VEHICLE_LICENSE_RECOGNIZE;

/// 行驶证识别请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleLicenseRecognizeBody {
    /// 识别的行驶证源文件。
    #[serde(skip_serializing)]
    pub file: Vec<u8>,
    /// multipart 文件名，仅用于设置文件 part 的 filename。
    #[serde(rename = "__file_name", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

impl VehicleLicenseRecognizeBody {
    /// 校验请求体。
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.file, "file 不能为空");
        Ok(())
    }
}

/// 行驶证识别响应 `data`。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleLicenseRecognizeResponse {
    /// 行驶证信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_license: Option<VehicleLicense>,
}

impl openlark_core::api::ApiResponseTrait for VehicleLicenseRecognizeResponse {}

/// 行驶证信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleLicense {
    /// 识别出的实体列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<VehicleLicenseEntity>>,
}

/// 行驶证识别实体。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleLicenseEntity {
    /// 实体类型，可选值有 plate_number、vehicle_type、owner、address、use_character、model、vin、engine_number、register_date、issue_date、license_issuing_authority、document_id、approved_passengers_capacity、total_mass、curb_weight、ratified_load_capacity、gabarite、traction_mass、remarks、inspection_record。
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    /// 实体值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 行驶证识别请求
#[derive(Debug, Clone)]
pub struct VehicleLicenseRecognizeRequest {
    config: Config,
}

impl VehicleLicenseRecognizeRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(
        self,
        body: VehicleLicenseRecognizeBody,
    ) -> SDKResult<VehicleLicenseRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: VehicleLicenseRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<VehicleLicenseRecognizeResponse> {
        body.validate()?;

        let req: ApiRequest<VehicleLicenseRecognizeResponse> =
            ApiRequest::post(DOCUMENT_AI_VEHICLE_LICENSE_RECOGNIZE)
                .body(serialize_params(&body, "行驶证识别")?)
                .file_content(body.file.clone());

        Transport::request_typed(req, &self.config, Some(option), "行驶证识别").await
    }
}

/// 行驶证识别请求构建器
#[derive(Debug, Clone)]
pub struct VehicleLicenseRecognizeRequestBuilder {
    request: VehicleLicenseRecognizeRequest,
    file: Option<Vec<u8>>,
    file_name: Option<String>,
}

impl VehicleLicenseRecognizeRequestBuilder {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            request: VehicleLicenseRecognizeRequest::new(config),
            file: None,
            file_name: None,
        }
    }

    /// 设置识别的行驶证源文件。
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
    pub fn body(self) -> VehicleLicenseRecognizeBody {
        VehicleLicenseRecognizeBody {
            file: self.file.unwrap_or_default(),
            file_name: self.file_name,
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<VehicleLicenseRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<VehicleLicenseRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行行驶证识别。
pub async fn vehicle_license_recognize(
    config: &Config,
    body: VehicleLicenseRecognizeBody,
) -> SDKResult<VehicleLicenseRecognizeResponse> {
    vehicle_license_recognize_with_options(config, body, RequestOption::default()).await
}

/// vehicle_license_recognize_with_options。
pub async fn vehicle_license_recognize_with_options(
    config: &Config,
    body: VehicleLicenseRecognizeBody,
    option: RequestOption,
) -> SDKResult<VehicleLicenseRecognizeResponse> {
    body.validate()?;

    let req: ApiRequest<VehicleLicenseRecognizeResponse> =
        ApiRequest::post(DOCUMENT_AI_VEHICLE_LICENSE_RECOGNIZE)
            .body(serialize_params(&body, "行驶证识别")?)
            .file_content(body.file.clone());

    Transport::request_typed(req, config, Some(option), "行驶证识别").await
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
        let builder = VehicleLicenseRecognizeRequestBuilder::new(config.clone());
        assert!(builder.file.is_none());
    }

    #[test]
    fn test_body_validation_empty() {
        let body = VehicleLicenseRecognizeBody {
            file: Vec::new(),
            file_name: None,
        };
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = VehicleLicenseRecognizeBody {
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
            VehicleLicenseRecognizeRequestBuilder::new(config.clone()).file_name("file.jpg");
        assert_eq!(builder.file_name, Some("file.jpg".to_string()));
    }

    #[test]
    fn test_builder_body_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let body = VehicleLicenseRecognizeRequestBuilder::new(config.clone())
            .file(b"file content".to_vec())
            .file_name("file.jpg")
            .body();
        assert_eq!(body.file, b"file content".to_vec());
        assert_eq!(body.file_name, Some("file.jpg".to_string()));
    }

    #[test]
    fn test_body_serialization() {
        let body = VehicleLicenseRecognizeBody {
            file: b"file content".to_vec(),
            file_name: Some("file.jpg".to_string()),
        };
        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(!json_str.contains("\"file\""));
        assert!(json_str.contains("__file_name"));
        assert!(json_str.contains("file.jpg"));
    }

    #[test]
    fn test_vehicle_license_entity_struct() {
        let entity = VehicleLicenseEntity {
            entity_type: Some("plate_number".to_string()),
            value: Some("京A12345".to_string()),
        };
        assert_eq!(entity.entity_type, Some("plate_number".to_string()));
        assert_eq!(entity.value, Some("京A12345".to_string()));
    }

    #[test]
    fn test_vehicle_license_entity_serialization() {
        let entity = VehicleLicenseEntity {
            entity_type: Some("plate_number".to_string()),
            value: Some("京A12345".to_string()),
        };
        let json_str = serde_json::to_string(&entity).expect("序列化失败");
        assert!(json_str.contains("\"type\""));
        assert!(json_str.contains("plate_number"));
    }

    #[test]
    fn test_response_struct() {
        let response = VehicleLicenseRecognizeResponse {
            vehicle_license: None,
        };
        assert!(response.vehicle_license.is_none());

        let response_with_data = VehicleLicenseRecognizeResponse {
            vehicle_license: Some(VehicleLicense {
                entities: Some(vec![VehicleLicenseEntity {
                    entity_type: Some("plate_number".to_string()),
                    value: Some("京A12345".to_string()),
                }]),
            }),
        };
        let value = response_with_data
            .vehicle_license
            .as_ref()
            .and_then(|vehicle_license| vehicle_license.entities.as_ref())
            .and_then(|entities| entities.first())
            .and_then(|entity| entity.value.as_deref());
        assert_eq!(value, Some("京A12345"));
    }
}
