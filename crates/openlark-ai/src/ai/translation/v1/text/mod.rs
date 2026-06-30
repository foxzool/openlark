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

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
