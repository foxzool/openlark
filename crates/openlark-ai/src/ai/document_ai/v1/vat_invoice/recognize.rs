//! 增值税发票识别
//!
//! 识别增值税发票中的关键信息。
//!
//! docPath: https://open.feishu.cn/document/document_ai-v1/vat_invoice_recognize

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::DOCUMENT_AI_VAT_INVOICE_RECOGNIZE;

/// 增值税发票识别请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VatInvoiceRecognizeBody {
    /// file_token 字段。
    pub file_token: String,
    /// is_async 字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_async: Option<bool>,
}

impl VatInvoiceRecognizeBody {
    /// 校验请求体。
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.file_token, "file_token 不能为空");
        Ok(())
    }
}

/// Vat_Invoice_Recognize_Response 响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VatInvoiceRecognizeResponse {
    /// data 字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<VatInvoiceRecognizeResult>,
}

impl openlark_core::api::ApiResponseTrait for VatInvoiceRecognizeResponse {}

/// Vat_Invoice_Recognize_Result 结果。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VatInvoiceRecognizeResult {
    /// parsing_result 字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsing_result: Option<ParsingResult>,
}

/// Parsing_Result 结果。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsingResult {
    /// 发票代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_code: Option<String>,
    /// 发票号码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_number: Option<String>,
    /// 开票日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_date: Option<String>,
    /// 校验码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_code: Option<String>,
    /// 购买方名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_name: Option<String>,
    /// 购买方纳税人识别号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_id: Option<String>,
    /// 销售方名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_name: Option<String>,
    /// 销售方纳税人识别号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_id: Option<String>,
    /// 金额
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// 税额
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amount: Option<String>,
    /// 价税合计
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<String>,
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
                .body(serialize_params(&body, "增值税发票识别")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "增值税发票识别")
    }
}

/// Vat_Invoice_Recognize_Request_Builder 请求构建器。
#[derive(Debug, Clone)]
pub struct VatInvoiceRecognizeRequestBuilder {
    request: VatInvoiceRecognizeRequest,
    file_token: Option<String>,
    is_async: Option<bool>,
}

impl VatInvoiceRecognizeRequestBuilder {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            request: VatInvoiceRecognizeRequest::new(config),
            file_token: None,
            is_async: None,
        }
    }

    /// file_token。
    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.file_token = Some(file_token.into());
        self
    }

    /// 设置 is_async。
    pub fn is_async(mut self, is_async: impl Into<bool>) -> Self {
        self.is_async = Some(is_async.into());
        self
    }

    /// 构建请求体。
    pub fn body(self) -> VatInvoiceRecognizeBody {
        VatInvoiceRecognizeBody {
            file_token: self.file_token.unwrap_or_default(),
            is_async: self.is_async,
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
            .body(serialize_params(&body, "增值税发票识别")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "增值税发票识别")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_empty() {
        let body = VatInvoiceRecognizeBody {
            file_token: "".to_string(),
            is_async: None,
        };
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = VatInvoiceRecognizeBody {
            file_token: "valid_token".to_string(),
            is_async: None,
        };
        assert!(body.validate().is_ok());
    }
}
