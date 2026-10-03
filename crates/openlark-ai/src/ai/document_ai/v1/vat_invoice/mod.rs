/// recognize 模块。
pub mod recognize;

use openlark_core::config::Config;
use std::sync::Arc;

/// 增值税发票识别资源服务（对齐 URL /document_ai/v1/vat_invoice）。
#[derive(Debug, Clone)]
pub struct VatInvoiceService {
    config: Arc<Config>,
}

impl VatInvoiceService {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 增值税发票识别（对齐 URL /document_ai/v1/vat_invoice/recognize）。
    pub fn recognize(&self) -> recognize::VatInvoiceRecognizeRequestBuilder {
        recognize::VatInvoiceRecognizeRequestBuilder::new((*self.config).clone())
    }
}
