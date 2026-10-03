//! Translation V1 模块

pub mod text;

use openlark_core::config::Config;
use std::sync::Arc;

/// Translation V1 API
#[derive(Clone)]
pub struct TranslationV1 {
    config: Arc<Config>,
}

impl TranslationV1 {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// text。
    pub fn text(&self) -> text::Text {
        text::Text::new(self.config.clone())
    }
}
