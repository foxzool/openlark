//! 身份证识别
//!
//! 识别身份证中的关键信息。
//!
//! docPath: <https://open.feishu.cn/document/document_ai-v1/id_card_recognize>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::serialize_params;
use crate::endpoints::DOCUMENT_AI_ID_CARD_RECOGNIZE;

/// 身份证识别请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdCardRecognizeBody {
    /// 识别身份证的源文件。
    #[serde(skip_serializing)]
    pub file: Vec<u8>,
    /// multipart 文件名，仅用于设置文件 part 的 filename。
    #[serde(rename = "__file_name", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

impl IdCardRecognizeBody {
    /// 校验请求体。
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.file, "file 不能为空");
        Ok(())
    }
}

/// 身份证识别响应 `data`。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdCardRecognizeResponse {
    /// 身份证信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_card: Option<IdCard>,
}

impl openlark_core::api::ApiResponseTrait for IdCardRecognizeResponse {}

/// 身份证信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdCard {
    /// 识别的实体列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<IdEntity>>,
    /// 正反面，1 为身份证-姓名页，0 为身份证-国徽页。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<i32>,
    /// 四角坐标 [x0, y0, x1, y1, x2, y2, x3, y3]。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conners: Option<Vec<i64>>,
}

/// 身份证识别实体。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdEntity {
    /// 实体类型，可选值有 identity_code（公民身份号码）、identity_name（姓名）、address（住址）、valid_date_start、valid_date_end、gender、race、issued_by、birth。
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    /// 实体值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 身份证识别请求
#[derive(Debug, Clone)]
pub struct IdCardRecognizeRequest {
    config: Config,
}

impl IdCardRecognizeRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(self, body: IdCardRecognizeBody) -> SDKResult<IdCardRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: IdCardRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<IdCardRecognizeResponse> {
        body.validate()?;

        let req: ApiRequest<IdCardRecognizeResponse> =
            ApiRequest::post(DOCUMENT_AI_ID_CARD_RECOGNIZE)
                .body(serialize_params(&body, "身份证识别")?)
                .file_content(body.file.clone());

        Transport::request_typed(req, &self.config, Some(option), "身份证识别").await
    }
}

/// 身份证识别请求构建器
#[derive(Debug, Clone)]
pub struct IdCardRecognizeRequestBuilder {
    request: IdCardRecognizeRequest,
    file: Option<Vec<u8>>,
    file_name: Option<String>,
}

impl IdCardRecognizeRequestBuilder {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            request: IdCardRecognizeRequest::new(config),
            file: None,
            file_name: None,
        }
    }

    /// 设置识别身份证的源文件。
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
    pub fn body(self) -> IdCardRecognizeBody {
        IdCardRecognizeBody {
            file: self.file.unwrap_or_default(),
            file_name: self.file_name,
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<IdCardRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<IdCardRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行身份证识别。
pub async fn id_card_recognize(
    config: &Config,
    body: IdCardRecognizeBody,
) -> SDKResult<IdCardRecognizeResponse> {
    id_card_recognize_with_options(config, body, RequestOption::default()).await
}

/// id_card_recognize_with_options。
pub async fn id_card_recognize_with_options(
    config: &Config,
    body: IdCardRecognizeBody,
    option: RequestOption,
) -> SDKResult<IdCardRecognizeResponse> {
    body.validate()?;

    let req: ApiRequest<IdCardRecognizeResponse> = ApiRequest::post(DOCUMENT_AI_ID_CARD_RECOGNIZE)
        .body(serialize_params(&body, "身份证识别")?)
        .file_content(body.file.clone());

    Transport::request_typed(req, config, Some(option), "身份证识别").await
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
        let builder = IdCardRecognizeRequestBuilder::new(config.clone());
        assert!(builder.file.is_none());
    }

    #[test]
    fn test_body_validation_empty() {
        let body = IdCardRecognizeBody {
            file: Vec::new(),
            file_name: None,
        };
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = IdCardRecognizeBody {
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
        let builder = IdCardRecognizeRequestBuilder::new(config.clone()).file_name("file.jpg");
        assert_eq!(builder.file_name, Some("file.jpg".to_string()));
    }

    #[test]
    fn test_builder_body_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let body = IdCardRecognizeRequestBuilder::new(config.clone())
            .file(b"file content".to_vec())
            .file_name("file.jpg")
            .body();
        assert_eq!(body.file, b"file content".to_vec());
        assert_eq!(body.file_name, Some("file.jpg".to_string()));
    }

    #[test]
    fn test_body_serialization() {
        let body = IdCardRecognizeBody {
            file: b"file content".to_vec(),
            file_name: Some("file.jpg".to_string()),
        };
        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(!json_str.contains("\"file\""));
        assert!(json_str.contains("__file_name"));
        assert!(json_str.contains("file.jpg"));
    }

    #[test]
    fn test_id_entity_struct() {
        let entity = IdEntity {
            entity_type: Some("identity_name".to_string()),
            value: Some("张三".to_string()),
        };
        assert_eq!(entity.entity_type, Some("identity_name".to_string()));
        assert_eq!(entity.value, Some("张三".to_string()));
    }

    #[test]
    fn test_id_entity_serialization() {
        let entity = IdEntity {
            entity_type: Some("identity_name".to_string()),
            value: Some("张三".to_string()),
        };
        let json_str = serde_json::to_string(&entity).expect("序列化失败");
        assert!(json_str.contains("\"type\""));
        assert!(json_str.contains("identity_name"));
    }

    #[test]
    fn test_response_struct() {
        let response = IdCardRecognizeResponse { id_card: None };
        assert!(response.id_card.is_none());

        let response_with_data = IdCardRecognizeResponse {
            id_card: Some(IdCard {
                entities: Some(vec![IdEntity {
                    entity_type: Some("identity_name".to_string()),
                    value: Some("张三".to_string()),
                }]),
                side: Some(1),
                conners: Some(vec![1, 2, 3, 4]),
            }),
        };
        let value = response_with_data
            .id_card
            .as_ref()
            .and_then(|id_card| id_card.entities.as_ref())
            .and_then(|entities| entities.first())
            .and_then(|entity| entity.value.as_deref());
        assert_eq!(value, Some("张三"));
    }
}
