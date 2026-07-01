/// recognize 模块。
pub mod recognize;

use openlark_core::config::Config;
use std::sync::Arc;

/// 行驶证识别资源服务（对齐 URL /document_ai/v1/vehicle_license）。
#[derive(Debug, Clone)]
pub struct VehicleLicenseService {
    config: Arc<Config>,
}

impl VehicleLicenseService {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 行驶证识别（对齐 URL /document_ai/v1/vehicle_license/recognize）。
    pub fn recognize(&self) -> recognize::VehicleLicenseRecognizeRequestBuilder {
        recognize::VehicleLicenseRecognizeRequestBuilder::new((*self.config).clone())
    }
}
