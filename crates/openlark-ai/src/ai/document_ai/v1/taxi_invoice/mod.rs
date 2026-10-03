/// recognize 模块。
pub mod recognize;

use openlark_core::config::Config;
use std::sync::Arc;

/// 出租车发票识别资源服务（对齐 URL /document_ai/v1/taxi_invoice）。
#[derive(Debug, Clone)]
pub struct TaxiInvoiceService {
    config: Arc<Config>,
}

impl TaxiInvoiceService {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 出租车发票识别（对齐 URL /document_ai/v1/taxi_invoice/recognize）。
    pub fn recognize(&self) -> recognize::TaxiInvoiceRecognizeRequestBuilder {
        recognize::TaxiInvoiceRecognizeRequestBuilder::new((*self.config).clone())
    }
}
