//! 中国护照识别
//!
//! 识别中国护照中的关键信息。
//!
//! docPath: <https://open.feishu.cn/document/document_ai-v1/chinese_passport_recognize>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::serialize_params;
use crate::endpoints::DOCUMENT_AI_CHINESE_PASSPORT_RECOGNIZE;

/// 中国护照识别请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChinesePassportRecognizeBody {
    /// 识别的中国护照源文件。
    #[serde(skip_serializing)]
    pub file: Vec<u8>,
    /// multipart 文件名，仅用于设置文件 part 的 filename。
    #[serde(rename = "__file_name", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

impl ChinesePassportRecognizeBody {
    /// 校验请求体。
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.file, "file 不能为空");
        Ok(())
    }
}

/// 中国护照识别响应 `data`。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChinesePassportRecognizeResponse {
    /// 中国护照信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chinese_passport: Option<ChinesePassport>,
}

impl openlark_core::api::ApiResponseTrait for ChinesePassportRecognizeResponse {}

/// 中国护照信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChinesePassport {
    /// 识别出的实体列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<ChinesePassportEntity>>,
}

/// 中国护照识别实体。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChinesePassportEntity {
    /// 实体类型，可选值有 full_name_cn、full_name_en、date_of_birth、date_of_expiry、place_of_issue、passport_number。
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    /// 实体值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 中国护照识别请求
#[derive(Debug, Clone)]
pub struct ChinesePassportRecognizeRequest {
    config: Config,
}

impl ChinesePassportRecognizeRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(
        self,
        body: ChinesePassportRecognizeBody,
    ) -> SDKResult<ChinesePassportRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: ChinesePassportRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<ChinesePassportRecognizeResponse> {
        body.validate()?;

        let req: ApiRequest<ChinesePassportRecognizeResponse> =
            ApiRequest::post(DOCUMENT_AI_CHINESE_PASSPORT_RECOGNIZE)
                .body(serialize_params(&body, "中国护照识别")?)
                .file_content(body.file.clone());

        Transport::request_typed(req, &self.config, Some(option), "中国护照识别").await
    }
}

/// 中国护照识别请求构建器
#[derive(Debug, Clone)]
pub struct ChinesePassportRecognizeRequestBuilder {
    request: ChinesePassportRecognizeRequest,
    file: Option<Vec<u8>>,
    file_name: Option<String>,
}

impl ChinesePassportRecognizeRequestBuilder {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            request: ChinesePassportRecognizeRequest::new(config),
            file: None,
            file_name: None,
        }
    }

    /// 设置识别的中国护照源文件。
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
    pub fn body(self) -> ChinesePassportRecognizeBody {
        ChinesePassportRecognizeBody {
            file: self.file.unwrap_or_default(),
            file_name: self.file_name,
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ChinesePassportRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ChinesePassportRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行中国护照识别。
pub async fn chinese_passport_recognize(
    config: &Config,
    body: ChinesePassportRecognizeBody,
) -> SDKResult<ChinesePassportRecognizeResponse> {
    chinese_passport_recognize_with_options(config, body, RequestOption::default()).await
}

/// chinese_passport_recognize_with_options。
pub async fn chinese_passport_recognize_with_options(
    config: &Config,
    body: ChinesePassportRecognizeBody,
    option: RequestOption,
) -> SDKResult<ChinesePassportRecognizeResponse> {
    body.validate()?;

    let req: ApiRequest<ChinesePassportRecognizeResponse> =
        ApiRequest::post(DOCUMENT_AI_CHINESE_PASSPORT_RECOGNIZE)
            .body(serialize_params(&body, "中国护照识别")?)
            .file_content(body.file.clone());

    Transport::request_typed(req, config, Some(option), "中国护照识别").await
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
        let builder = ChinesePassportRecognizeRequestBuilder::new(config.clone());
        assert!(builder.file.is_none());
    }

    #[test]
    fn test_body_validation_empty() {
        let body = ChinesePassportRecognizeBody {
            file: Vec::new(),
            file_name: None,
        };
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = ChinesePassportRecognizeBody {
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
            ChinesePassportRecognizeRequestBuilder::new(config.clone()).file_name("file.jpg");
        assert_eq!(builder.file_name, Some("file.jpg".to_string()));
    }

    #[test]
    fn test_builder_body_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let body = ChinesePassportRecognizeRequestBuilder::new(config.clone())
            .file(b"file content".to_vec())
            .file_name("file.jpg")
            .body();
        assert_eq!(body.file, b"file content".to_vec());
        assert_eq!(body.file_name, Some("file.jpg".to_string()));
    }

    #[test]
    fn test_body_serialization() {
        let body = ChinesePassportRecognizeBody {
            file: b"file content".to_vec(),
            file_name: Some("file.jpg".to_string()),
        };
        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(!json_str.contains("\"file\""));
        assert!(json_str.contains("__file_name"));
        assert!(json_str.contains("file.jpg"));
    }

    #[test]
    fn test_chinese_passport_entity_struct() {
        let entity = ChinesePassportEntity {
            entity_type: Some("passport_number".to_string()),
            value: Some("E12345678".to_string()),
        };
        assert_eq!(entity.entity_type, Some("passport_number".to_string()));
        assert_eq!(entity.value, Some("E12345678".to_string()));
    }

    #[test]
    fn test_chinese_passport_entity_serialization() {
        let entity = ChinesePassportEntity {
            entity_type: Some("passport_number".to_string()),
            value: Some("E12345678".to_string()),
        };
        let json_str = serde_json::to_string(&entity).expect("序列化失败");
        assert!(json_str.contains("\"type\""));
        assert!(json_str.contains("passport_number"));
    }

    #[test]
    fn test_response_struct() {
        let response = ChinesePassportRecognizeResponse {
            chinese_passport: None,
        };
        assert!(response.chinese_passport.is_none());

        let response_with_data = ChinesePassportRecognizeResponse {
            chinese_passport: Some(ChinesePassport {
                entities: Some(vec![ChinesePassportEntity {
                    entity_type: Some("passport_number".to_string()),
                    value: Some("E12345678".to_string()),
                }]),
            }),
        };
        let value = response_with_data
            .chinese_passport
            .as_ref()
            .and_then(|chinese_passport| chinese_passport.entities.as_ref())
            .and_then(|entities| entities.first())
            .and_then(|entity| entity.value.as_deref());
        assert_eq!(value, Some("E12345678"));
    }
}
