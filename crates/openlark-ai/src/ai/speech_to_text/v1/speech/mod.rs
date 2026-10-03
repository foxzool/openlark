//! Speech recognition module

pub mod file_recognize;
/// stream_recognize 模块。
pub mod stream_recognize;

use openlark_core::config::Config;
use std::sync::Arc;

/// Speech recognition API
#[derive(Clone)]
pub struct Speech {
    config: Arc<Config>,
}

impl Speech {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 语音文件识别（对齐 URL /speech_to_text/v1/speech/file_recognize）。
    pub fn file_recognize(&self) -> file_recognize::FileRecognizeRequestBuilder {
        file_recognize::FileRecognizeRequestBuilder::new((*self.config).clone())
    }

    /// 流式语音识别（对齐 URL /speech_to_text/v1/speech/stream_recognize）。
    pub fn stream_recognize(&self) -> stream_recognize::StreamRecognizeRequestBuilder {
        stream_recognize::StreamRecognizeRequestBuilder::new((*self.config).clone())
    }
}
