//! 文本翻译
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/translation-v1/text/translate>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::serialize_params;
use crate::endpoints::TRANSLATION_V1_TEXT_TRANSLATE;

/// 请求级术语（`term`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TranslateTerm {
    /// 原文术语。
    pub from: String,
    /// 译文术语。
    pub to: String,
}

/// 文本翻译请求体。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TextTranslateBody {
    /// 源语言（如 `zh` / `en`）。
    pub source_language: String,
    /// 待翻译文本（上限 1000 字符）。
    pub text: String,
    /// 目标语言（如 `zh` / `en`）。
    pub target_language: String,
    /// 请求级术语表（最多 128 个）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glossary: Option<Vec<TranslateTerm>>,
}

impl TextTranslateBody {
    /// 验证请求参数。
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.source_language, "source_language 不能为空");
        validate_required!(self.target_language, "target_language 不能为空");
        validate_required!(self.text, "text 不能为空");
        Ok(())
    }
}

/// 文本翻译响应 `data`。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct TextTranslateResponse {
    /// 译文。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl openlark_core::api::ApiResponseTrait for TextTranslateResponse {
    fn empty_success() -> Option<Self> {
        Some(Self::default())
    }
}

/// 文本翻译请求。
#[derive(Debug, Clone)]
pub struct TextTranslateRequest {
    config: Config,
}

impl TextTranslateRequest {
    /// 创建新的文本翻译请求。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行文本翻译请求。
    pub async fn execute(self, body: TextTranslateBody) -> SDKResult<TextTranslateResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行文本翻译请求（支持自定义选项）。
    pub async fn execute_with_options(
        self,
        body: TextTranslateBody,
        option: RequestOption,
    ) -> SDKResult<TextTranslateResponse> {
        body.validate()?;

        let req: ApiRequest<TextTranslateResponse> =
            ApiRequest::post(TRANSLATION_V1_TEXT_TRANSLATE)
                .body(serialize_params(&body, "文本翻译")?);

        Transport::request_typed(req, &self.config, Some(option), "文本翻译").await
    }
}

/// 文本翻译请求构建器。
#[derive(Debug, Clone)]
pub struct TextTranslateRequestBuilder {
    request: TextTranslateRequest,
    text: Option<String>,
    source_language: Option<String>,
    target_language: Option<String>,
    glossary: Option<Vec<TranslateTerm>>,
}

impl TextTranslateRequestBuilder {
    /// 创建新的构建器。
    pub fn new(config: Config) -> Self {
        Self {
            request: TextTranslateRequest::new(config),
            text: None,
            source_language: None,
            target_language: None,
            glossary: None,
        }
    }

    /// 设置待翻译文本。
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// 设置源语言。
    pub fn source_language(mut self, source_language: impl Into<String>) -> Self {
        self.source_language = Some(source_language.into());
        self
    }

    /// 设置目标语言。
    pub fn target_language(mut self, target_language: impl Into<String>) -> Self {
        self.target_language = Some(target_language.into());
        self
    }

    /// 设置术语表。
    pub fn glossary(mut self, glossary: Vec<TranslateTerm>) -> Self {
        self.glossary = Some(glossary);
        self
    }

    /// 构建请求体。
    pub fn body(self) -> TextTranslateBody {
        TextTranslateBody {
            text: self.text.unwrap_or_default(),
            source_language: self.source_language.unwrap_or_default(),
            target_language: self.target_language.unwrap_or_default(),
            glossary: self.glossary,
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<TextTranslateResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 执行请求（支持自定义选项）。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<TextTranslateResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行文本翻译。
pub async fn text_translate(
    config: &Config,
    body: TextTranslateBody,
) -> SDKResult<TextTranslateResponse> {
    text_translate_with_options(config, body, RequestOption::default()).await
}

/// 执行文本翻译（支持自定义选项）。
pub async fn text_translate_with_options(
    config: &Config,
    body: TextTranslateBody,
    option: RequestOption,
) -> SDKResult<TextTranslateResponse> {
    body.validate()?;

    let req: ApiRequest<TextTranslateResponse> =
        ApiRequest::post(TRANSLATION_V1_TEXT_TRANSLATE).body(serialize_params(&body, "文本翻译")?);

    Transport::request_typed(req, config, Some(option), "文本翻译").await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_body_serialization_official_fields() {
        let body = TextTranslateBody {
            source_language: "zh".into(),
            text: "尝试使用一下飞书吧".into(),
            target_language: "en".into(),
            glossary: Some(vec![TranslateTerm {
                from: "飞书".into(),
                to: "Lark".into(),
            }]),
        };
        let v = serde_json::to_value(&body).unwrap();
        assert_eq!(v["text"], "尝试使用一下飞书吧");
        assert_eq!(v["glossary"][0]["from"], "飞书");
        assert!(v.get("texts").is_none());
    }

    #[test]
    fn test_body_validation() {
        let body = TextTranslateBody {
            source_language: "zh".into(),
            text: "hi".into(),
            target_language: "en".into(),
            glossary: None,
        };
        assert!(body.validate().is_ok());
        let bad = TextTranslateBody {
            source_language: "".into(),
            text: "hi".into(),
            target_language: "en".into(),
            glossary: None,
        };
        assert!(bad.validate().is_err());
    }

    #[test]
    fn test_builder() {
        let config = Config::builder()
            .app_id("a")
            .app_secret("s")
            .build();
        let body = TextTranslateRequestBuilder::new(config)
            .text("Hello")
            .source_language("en")
            .target_language("zh")
            .body();
        assert_eq!(body.text, "Hello");
    }
}
