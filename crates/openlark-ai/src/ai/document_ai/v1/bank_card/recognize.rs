//! 银行卡识别
//!
//! 识别银行卡中的关键信息。
//!
//! docPath: https://open.feishu.cn/document/document_ai-v1/bank_card_recognize

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::DOCUMENT_AI_BANK_CARD_RECOGNIZE;

/// 银行卡识别请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankCardRecognizeBody {
    /// 识别的银行卡源文件。
    #[serde(skip_serializing)]
    pub file: Vec<u8>,
    /// multipart 文件名，仅用于设置文件 part 的 filename。
    #[serde(rename = "__file_name", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

impl BankCardRecognizeBody {
    /// 校验请求体。
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.file, "file 不能为空");
        Ok(())
    }
<<<<<<< HEAD
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.file, "file 不能为空");
=======
    pub fn validate(&self) -> Result<(), String> {
        if self.file.is_empty() {
            return Err("file 不能为空".to_string());
        }
>>>>>>> origin/main
        Ok(())
    }
}

/// 银行卡识别响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankCardRecognizeResponse {
    /// data 字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<BankCardRecognizeResult>,
}

impl openlark_core::api::ApiResponseTrait for BankCardRecognizeResponse {}

/// 银行卡识别结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankCardRecognizeResult {
    /// 银行卡识别结果。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_card: Option<BankCard>,
}

/// 银行卡信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankCard {
    /// 识别出的实体。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<BankCardEntity>>,
}

/// 银行卡识别实体。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankCardEntity {
    /// 实体类型，例如 card_number、date_of_expiry。
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    /// 实体值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 银行卡识别请求
#[derive(Debug, Clone)]
pub struct BankCardRecognizeRequest {
    config: Config,
}

impl BankCardRecognizeRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(
        self,
        body: BankCardRecognizeBody,
    ) -> SDKResult<BankCardRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: BankCardRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<BankCardRecognizeResponse> {
        body.validate()?;

        let req: ApiRequest<BankCardRecognizeResponse> =
            ApiRequest::post(DOCUMENT_AI_BANK_CARD_RECOGNIZE)
                .body(serialize_params(&body, "银行卡识别")?)
                .file_content(body.file.clone());

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "银行卡识别")
    }
}

/// 银行卡识别请求构建器
#[derive(Debug, Clone)]
pub struct BankCardRecognizeRequestBuilder {
    request: BankCardRecognizeRequest,
    file: Option<Vec<u8>>,
    file_name: Option<String>,
}

impl BankCardRecognizeRequestBuilder {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            request: BankCardRecognizeRequest::new(config),
            file: None,
            file_name: None,
        }
    }

    /// 设置识别的银行卡源文件。
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
    pub fn body(self) -> BankCardRecognizeBody {
        BankCardRecognizeBody {
            file: self.file.unwrap_or_default(),
            file_name: self.file_name,
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<BankCardRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BankCardRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// bank_card_recognize。
pub async fn bank_card_recognize(
    config: &Config,
    body: BankCardRecognizeBody,
) -> SDKResult<BankCardRecognizeResponse> {
    bank_card_recognize_with_options(config, body, RequestOption::default()).await
}

/// bank_card_recognize_with_options。
pub async fn bank_card_recognize_with_options(
    config: &Config,
    body: BankCardRecognizeBody,
    option: RequestOption,
) -> SDKResult<BankCardRecognizeResponse> {
    body.validate()?;

    let req: ApiRequest<BankCardRecognizeResponse> =
        ApiRequest::post(DOCUMENT_AI_BANK_CARD_RECOGNIZE)
            .body(serialize_params(&body, "银行卡识别")?)
            .file_content(body.file.clone());

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "银行卡识别")
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
        let builder = BankCardRecognizeRequestBuilder::new(config.clone());
        assert!(builder.file.is_none());
    }

    #[test]
    fn test_body_validation_empty() {
        let body = BankCardRecognizeBody {
            file: Vec::new(),
            file_name: None,
        };
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = BankCardRecognizeBody {
            file: b"bank card image".to_vec(),
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
        let builder = BankCardRecognizeRequestBuilder::new(config.clone()).file_name("card.jpg");
        assert_eq!(builder.file_name, Some("card.jpg".to_string()));
    }

    #[test]
    fn test_builder_body_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let body = BankCardRecognizeRequestBuilder::new(config.clone())
            .file(b"card image".to_vec())
            .file_name("card.jpg")
            .body();
        assert_eq!(body.file, b"card image".to_vec());
        assert_eq!(body.file_name, Some("card.jpg".to_string()));
    }

    #[test]
    fn test_body_serialization() {
        let body = BankCardRecognizeBody {
            file: b"card image".to_vec(),
            file_name: Some("card.jpg".to_string()),
        };
        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(!json_str.contains("\"file\""));
        assert!(json_str.contains("__file_name"));
        assert!(json_str.contains("card.jpg"));
    }

    #[test]
    fn test_bank_card_entity_struct() {
        let entity = BankCardEntity {
            entity_type: Some("card_number".to_string()),
            value: Some("6222021234567890123".to_string()),
        };
        assert_eq!(entity.entity_type, Some("card_number".to_string()));
        assert_eq!(entity.value, Some("6222021234567890123".to_string()));
    }

    #[test]
    fn test_bank_card_entity_serialization() {
        let entity = BankCardEntity {
            entity_type: Some("date_of_expiry".to_string()),
            value: Some("10/28".to_string()),
        };
        let json_str = serde_json::to_string(&entity).expect("序列化失败");
        assert!(json_str.contains("\"type\""));
        assert!(json_str.contains("date_of_expiry"));
    }

    #[test]
    fn test_response_struct() {
        let response = BankCardRecognizeResponse { data: None };
        assert!(response.data.is_none());

        let result = BankCardRecognizeResult {
            bank_card: Some(BankCard {
                entities: Some(vec![BankCardEntity {
                    entity_type: Some("card_number".to_string()),
                    value: Some("6222021234567890123".to_string()),
                }]),
            }),
        };
        let response_with_data = BankCardRecognizeResponse { data: Some(result) };
        assert!(response_with_data.data.is_some());
        let card_number = response_with_data
            .data
            .as_ref()
            .and_then(|data| data.bank_card.as_ref())
            .and_then(|bank_card| bank_card.entities.as_ref())
            .and_then(|entities| entities.first())
            .and_then(|entity| entity.value.as_deref());
        assert_eq!(card_number, Some("6222021234567890123"));
    }
}
