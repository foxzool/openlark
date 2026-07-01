/// recognize 模块。
pub mod recognize;

use openlark_core::config::Config;
use std::sync::Arc;

/// 名片识别资源服务（对齐 URL /document_ai/v1/business_card）。
#[derive(Debug, Clone)]
pub struct BusinessCardService {
    config: Arc<Config>,
}

impl BusinessCardService {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 名片识别（对齐 URL /document_ai/v1/business_card/recognize）。
    pub fn recognize(&self) -> recognize::BusinessCardRecognizeRequestBuilder {
        recognize::BusinessCardRecognizeRequestBuilder::new((*self.config).clone())
    }
}
