//! 增值税发票识别
//!
//! 识别增值税发票中的关键信息。
//!
//! docPath: <https://open.feishu.cn/document/document_ai-v1/vat_invoice_recognize>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::serialize_params;
use crate::endpoints::DOCUMENT_AI_VAT_INVOICE_RECOGNIZE;

/// 增值税发票识别请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VatInvoiceRecognizeBody {
    /// 识别的增值税发票源文件（支持 JPG/JPEG/PNG/PDF/BMP/OFD，文件大小需小于 10M）。
    #[serde(skip_serializing)]
    pub file: Vec<u8>,
    /// multipart 文件名，仅用于设置文件 part 的 filename。
    #[serde(rename = "__file_name", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

impl VatInvoiceRecognizeBody {
    /// 校验请求体。
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.file, "file 不能为空");
        Ok(())
    }
}

/// 增值税发票识别响应 `data`。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VatInvoiceRecognizeResponse {
    /// 增值税发票信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_invoices: Option<Vec<VatInvoice>>,
}

impl openlark_core::api::ApiResponseTrait for VatInvoiceRecognizeResponse {}

/// 增值税发票信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VatInvoice {
    /// 识别出的实体列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<VatInvoiceEntity>>,
}

/// 增值税发票识别实体。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VatInvoiceEntity {
    /// 实体类型，可选值有 invoice_name、invoice_code、invoice_no、invoice_date、total_price、total_tax、big_total_price_and_tax、check_code、total_price_and_tax、buyer_name、buyer_taxpayer_no、buyer_address_phone、buyer_account、seller_name、seller_taxpayer_no、seller_address_phone、seller_account、payee、password_area、remarks、reviewer、drawer、is_sealed、seller_name_in_seal、seller_taxpayer_no_in_seal、invoice_special_seal、machine_num。
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    /// 识别出字段的文本信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 识别出的票据详细信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Vec<VatInvoiceKvEntity>>>,
}

/// 增值税发票票据详细键值实体。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VatInvoiceKvEntity {
    /// 键类型，例如 entry_name。
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    /// 键值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Vat_Invoice_Recognize_Request 请求。
#[derive(Debug, Clone)]
pub struct VatInvoiceRecognizeRequest {
    config: Config,
}

impl VatInvoiceRecognizeRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(
        self,
        body: VatInvoiceRecognizeBody,
    ) -> SDKResult<VatInvoiceRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: VatInvoiceRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<VatInvoiceRecognizeResponse> {
        body.validate()?;

        let req: ApiRequest<VatInvoiceRecognizeResponse> =
            ApiRequest::post(DOCUMENT_AI_VAT_INVOICE_RECOGNIZE)
                .body(serialize_params(&body, "增值税发票识别")?)
                .file_content(body.file.clone());

        Transport::request_typed(req, &self.config, Some(option), "增值税发票识别").await
    }
}

/// Vat_Invoice_Recognize_Request_Builder 请求构建器。
#[derive(Debug, Clone)]
pub struct VatInvoiceRecognizeRequestBuilder {
    request: VatInvoiceRecognizeRequest,
    file: Option<Vec<u8>>,
    file_name: Option<String>,
}

impl VatInvoiceRecognizeRequestBuilder {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            request: VatInvoiceRecognizeRequest::new(config),
            file: None,
            file_name: None,
        }
    }

    /// 设置识别的增值税发票源文件。
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
    pub fn body(self) -> VatInvoiceRecognizeBody {
        VatInvoiceRecognizeBody {
            file: self.file.unwrap_or_default(),
            file_name: self.file_name,
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<VatInvoiceRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<VatInvoiceRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// vat_invoice_recognize。
pub async fn vat_invoice_recognize(
    config: &Config,
    body: VatInvoiceRecognizeBody,
) -> SDKResult<VatInvoiceRecognizeResponse> {
    vat_invoice_recognize_with_options(config, body, RequestOption::default()).await
}

/// vat_invoice_recognize_with_options。
pub async fn vat_invoice_recognize_with_options(
    config: &Config,
    body: VatInvoiceRecognizeBody,
    option: RequestOption,
) -> SDKResult<VatInvoiceRecognizeResponse> {
    body.validate()?;

    let req: ApiRequest<VatInvoiceRecognizeResponse> =
        ApiRequest::post(DOCUMENT_AI_VAT_INVOICE_RECOGNIZE)
            .body(serialize_params(&body, "增值税发票识别")?)
            .file_content(body.file.clone());

    Transport::request_typed(req, config, Some(option), "增值税发票识别").await
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
    }

    #[test]
    fn test_body_validation_empty() {
        let body = VatInvoiceRecognizeBody {
            file: Vec::new(),
            file_name: None,
        };
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = VatInvoiceRecognizeBody {
            file: b"vat invoice file".to_vec(),
            file_name: None,
        };
        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_builder_default_state() {
        let builder = VatInvoiceRecognizeRequestBuilder::new(test_config());
        assert!(builder.file.is_none());
        assert!(builder.file_name.is_none());
    }

    #[test]
    fn test_builder_body_creation() {
        let body = VatInvoiceRecognizeRequestBuilder::new(test_config())
            .file(b"invoice file".to_vec())
            .file_name("invoice.pdf")
            .body();
        assert_eq!(body.file, b"invoice file".to_vec());
        assert_eq!(body.file_name, Some("invoice.pdf".to_string()));
    }

    #[test]
    fn test_body_serialization() {
        let body = VatInvoiceRecognizeBody {
            file: b"invoice file".to_vec(),
            file_name: Some("invoice.pdf".to_string()),
        };
        let json_str = serde_json::to_string(&body).expect("序列化失败");
        // file 为二进制内容，走 multipart file part，不进 JSON
        assert!(!json_str.contains("\"file\""));
        assert!(json_str.contains("__file_name"));
        assert!(json_str.contains("invoice.pdf"));
    }

    #[test]
    fn test_response_deserialize_official_example() {
        // 官方文档响应体示例（data 内容）
        let json = serde_json::json!({
            "vat_invoices": [
                {
                    "entities": [
                        {
                            "type": "buyer_name",
                            "value": "发呆公司",
                            "items": [
                                [
                                    {"type": "entry_name", "value": "餐饮服务"}
                                ]
                            ]
                        }
                    ]
                }
            ]
        });
        let resp: VatInvoiceRecognizeResponse = serde_json::from_value(json).expect("反序列化失败");
        let entity = resp
            .vat_invoices
            .as_ref()
            .and_then(|invoices| invoices.first())
            .and_then(|invoice| invoice.entities.as_ref())
            .and_then(|entities| entities.first())
            .expect("实体不存在");
        assert_eq!(entity.entity_type.as_deref(), Some("buyer_name"));
        assert_eq!(entity.value.as_deref(), Some("发呆公司"));
        let kv = entity
            .items
            .as_ref()
            .and_then(|items| items.first())
            .and_then(|group| group.first())
            .expect("items 不存在");
        assert_eq!(kv.entity_type.as_deref(), Some("entry_name"));
        assert_eq!(kv.value.as_deref(), Some("餐饮服务"));
    }
}
