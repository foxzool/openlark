/// recognize 模块。
pub mod recognize;

use openlark_core::config::Config;
use std::sync::Arc;

/// 健康证识别资源服务（对齐 URL /document_ai/v1/health_certificate）。
#[derive(Debug, Clone)]
pub struct HealthCertificateService {
    config: Arc<Config>,
}

impl HealthCertificateService {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 健康证识别（对齐 URL /document_ai/v1/health_certificate/recognize）。
    pub fn recognize(&self) -> recognize::HealthCertificateRecognizeRequestBuilder {
        recognize::HealthCertificateRecognizeRequestBuilder::new((*self.config).clone())
    }
}
