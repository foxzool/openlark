/// recognize 模块。
pub mod recognize;

use openlark_core::config::Config;
use std::sync::Arc;

/// 台湾居民来往大陆通行证识别资源服务（对齐 URL /document_ai/v1/tw_mainland_travel_permit）。
#[derive(Debug, Clone)]
pub struct TwMainlandTravelPermitService {
    config: Arc<Config>,
}

impl TwMainlandTravelPermitService {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 台湾居民来往大陆通行证识别（对齐 URL /document_ai/v1/tw_mainland_travel_permit/recognize）。
    pub fn recognize(&self) -> recognize::TwMainlandTravelPermitRecognizeRequestBuilder {
        recognize::TwMainlandTravelPermitRecognizeRequestBuilder::new((*self.config).clone())
    }
}
