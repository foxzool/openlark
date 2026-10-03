/// recognize 模块。
pub mod recognize;

use openlark_core::config::Config;
use std::sync::Arc;

/// 食品经营许可证识别资源服务（对齐 URL /document_ai/v1/food_manage_license）。
#[derive(Debug, Clone)]
pub struct FoodManageLicenseService {
    config: Arc<Config>,
}

impl FoodManageLicenseService {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 食品经营许可证识别（对齐 URL /document_ai/v1/food_manage_license/recognize）。
    pub fn recognize(&self) -> recognize::FoodManageLicenseRecognizeRequestBuilder {
        recognize::FoodManageLicenseRecognizeRequestBuilder::new((*self.config).clone())
    }
}
