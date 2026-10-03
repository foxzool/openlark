/// recognize 模块。
pub mod recognize;

use openlark_core::config::Config;
use std::sync::Arc;

/// 机动车发票识别资源服务（对齐 URL /document_ai/v1/vehicle_invoice）。
#[derive(Debug, Clone)]
pub struct VehicleInvoiceService {
    config: Arc<Config>,
}

impl VehicleInvoiceService {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 机动车发票识别（对齐 URL /document_ai/v1/vehicle_invoice/recognize）。
    pub fn recognize(&self) -> recognize::VehicleInvoiceRecognizeRequestBuilder {
        recognize::VehicleInvoiceRecognizeRequestBuilder::new((*self.config).clone())
    }
}
