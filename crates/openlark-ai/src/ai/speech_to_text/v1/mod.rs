//! Speech-to-Text V1 模块

pub mod speech;

use openlark_core::config::Config;
use std::sync::Arc;

/// Speech-to-Text V1 API
#[derive(Clone)]
pub struct SpeechToTextV1 {
    config: Arc<Config>,
}

impl SpeechToTextV1 {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// speech。
    pub fn speech(&self) -> speech::Speech {
        speech::Speech::new(self.config.clone())
    }
}
