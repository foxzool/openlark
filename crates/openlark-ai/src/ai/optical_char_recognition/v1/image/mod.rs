//! Image OCR module

pub mod basic_recognize;

use openlark_core::config::Config;
use std::sync::Arc;

/// Image OCR API
#[derive(Clone)]
pub struct Image {
    config: Arc<Config>,
}

impl Image {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// OCR 基础识别（对齐 URL /optical_char_recognition/v1/image/basic_recognize）。
    pub fn basic_recognize(&self) -> basic_recognize::BasicRecognizeRequestBuilder {
        basic_recognize::BasicRecognizeRequestBuilder::new((*self.config).clone())
    }
}
