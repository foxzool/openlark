//! Document AI 链式调用入口（meta 风格）
//!
//! 说明：
//! - 本文件放在 `common/` 下，提供链式调用风格访问 Document AI API
//! - 真实 API 实现位于 `src/document_ai/document_ai/v1/recognize/`

use std::sync::Arc;

use openlark_core::config::Config;

/// Document AI 链式入口：`document_ai.v1.recognize.resume_parse(...)`
#[derive(Debug, Clone)]
pub struct DocumentAiClient {
    config: Arc<Config>,
    pub v1: DocumentAiV1Client,
}

impl DocumentAiClient {
    pub fn new(config: Config) -> Self {
        let config = Arc::new(config);
        Self {
            config: config.clone(),
            v1: DocumentAiV1Client::new(config),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// Document AI v1：`document_ai.v1`
#[derive(Debug, Clone)]
pub struct DocumentAiV1Client {
    config: Arc<Config>,
    pub recognize: RecognizeResource,
}

impl DocumentAiV1Client {
    fn new(config: Arc<Config>) -> Self {
        Self {
            config: config.clone(),
            recognize: RecognizeResource::new(config),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// Document AI v1 识别资源：`document_ai.v1.recognize`
#[derive(Debug, Clone)]
pub struct RecognizeResource {
    config: Arc<Config>,
}

impl RecognizeResource {
    fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 简历解析
    ///
    /// docPath: <https://open.feishu.cn/document/document_ai-v1/resume_parse>
    pub fn resume_parse(
        &self,
    ) -> crate::document_ai::document_ai::v1::recognize::ResumeParseRequestBuilder {
        crate::document_ai::document_ai::v1::recognize::ResumeParseRequestBuilder::new(
            (*self.config).clone(),
        )
    }

    /// 身份证识别
    ///
    /// docPath: <https://open.feishu.cn/document/document_ai-v1/id_card_recognize>
    pub fn id_card_recognize(
        &self,
    ) -> crate::document_ai::document_ai::v1::recognize::IdCardRecognizeRequestBuilder {
        crate::document_ai::document_ai::v1::recognize::IdCardRecognizeRequestBuilder::new(
            (*self.config).clone(),
        )
    }

    /// 银行卡识别
    ///
    /// docPath: <https://open.feishu.cn/document/document_ai-v1/bank_card_recognize>
    pub fn bank_card_recognize(
        &self,
    ) -> crate::document_ai::document_ai::v1::recognize::BankCardRecognizeRequestBuilder {
        crate::document_ai::document_ai::v1::recognize::BankCardRecognizeRequestBuilder::new(
            (*self.config).clone(),
        )
    }

    /// 营业执照识别
    ///
    /// docPath: <https://open.feishu.cn/document/document_ai-v1/business_license_recognize>
    pub fn business_license_recognize(
        &self,
    ) -> crate::document_ai::document_ai::v1::recognize::BusinessLicenseRecognizeRequestBuilder
    {
        crate::document_ai::document_ai::v1::recognize::BusinessLicenseRecognizeRequestBuilder::new(
            (*self.config).clone(),
        )
    }

    /// 增值税发票识别
    ///
    /// docPath: <https://open.feishu.cn/document/document_ai-v1/vat_invoice_recognize>
    pub fn vat_invoice_recognize(
        &self,
    ) -> crate::document_ai::document_ai::v1::recognize::VatInvoiceRecognizeRequestBuilder {
        crate::document_ai::document_ai::v1::recognize::VatInvoiceRecognizeRequestBuilder::new(
            (*self.config).clone(),
        )
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
