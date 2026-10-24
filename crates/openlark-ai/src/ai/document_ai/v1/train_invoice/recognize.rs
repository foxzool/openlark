//! 火车票识别
//!
//! 识别火车票信息。
//!
//! docPath: <https://open.feishu.cn/document/document_ai-v1/train_invoice_recognize>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::serialize_params;
use crate::endpoints::DOCUMENT_AI_TRAIN_INVOICE_RECOGNIZE;

/// 火车票识别请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainInvoiceRecognizeBody {
    /// 识别的火车票源文件。
    #[serde(skip_serializing)]
    pub file: Vec<u8>,
    /// multipart 文件名，仅用于设置文件 part 的 filename。
    #[serde(rename = "__file_name", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

impl TrainInvoiceRecognizeBody {
    /// 校验请求体。
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.file, "file 不能为空");
        Ok(())
    }
}

/// 火车票识别响应 `data`。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainInvoiceRecognizeResponse {
    /// 火车票信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub train_invoices: Option<Vec<TrainInvoice>>,
}

impl openlark_core::api::ApiResponseTrait for TrainInvoiceRecognizeResponse {}

/// 火车票信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainInvoice {
    /// 识别出的实体列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<TrainInvoiceEntity>>,
}

/// 火车票识别实体。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainInvoiceEntity {
    /// 实体类型，可选值有 start_station、end_station、train_num、name、seat_num、ticket_num、total_amount、time、price、seat_cls、id_num、sale_num、sale_station。
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    /// 实体值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 火车票识别请求
#[derive(Debug, Clone)]
pub struct TrainInvoiceRecognizeRequest {
    config: Config,
}

impl TrainInvoiceRecognizeRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(
        self,
        body: TrainInvoiceRecognizeBody,
    ) -> SDKResult<TrainInvoiceRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: TrainInvoiceRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<TrainInvoiceRecognizeResponse> {
        body.validate()?;

        let req: ApiRequest<TrainInvoiceRecognizeResponse> =
            ApiRequest::post(DOCUMENT_AI_TRAIN_INVOICE_RECOGNIZE)
                .body(serialize_params(&body, "火车票识别")?)
                .file_content(body.file.clone());

        Transport::request_typed(req, &self.config, Some(option), "火车票识别").await
    }
}

/// 火车票识别请求构建器
#[derive(Debug, Clone)]
pub struct TrainInvoiceRecognizeRequestBuilder {
    request: TrainInvoiceRecognizeRequest,
    file: Option<Vec<u8>>,
    file_name: Option<String>,
}

impl TrainInvoiceRecognizeRequestBuilder {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            request: TrainInvoiceRecognizeRequest::new(config),
            file: None,
            file_name: None,
        }
    }

    /// 设置识别的火车票源文件。
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
    pub fn body(self) -> TrainInvoiceRecognizeBody {
        TrainInvoiceRecognizeBody {
            file: self.file.unwrap_or_default(),
            file_name: self.file_name,
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<TrainInvoiceRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<TrainInvoiceRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行火车票识别。
pub async fn train_invoice_recognize(
    config: &Config,
    body: TrainInvoiceRecognizeBody,
) -> SDKResult<TrainInvoiceRecognizeResponse> {
    train_invoice_recognize_with_options(config, body, RequestOption::default()).await
}

/// train_invoice_recognize_with_options。
pub async fn train_invoice_recognize_with_options(
    config: &Config,
    body: TrainInvoiceRecognizeBody,
    option: RequestOption,
) -> SDKResult<TrainInvoiceRecognizeResponse> {
    body.validate()?;

    let req: ApiRequest<TrainInvoiceRecognizeResponse> =
        ApiRequest::post(DOCUMENT_AI_TRAIN_INVOICE_RECOGNIZE)
            .body(serialize_params(&body, "火车票识别")?)
            .file_content(body.file.clone());

    Transport::request_typed(req, config, Some(option), "火车票识别").await
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
        let builder = TrainInvoiceRecognizeRequestBuilder::new(config.clone());
        assert!(builder.file.is_none());
    }

    #[test]
    fn test_body_validation_empty() {
        let body = TrainInvoiceRecognizeBody {
            file: Vec::new(),
            file_name: None,
        };
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = TrainInvoiceRecognizeBody {
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
            TrainInvoiceRecognizeRequestBuilder::new(config.clone()).file_name("file.jpg");
        assert_eq!(builder.file_name, Some("file.jpg".to_string()));
    }

    #[test]
    fn test_builder_body_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let body = TrainInvoiceRecognizeRequestBuilder::new(config.clone())
            .file(b"file content".to_vec())
            .file_name("file.jpg")
            .body();
        assert_eq!(body.file, b"file content".to_vec());
        assert_eq!(body.file_name, Some("file.jpg".to_string()));
    }

    #[test]
    fn test_body_serialization() {
        let body = TrainInvoiceRecognizeBody {
            file: b"file content".to_vec(),
            file_name: Some("file.jpg".to_string()),
        };
        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(!json_str.contains("\"file\""));
        assert!(json_str.contains("__file_name"));
        assert!(json_str.contains("file.jpg"));
    }

    #[test]
    fn test_train_invoice_entity_struct() {
        let entity = TrainInvoiceEntity {
            entity_type: Some("train_num".to_string()),
            value: Some("G101".to_string()),
        };
        assert_eq!(entity.entity_type, Some("train_num".to_string()));
        assert_eq!(entity.value, Some("G101".to_string()));
    }

    #[test]
    fn test_train_invoice_entity_serialization() {
        let entity = TrainInvoiceEntity {
            entity_type: Some("train_num".to_string()),
            value: Some("G101".to_string()),
        };
        let json_str = serde_json::to_string(&entity).expect("序列化失败");
        assert!(json_str.contains("\"type\""));
        assert!(json_str.contains("train_num"));
    }

    #[test]
    fn test_response_struct() {
        let response = TrainInvoiceRecognizeResponse {
            train_invoices: None,
        };
        assert!(response.train_invoices.is_none());

        let response_with_data = TrainInvoiceRecognizeResponse {
            train_invoices: Some(vec![TrainInvoice {
                entities: Some(vec![TrainInvoiceEntity {
                    entity_type: Some("train_num".to_string()),
                    value: Some("G101".to_string()),
                }]),
            }]),
        };
        let value = response_with_data
            .train_invoices
            .as_ref()
            .and_then(|items| items.first())
            .and_then(|train_invoice| train_invoice.entities.as_ref())
            .and_then(|entities| entities.first())
            .and_then(|entity| entity.value.as_deref());
        assert_eq!(value, Some("G101"));
    }
}
