//! 文本语言检测
//!
//! 提供文本语言检测服务，检测给定文本的语言类型。
//!
//! docPath: https://open.feishu.cn/document/translation-v1/text_detect

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::TRANSLATION_V1_TEXT_DETECT;

/// 文本语言检测请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDetectBody {
    /// 待检测的文本
    pub text: String,
}

impl TextDetectBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.text, "text 不能为空");
        Ok(())
    }
}

/// 文本语言检测响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDetectResponse {
    /// 检测结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<TextDetectResult>,
}

impl openlark_core::api::ApiResponseTrait for TextDetectResponse {}

/// 文本语言检测结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDetectResult {
    /// 检测到的语言
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// 语言概率
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
}

/// 文本语言检测请求
#[derive(Debug, Clone)]
pub struct TextDetectRequest {
    config: Config,
}

impl TextDetectRequest {
    /// 创建新的文本语言检测请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行文本语言检测请求
    pub async fn execute(self, body: TextDetectBody) -> SDKResult<TextDetectResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行文本语言检测请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: TextDetectBody,
        option: RequestOption,
    ) -> SDKResult<TextDetectResponse> {
        body.validate()?;

        let req: ApiRequest<TextDetectResponse> = ApiRequest::post(TRANSLATION_V1_TEXT_DETECT)
            .body(serialize_params(&body, "文本语言检测")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "文本语言检测")
    }
}

/// 文本语言检测请求构建器
#[derive(Debug, Clone)]
pub struct TextDetectRequestBuilder {
    request: TextDetectRequest,
    text: Option<String>,
}

impl TextDetectRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Config) -> Self {
        Self {
            request: TextDetectRequest::new(config),
            text: None,
        }
    }

    /// 设置待检测文本
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// 构建请求体
    pub fn body(self) -> TextDetectBody {
        TextDetectBody {
            text: self.text.unwrap_or_default(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TextDetectResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<TextDetectResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行文本语言检测
///
/// docPath: https://open.feishu.cn/document/translation-v1/text_detect
pub async fn text_detect(config: &Config, body: TextDetectBody) -> SDKResult<TextDetectResponse> {
    text_detect_with_options(config, body, RequestOption::default()).await
}

/// 执行文本语言检测（支持自定义选项）
pub async fn text_detect_with_options(
    config: &Config,
    body: TextDetectBody,
    option: RequestOption,
) -> SDKResult<TextDetectResponse> {
    body.validate()?;

    let req: ApiRequest<TextDetectResponse> =
        ApiRequest::post(TRANSLATION_V1_TEXT_DETECT).body(serialize_params(&body, "文本语言检测")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "文本语言检测")
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
        let builder = TextDetectRequestBuilder::new(config.clone());

        assert!(builder.text.is_none());
    }

    #[test]
    fn test_builder_text() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = TextDetectRequestBuilder::new(config.clone()).text("Hello World");

        assert_eq!(builder.text, Some("Hello World".to_string()));
    }

    #[test]
    fn test_body_validation_empty_text() {
        let body = TextDetectBody {
            text: "".to_string(),
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = TextDetectBody {
            text: "Hello World".to_string(),
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
        let body = TextDetectRequestBuilder::new(config.clone())
            .text("你好世界")
            .body();

        assert_eq!(body.text, "你好世界");
    }
}
