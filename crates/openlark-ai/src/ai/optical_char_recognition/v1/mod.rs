//! OCR V1 模块

pub mod image;

use openlark_core::config::Config;
use std::sync::Arc;

/// OCR V1 API
#[derive(Clone)]
pub struct OcrV1 {
    config: Arc<Config>,
}

impl OcrV1 {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// image。
    pub fn image(&self) -> image::Image {
        image::Image::new(self.config.clone())
    }
}
