/// recognize 模块。
pub mod recognize;

use openlark_core::config::Config;
use std::sync::Arc;

/// 食品生产许可证识别资源服务（对齐 URL /document_ai/v1/food_produce_license）。
#[derive(Debug, Clone)]
pub struct FoodProduceLicenseService {
    config: Arc<Config>,
}

impl FoodProduceLicenseService {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 食品生产许可证识别（对齐 URL /document_ai/v1/food_produce_license/recognize）。
    pub fn recognize(&self) -> recognize::FoodProduceLicenseRecognizeRequestBuilder {
        recognize::FoodProduceLicenseRecognizeRequestBuilder::new((*self.config).clone())
    }
}
