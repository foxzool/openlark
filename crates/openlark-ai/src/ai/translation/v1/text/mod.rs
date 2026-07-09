//! Text translation module

pub mod detect;
/// translate 模块。
pub mod translate;

use openlark_core::config::Config;
use std::sync::Arc;

/// Text translation API
#[derive(Clone)]
pub struct Text {
    config: Arc<Config>,
}

impl Text {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 文本翻译（对齐 URL /translation/v1/text/translate）。
    pub fn translate(&self) -> translate::TextTranslateRequestBuilder {
        translate::TextTranslateRequestBuilder::new((*self.config).clone())
    }

    /// 语种检测（对齐 URL /translation/v1/text/detect）。
    pub fn detect(&self) -> detect::TextDetectRequestBuilder {
        detect::TextDetectRequestBuilder::new((*self.config).clone())
    }
}
