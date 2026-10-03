/// recognize 模块。
pub mod recognize;

use openlark_core::config::Config;
use std::sync::Arc;

/// 火车票识别资源服务（对齐 URL /document_ai/v1/train_invoice）。
#[derive(Debug, Clone)]
pub struct TrainInvoiceService {
    config: Arc<Config>,
}

impl TrainInvoiceService {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 火车票识别（对齐 URL /document_ai/v1/train_invoice/recognize）。
    pub fn recognize(&self) -> recognize::TrainInvoiceRecognizeRequestBuilder {
        recognize::TrainInvoiceRecognizeRequestBuilder::new((*self.config).clone())
    }
}
