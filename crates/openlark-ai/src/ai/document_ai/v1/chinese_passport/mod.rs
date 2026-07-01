/// recognize 模块。
pub mod recognize;

use openlark_core::config::Config;
use std::sync::Arc;

/// 中国护照识别资源服务（对齐 URL /document_ai/v1/chinese_passport）。
#[derive(Debug, Clone)]
pub struct ChinesePassportService {
    config: Arc<Config>,
}

impl ChinesePassportService {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 中国护照识别（对齐 URL /document_ai/v1/chinese_passport/recognize）。
    pub fn recognize(&self) -> recognize::ChinesePassportRecognizeRequestBuilder {
        recognize::ChinesePassportRecognizeRequestBuilder::new((*self.config).clone())
    }
}
