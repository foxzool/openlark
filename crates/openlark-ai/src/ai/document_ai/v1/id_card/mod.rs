/// recognize 模块。
pub mod recognize;

use openlark_core::config::Config;
use std::sync::Arc;

/// 身份证识别资源服务（对齐 URL /document_ai/v1/id_card）。
#[derive(Debug, Clone)]
pub struct IdCardService {
    config: Arc<Config>,
}

impl IdCardService {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 身份证识别（对齐 URL /document_ai/v1/id_card/recognize）。
    pub fn recognize(&self) -> recognize::IdCardRecognizeRequestBuilder {
        recognize::IdCardRecognizeRequestBuilder::new((*self.config).clone())
    }
}
