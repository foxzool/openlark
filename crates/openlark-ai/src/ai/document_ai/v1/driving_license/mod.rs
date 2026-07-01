/// recognize 模块。
pub mod recognize;

use openlark_core::config::Config;
use std::sync::Arc;

/// 驾驶证识别资源服务（对齐 URL /document_ai/v1/driving_license）。
#[derive(Debug, Clone)]
pub struct DrivingLicenseService {
    config: Arc<Config>,
}

impl DrivingLicenseService {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 驾驶证识别（对齐 URL /document_ai/v1/driving_license/recognize）。
    pub fn recognize(&self) -> recognize::DrivingLicenseRecognizeRequestBuilder {
        recognize::DrivingLicenseRecognizeRequestBuilder::new((*self.config).clone())
    }
}
