//! 港澳通行证识别
//!
//! 识别港澳通行证信息。
//!
//! docPath: <https://open.feishu.cn/document/document_ai-v1/hkm_mainland_travel_permit_recognize>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::serialize_params;
use crate::endpoints::DOCUMENT_AI_HKM_MAINLAND_TRAVEL_PERMIT_RECOGNIZE;

/// 港澳通行证识别请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HkmMainlandTravelPermitRecognizeBody {
    /// 识别的港澳居民来往内地通行证源文件。
    #[serde(skip_serializing)]
    pub file: Vec<u8>,
    /// multipart 文件名，仅用于设置文件 part 的 filename。
    #[serde(rename = "__file_name", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

impl HkmMainlandTravelPermitRecognizeBody {
    /// 校验请求体。
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.file, "file 不能为空");
        Ok(())
    }
}

/// 港澳通行证识别响应 `data`。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HkmMainlandTravelPermitRecognizeResponse {
    /// 港澳居民来往内地通行证信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hkm_mainland_travel_permit: Option<HkmMainlandTravelPermit>,
}

impl openlark_core::api::ApiResponseTrait for HkmMainlandTravelPermitRecognizeResponse {}

/// 港澳居民来往内地通行证信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HkmMainlandTravelPermit {
    /// 识别出的实体列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<HkmMainlandTravelPermitEntity>>,
}

/// 港澳居民来往内地通行证识别实体。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HkmMainlandTravelPermitEntity {
    /// 实体类型，可选值有 full_name_cn、full_name_en、date_of_birth、date_of_expiry、card_number。
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    /// 实体值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 港澳通行证识别请求
#[derive(Debug, Clone)]
pub struct HkmMainlandTravelPermitRecognizeRequest {
    config: Config,
}

impl HkmMainlandTravelPermitRecognizeRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(
        self,
        body: HkmMainlandTravelPermitRecognizeBody,
    ) -> SDKResult<HkmMainlandTravelPermitRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: HkmMainlandTravelPermitRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<HkmMainlandTravelPermitRecognizeResponse> {
        body.validate()?;

        let req: ApiRequest<HkmMainlandTravelPermitRecognizeResponse> =
            ApiRequest::post(DOCUMENT_AI_HKM_MAINLAND_TRAVEL_PERMIT_RECOGNIZE)
                .body(serialize_params(&body, "港澳通行证识别")?)
                .file_content(body.file.clone());

        Transport::request_typed(req, &self.config, Some(option), "港澳通行证识别").await
    }
}

/// 港澳通行证识别请求构建器
#[derive(Debug, Clone)]
pub struct HkmMainlandTravelPermitRecognizeRequestBuilder {
    request: HkmMainlandTravelPermitRecognizeRequest,
    file: Option<Vec<u8>>,
    file_name: Option<String>,
}

impl HkmMainlandTravelPermitRecognizeRequestBuilder {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            request: HkmMainlandTravelPermitRecognizeRequest::new(config),
            file: None,
            file_name: None,
        }
    }

    /// 设置识别的港澳居民来往内地通行证源文件。
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
    pub fn body(self) -> HkmMainlandTravelPermitRecognizeBody {
        HkmMainlandTravelPermitRecognizeBody {
            file: self.file.unwrap_or_default(),
            file_name: self.file_name,
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<HkmMainlandTravelPermitRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<HkmMainlandTravelPermitRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行港澳通行证识别。
pub async fn hkm_mainland_travel_permit_recognize(
    config: &Config,
    body: HkmMainlandTravelPermitRecognizeBody,
) -> SDKResult<HkmMainlandTravelPermitRecognizeResponse> {
    hkm_mainland_travel_permit_recognize_with_options(config, body, RequestOption::default()).await
}

/// hkm_mainland_travel_permit_recognize_with_options。
pub async fn hkm_mainland_travel_permit_recognize_with_options(
    config: &Config,
    body: HkmMainlandTravelPermitRecognizeBody,
    option: RequestOption,
) -> SDKResult<HkmMainlandTravelPermitRecognizeResponse> {
    body.validate()?;

    let req: ApiRequest<HkmMainlandTravelPermitRecognizeResponse> =
        ApiRequest::post(DOCUMENT_AI_HKM_MAINLAND_TRAVEL_PERMIT_RECOGNIZE)
            .body(serialize_params(&body, "港澳通行证识别")?)
            .file_content(body.file.clone());

    Transport::request_typed(req, config, Some(option), "港澳通行证识别").await
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
        let builder = HkmMainlandTravelPermitRecognizeRequestBuilder::new(config.clone());
        assert!(builder.file.is_none());
    }

    #[test]
    fn test_body_validation_empty() {
        let body = HkmMainlandTravelPermitRecognizeBody {
            file: Vec::new(),
            file_name: None,
        };
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = HkmMainlandTravelPermitRecognizeBody {
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
        let builder = HkmMainlandTravelPermitRecognizeRequestBuilder::new(config.clone())
            .file_name("file.jpg");
        assert_eq!(builder.file_name, Some("file.jpg".to_string()));
    }

    #[test]
    fn test_builder_body_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let body = HkmMainlandTravelPermitRecognizeRequestBuilder::new(config.clone())
            .file(b"file content".to_vec())
            .file_name("file.jpg")
            .body();
        assert_eq!(body.file, b"file content".to_vec());
        assert_eq!(body.file_name, Some("file.jpg".to_string()));
    }

    #[test]
    fn test_body_serialization() {
        let body = HkmMainlandTravelPermitRecognizeBody {
            file: b"file content".to_vec(),
            file_name: Some("file.jpg".to_string()),
        };
        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(!json_str.contains("\"file\""));
        assert!(json_str.contains("__file_name"));
        assert!(json_str.contains("file.jpg"));
    }

    #[test]
    fn test_hkm_mainland_travel_permit_entity_struct() {
        let entity = HkmMainlandTravelPermitEntity {
            entity_type: Some("full_name_cn".to_string()),
            value: Some("张三".to_string()),
        };
        assert_eq!(entity.entity_type, Some("full_name_cn".to_string()));
        assert_eq!(entity.value, Some("张三".to_string()));
    }

    #[test]
    fn test_hkm_mainland_travel_permit_entity_serialization() {
        let entity = HkmMainlandTravelPermitEntity {
            entity_type: Some("full_name_cn".to_string()),
            value: Some("张三".to_string()),
        };
        let json_str = serde_json::to_string(&entity).expect("序列化失败");
        assert!(json_str.contains("\"type\""));
        assert!(json_str.contains("full_name_cn"));
    }

    #[test]
    fn test_response_struct() {
        let response = HkmMainlandTravelPermitRecognizeResponse {
            hkm_mainland_travel_permit: None,
        };
        assert!(response.hkm_mainland_travel_permit.is_none());

        let response_with_data = HkmMainlandTravelPermitRecognizeResponse {
            hkm_mainland_travel_permit: Some(HkmMainlandTravelPermit {
                entities: Some(vec![HkmMainlandTravelPermitEntity {
                    entity_type: Some("full_name_cn".to_string()),
                    value: Some("张三".to_string()),
                }]),
            }),
        };
        let value = response_with_data
            .hkm_mainland_travel_permit
            .as_ref()
            .and_then(|hkm_mainland_travel_permit| hkm_mainland_travel_permit.entities.as_ref())
            .and_then(|entities| entities.first())
            .and_then(|entity| entity.value.as_deref());
        assert_eq!(value, Some("张三"));
    }
}
