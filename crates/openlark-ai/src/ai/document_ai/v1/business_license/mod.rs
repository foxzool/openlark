/// recognize 模块。
pub mod recognize;

use openlark_core::config::Config;
use std::sync::Arc;

/// 营业执照识别资源服务（对齐 URL /document_ai/v1/business_license）。
#[derive(Debug, Clone)]
pub struct BusinessLicenseService {
    config: Arc<Config>,
}

impl BusinessLicenseService {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 营业执照识别（对齐 URL /document_ai/v1/business_license/recognize）。
    pub fn recognize(&self) -> recognize::BusinessLicenseRecognizeRequestBuilder {
        recognize::BusinessLicenseRecognizeRequestBuilder::new((*self.config).clone())
    }
}
