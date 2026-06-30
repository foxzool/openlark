/// recognize 模块。
pub mod recognize;

use openlark_core::config::Config;
use std::sync::Arc;

/// 港澳居民来往内地通行证识别资源服务（对齐 URL /document_ai/v1/hkm_mainland_travel_permit）。
#[derive(Debug, Clone)]
pub struct HkmMainlandTravelPermitService {
    config: Arc<Config>,
}

impl HkmMainlandTravelPermitService {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 港澳居民来往内地通行证识别（对齐 URL /document_ai/v1/hkm_mainland_travel_permit/recognize）。
    pub fn recognize(&self) -> recognize::HkmMainlandTravelPermitRecognizeRequestBuilder {
        recognize::HkmMainlandTravelPermitRecognizeRequestBuilder::new((*self.config).clone())
    }
}
