//! AI 服务入口
//!
//! 提供 AI 服务的客户端入口，支持链式调用风格。

use openlark_core::config::Config;
use std::sync::Arc;

/// AI 服务客户端
///
/// 提供 AI 相关服务的链式访问入口。
///
/// # 示例
///
/// ```rust
/// use openlark_ai::service::AiClient;
/// use openlark_ai::prelude::Config;
///
/// let config = Config::builder()
///     .app_id("your_app_id")
///     .app_secret("your_app_secret")
///     .build();
///
/// let client = AiClient::new(config);
/// // client.document_ai().v1().recognize()...
/// ```
#[derive(Debug, Clone)]
pub struct AiClient {
    config: Arc<Config>,
}

impl AiClient {
    /// 创建新的 AI 客户端
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 文档 AI 服务
    pub fn document_ai(&self) -> DocumentAiClient {
        DocumentAiClient::new(self.config.clone())
    }

    /// OCR 服务
    pub fn ocr(&self) -> OcrClient {
        OcrClient::new(self.config.clone())
    }

    /// 语音转文字服务
    pub fn speech_to_text(&self) -> SpeechToTextClient {
        SpeechToTextClient::new(self.config.clone())
    }

    /// 翻译服务
    pub fn translation(&self) -> TranslationClient {
        TranslationClient::new(self.config.clone())
    }
}

/// 文档 AI 客户端
///
/// 提供文档 AI 识别服务的链式访问。
#[derive(Debug, Clone)]
pub struct DocumentAiClient {
    config: Arc<Config>,
}

impl DocumentAiClient {
    fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// V1 版本 API
    #[cfg(feature = "v1")]
    pub fn v1(&self) -> super::ai::document_ai::v1::DocumentAiV1 {
        super::ai::document_ai::v1::DocumentAiV1::new(self.config.clone())
    }
}

/// OCR 客户端
///
/// 提供 OCR 识别服务的链式访问。
#[derive(Debug, Clone)]
pub struct OcrClient {
    config: Arc<Config>,
}

impl OcrClient {
    fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// V1 版本 API
    #[cfg(feature = "v1")]
    pub fn v1(&self) -> super::ai::optical_char_recognition::v1::OcrV1 {
        super::ai::optical_char_recognition::v1::OcrV1::new(self.config.clone())
    }
}

/// 语音转文字客户端
///
/// 提供语音转文字服务的链式访问。
#[derive(Debug, Clone)]
pub struct SpeechToTextClient {
    config: Arc<Config>,
}

impl SpeechToTextClient {
    fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// V1 版本 API
    #[cfg(feature = "v1")]
    pub fn v1(&self) -> super::ai::speech_to_text::v1::SpeechToTextV1 {
        super::ai::speech_to_text::v1::SpeechToTextV1::new(self.config.clone())
    }
}

/// 翻译客户端
///
/// 提供翻译服务的链式访问。
#[derive(Debug, Clone)]
pub struct TranslationClient {
    config: Arc<Config>,
}

impl TranslationClient {
    fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// V1 版本 API
    #[cfg(feature = "v1")]
    pub fn v1(&self) -> super::ai::translation::v1::TranslationV1 {
        super::ai::translation::v1::TranslationV1::new(self.config.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_client_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let client = AiClient::new(config);
        assert_eq!(client.config().app_id(), "test_app_id");
    }

    #[test]
    fn test_document_ai_client_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let client = DocumentAiClient::new(Arc::new(config));
        assert_eq!(client.config().app_id(), "test_app_id");
    }

    #[test]
    fn test_ocr_client_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let client = OcrClient::new(Arc::new(config));
        assert_eq!(client.config().app_id(), "test_app_id");
    }

    #[test]
    fn test_speech_to_text_client_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let client = SpeechToTextClient::new(Arc::new(config));
        assert_eq!(client.config().app_id(), "test_app_id");
    }

    #[test]
    fn test_translation_client_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let client = TranslationClient::new(Arc::new(config));
        assert_eq!(client.config().app_id(), "test_app_id");
    }
}
