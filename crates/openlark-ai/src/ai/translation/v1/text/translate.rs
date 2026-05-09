//! 文本翻译
//!
//! 提供文本翻译服务，支持多种语言之间的翻译。
//!
//! docPath: https://open.feishu.cn/document/translation-v1/text_translate

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::TRANSLATION_V1_TEXT_TRANSLATE;

/// 文本翻译请求体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TextTranslateBody {
    /// 待翻译的文本列表
    pub texts: Vec<String>,
    /// 源语言语言代码（如：zh-CN, en-US, ja-JP）
    pub source_language: String,
    /// 目标语言语言代码（如：zh-CN, en-US, ja-JP）
    pub target_language: String,
}

impl TextTranslateBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.source_language, "source_language 不能为空");
        validate_required!(self.target_language, "target_language 不能为空");
        validate_required!(self.texts, "texts 不能为空");
        for text in &self.texts {
            validate_required!(text, "texts 中的文本不能为空");
        }
        Ok(())
    }
}

/// 文本翻译响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TextTranslateResponse {
    /// 翻译结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<TextTranslateResult>,
}

impl openlark_core::api::ApiResponseTrait for TextTranslateResponse {}

/// 文本翻译结果
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TextTranslateResult {
    /// 翻译后的文本列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations: Option<Vec<TranslationItem>>,
}

/// 翻译项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TranslationItem {
    /// 原文
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_text: Option<String>,
    /// 译文
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_text: Option<String>,
}

/// 文本翻译请求
#[derive(Debug, Clone)]
pub struct TextTranslateRequest {
    config: Config,
}

impl TextTranslateRequest {
    /// 创建新的文本翻译请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行文本翻译请求
    pub async fn execute(self, body: TextTranslateBody) -> SDKResult<TextTranslateResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行文本翻译请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: TextTranslateBody,
        option: RequestOption,
    ) -> SDKResult<TextTranslateResponse> {
        body.validate()?;

        let req: ApiRequest<TextTranslateResponse> =
            ApiRequest::post(TRANSLATION_V1_TEXT_TRANSLATE)
                .body(serialize_params(&body, "文本翻译")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "文本翻译")
    }
}

/// 文本翻译请求构建器
#[derive(Debug, Clone)]
pub struct TextTranslateRequestBuilder {
    request: TextTranslateRequest,
    texts: Vec<String>,
    source_language: Option<String>,
    target_language: Option<String>,
}

impl TextTranslateRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Config) -> Self {
        Self {
            request: TextTranslateRequest::new(config),
            texts: Vec::new(),
            source_language: None,
            target_language: None,
        }
    }

    /// 添加待翻译文本
    pub fn add_text(mut self, text: impl Into<String>) -> Self {
        self.texts.push(text.into());
        self
    }

    /// 设置待翻译文本列表
    pub fn texts(mut self, texts: impl Into<Vec<String>>) -> Self {
        self.texts = texts.into();
        self
    }

    /// 设置源语言
    pub fn source_language(mut self, source_language: impl Into<String>) -> Self {
        self.source_language = Some(source_language.into());
        self
    }

    /// 设置目标语言
    pub fn target_language(mut self, target_language: impl Into<String>) -> Self {
        self.target_language = Some(target_language.into());
        self
    }

    /// 构建请求体
    pub fn body(self) -> TextTranslateBody {
        TextTranslateBody {
            texts: self.texts,
            source_language: self.source_language.unwrap_or_default(),
            target_language: self.target_language.unwrap_or_default(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TextTranslateResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<TextTranslateResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行文本翻译
///
/// docPath: https://open.feishu.cn/document/translation-v1/text_translate
pub async fn text_translate(
    config: &Config,
    body: TextTranslateBody,
) -> SDKResult<TextTranslateResponse> {
    text_translate_with_options(config, body, RequestOption::default()).await
}

/// 执行文本翻译（支持自定义选项）
pub async fn text_translate_with_options(
    config: &Config,
    body: TextTranslateBody,
    option: RequestOption,
) -> SDKResult<TextTranslateResponse> {
    body.validate()?;

    let req: ApiRequest<TextTranslateResponse> =
        ApiRequest::post(TRANSLATION_V1_TEXT_TRANSLATE).body(serialize_params(&body, "文本翻译")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "文本翻译")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_default_state() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = TextTranslateRequestBuilder::new(config.clone());

        assert!(builder.texts.is_empty());
        assert!(builder.source_language.is_none());
        assert!(builder.target_language.is_none());
    }

    #[test]
    fn test_builder_add_text() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = TextTranslateRequestBuilder::new(config.clone())
            .add_text("Hello")
            .add_text("World");

        assert_eq!(builder.texts.len(), 2);
        assert_eq!(builder.texts[0], "Hello");
        assert_eq!(builder.texts[1], "World");
    }

    #[test]
    fn test_builder_source_language() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = TextTranslateRequestBuilder::new(config.clone()).source_language("en-US");

        assert_eq!(builder.source_language, Some("en-US".to_string()));
    }

    #[test]
    fn test_builder_target_language() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = TextTranslateRequestBuilder::new(config.clone()).target_language("zh-CN");

        assert_eq!(builder.target_language, Some("zh-CN".to_string()));
    }

    #[test]
    fn test_body_validation_empty_source_language() {
        let body = TextTranslateBody {
            texts: vec!["Hello".to_string()],
            source_language: "".to_string(),
            target_language: "zh-CN".to_string(),
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_validation_empty_target_language() {
        let body = TextTranslateBody {
            texts: vec!["Hello".to_string()],
            source_language: "en-US".to_string(),
            target_language: "".to_string(),
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_validation_empty_texts() {
        let body = TextTranslateBody {
            texts: vec![],
            source_language: "en-US".to_string(),
            target_language: "zh-CN".to_string(),
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = TextTranslateBody {
            texts: vec!["Hello".to_string(), "World".to_string()],
            source_language: "en-US".to_string(),
            target_language: "zh-CN".to_string(),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_from_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let body = TextTranslateRequestBuilder::new(config.clone())
            .add_text("Hello")
            .add_text("World")
            .source_language("en-US")
            .target_language("zh-CN")
            .body();

        assert_eq!(body.texts.len(), 2);
        assert_eq!(body.source_language, "en-US");
        assert_eq!(body.target_language, "zh-CN");
    }

    fn test_roundtrip<T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug>(
        original: &T,
    ) {
        let json = serde_json::to_string(original).expect("序列化失败");
        let deserialized: T = serde_json::from_str(&json).expect("反序列化失败");
        assert_eq!(original, &deserialized, "roundtrip 后数据不一致");
    }

    #[test]
    fn test_text_translate_body_serialization() {
        let body = TextTranslateBody {
            texts: vec!["Hello".to_string(), "World".to_string()],
            source_language: "en-US".to_string(),
            target_language: "zh-CN".to_string(),
        };
        test_roundtrip(&body);
    }

    #[test]
    fn test_text_translate_response_serialization() {
        let response = TextTranslateResponse {
            data: Some(TextTranslateResult {
                translations: Some(vec![
                    TranslationItem {
                        source_text: Some("Hello".to_string()),
                        target_text: Some("你好".to_string()),
                    },
                    TranslationItem {
                        source_text: Some("World".to_string()),
                        target_text: Some("世界".to_string()),
                    },
                ]),
            }),
        };
        test_roundtrip(&response);
    }

    #[test]
    fn test_translation_item_serialization() {
        let item = TranslationItem {
            source_text: Some("Hello".to_string()),
            target_text: Some("你好".to_string()),
        };
        test_roundtrip(&item);
    }

    #[test]
    fn test_text_translate_result_serialization() {
        let result = TextTranslateResult {
            translations: Some(vec![TranslationItem {
                source_text: Some("Test".to_string()),
                target_text: Some("测试".to_string()),
            }]),
        };
        test_roundtrip(&result);
    }
}
