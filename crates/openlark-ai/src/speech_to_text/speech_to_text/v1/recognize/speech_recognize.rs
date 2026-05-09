//! 通用语音识别
//!
//! 通用语音识别服务，支持多种音频格式和识别配置。
//!
//! docPath: https://open.feishu.cn/document/server-docs/ai/speech_to_text-v1/speech/recognize

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::SPEECH_TO_TEXT_V1_SPEECH_RECOGNIZE;

/// 通用语音识别请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechRecognizeBody {
    /// 音频数据（Base64 编码或文件 token）
    pub audio: String,
    /// 音频来源类型：base64 或 file_token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_type: Option<String>,
    /// 语言类型，默认为中文
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// 音频格式：pcm, wav, mp3 等
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// 采样率
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i32>,
    /// 是否开启标点符号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_punctuation: Option<bool>,
    /// 是否开启顺滑
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_inverse_text_normalization: Option<bool>,
}

impl SpeechRecognizeBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.audio, "audio 不能为空");
        Ok(())
    }
}

/// 通用语音识别响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechRecognizeResponse {
    /// 识别结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<SpeechRecognizeResult>,
}

impl openlark_core::api::ApiResponseTrait for SpeechRecognizeResponse {}

/// 语音识别结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechRecognizeResult {
    /// 识别出的文本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// 识别置信度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    /// 分词结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub words: Option<Vec<WordInfo>>,
}

/// 分词信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordInfo {
    /// 词语
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word: Option<String>,
    /// 开始时间（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}

/// 通用语音识别请求
#[derive(Debug, Clone)]
pub struct SpeechRecognizeRequest {
    config: Config,
}

impl SpeechRecognizeRequest {
    /// 创建新的通用语音识别请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行通用语音识别请求
    pub async fn execute(self, body: SpeechRecognizeBody) -> SDKResult<SpeechRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行通用语音识别请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: SpeechRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<SpeechRecognizeResponse> {
        body.validate()?;

        let req: ApiRequest<SpeechRecognizeResponse> =
            ApiRequest::post(SPEECH_TO_TEXT_V1_SPEECH_RECOGNIZE)
                .body(serialize_params(&body, "通用语音识别")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "通用语音识别")
    }
}

/// 通用语音识别请求构建器
#[derive(Debug, Clone)]
pub struct SpeechRecognizeRequestBuilder {
    request: SpeechRecognizeRequest,
    audio: Option<String>,
    audio_type: Option<String>,
    language: Option<String>,
    format: Option<String>,
    sample_rate: Option<i32>,
    enable_punctuation: Option<bool>,
    enable_inverse_text_normalization: Option<bool>,
}

impl SpeechRecognizeRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Config) -> Self {
        Self {
            request: SpeechRecognizeRequest::new(config),
            audio: None,
            audio_type: None,
            language: None,
            format: None,
            sample_rate: None,
            enable_punctuation: None,
            enable_inverse_text_normalization: None,
        }
    }

    /// 设置音频数据
    pub fn audio(mut self, audio: impl Into<String>) -> Self {
        self.audio = Some(audio.into());
        self
    }

    /// 设置音频类型
    pub fn audio_type(mut self, audio_type: impl Into<String>) -> Self {
        self.audio_type = Some(audio_type.into());
        self
    }

    /// 设置语言
    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// 设置音频格式
    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }

    /// 设置采样率
    pub fn sample_rate(mut self, sample_rate: impl Into<i32>) -> Self {
        self.sample_rate = Some(sample_rate.into());
        self
    }

    /// 设置是否开启标点符号
    pub fn enable_punctuation(mut self, enable: impl Into<bool>) -> Self {
        self.enable_punctuation = Some(enable.into());
        self
    }

    /// 设置是否开启顺滑
    pub fn enable_inverse_text_normalization(mut self, enable: impl Into<bool>) -> Self {
        self.enable_inverse_text_normalization = Some(enable.into());
        self
    }

    /// 构建请求体
    pub fn body(self) -> SpeechRecognizeBody {
        SpeechRecognizeBody {
            audio: self.audio.unwrap_or_default(),
            audio_type: self.audio_type,
            language: self.language,
            format: self.format,
            sample_rate: self.sample_rate,
            enable_punctuation: self.enable_punctuation,
            enable_inverse_text_normalization: self.enable_inverse_text_normalization,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SpeechRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SpeechRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行通用语音识别
///
/// docPath: https://open.feishu.cn/document/server-docs/ai/speech_to_text-v1/speech/recognize
pub async fn speech_recognize(
    config: &Config,
    body: SpeechRecognizeBody,
) -> SDKResult<SpeechRecognizeResponse> {
    speech_recognize_with_options(config, body, RequestOption::default()).await
}

/// 执行通用语音识别（支持自定义选项）
pub async fn speech_recognize_with_options(
    config: &Config,
    body: SpeechRecognizeBody,
    option: RequestOption,
) -> SDKResult<SpeechRecognizeResponse> {
    body.validate()?;

    let req: ApiRequest<SpeechRecognizeResponse> =
        ApiRequest::post(SPEECH_TO_TEXT_V1_SPEECH_RECOGNIZE)
            .body(serialize_params(&body, "通用语音识别")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "通用语音识别")
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
        let builder = SpeechRecognizeRequestBuilder::new(config.clone());

        assert!(builder.audio.is_none());
        assert!(builder.audio_type.is_none());
        assert!(builder.language.is_none());
        assert!(builder.format.is_none());
        assert!(builder.sample_rate.is_none());
        assert!(builder.enable_punctuation.is_none());
        assert!(builder.enable_inverse_text_normalization.is_none());
    }

    #[test]
    fn test_builder_audio() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = SpeechRecognizeRequestBuilder::new(config.clone()).audio("base64_audio_data");

        assert_eq!(builder.audio, Some("base64_audio_data".to_string()));
    }

    #[test]
    fn test_builder_sample_rate() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = SpeechRecognizeRequestBuilder::new(config.clone()).sample_rate(16000);

        assert_eq!(builder.sample_rate, Some(16000));
    }

    #[test]
    fn test_body_validation_empty_audio() {
        let body = SpeechRecognizeBody {
            audio: "".to_string(),
            audio_type: None,
            language: None,
            format: None,
            sample_rate: None,
            enable_punctuation: None,
            enable_inverse_text_normalization: None,
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = SpeechRecognizeBody {
            audio: "base64_audio_data".to_string(),
            audio_type: Some("base64".to_string()),
            language: Some("zh".to_string()),
            format: Some("pcm".to_string()),
            sample_rate: Some(16000),
            enable_punctuation: Some(true),
            enable_inverse_text_normalization: Some(true),
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
        let body = SpeechRecognizeRequestBuilder::new(config.clone())
            .audio("base64_data")
            .audio_type("base64")
            .language("zh")
            .format("pcm")
            .sample_rate(16000)
            .enable_punctuation(true)
            .enable_inverse_text_normalization(true)
            .body();

        assert_eq!(body.audio, "base64_data");
        assert_eq!(body.audio_type, Some("base64".to_string()));
        assert_eq!(body.language, Some("zh".to_string()));
        assert_eq!(body.format, Some("pcm".to_string()));
        assert_eq!(body.sample_rate, Some(16000));
        assert_eq!(body.enable_punctuation, Some(true));
        assert_eq!(body.enable_inverse_text_normalization, Some(true));
    }

    #[test]
    fn test_body_validation_whitespace() {
        let body = SpeechRecognizeBody {
            audio: "   ".to_string(),
            audio_type: None,
            language: None,
            format: None,
            sample_rate: None,
            enable_punctuation: None,
            enable_inverse_text_normalization: None,
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_serialization() {
        let body = SpeechRecognizeBody {
            audio: "base64_audio_data".to_string(),
            audio_type: Some("base64".to_string()),
            language: Some("zh".to_string()),
            format: None,
            sample_rate: Some(16000),
            enable_punctuation: Some(true),
            enable_inverse_text_normalization: None,
        };
        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(json_str.contains("audio"));
        assert!(json_str.contains("base64_audio_data"));
        assert!(!json_str.contains("format")); // None 不应序列化
    }

    #[test]
    fn test_response_struct() {
        let response = SpeechRecognizeResponse { data: None };
        assert!(response.data.is_none());

        let result = SpeechRecognizeResult {
            text: Some("你好世界".to_string()),
            confidence: Some(0.95),
            words: None,
        };
        let response_with_data = SpeechRecognizeResponse { data: Some(result) };
        assert!(response_with_data.data.is_some());
        let text = response_with_data
            .data
            .as_ref()
            .and_then(|data| data.text.as_deref());
        assert_eq!(text, Some("你好世界"));
    }

    #[test]
    fn test_result_struct() {
        let result = SpeechRecognizeResult {
            text: Some("测试识别结果".to_string()),
            confidence: Some(0.98),
            words: Some(vec![WordInfo {
                word: Some("测试".to_string()),
                start_time: Some(0),
                end_time: Some(500),
            }]),
        };
        assert_eq!(result.text, Some("测试识别结果".to_string()));
        assert_eq!(result.confidence, Some(0.98));
        assert!(result.words.is_some());
    }

    #[test]
    fn test_word_info_struct() {
        let word_info = WordInfo {
            word: Some("你好".to_string()),
            start_time: Some(100),
            end_time: Some(400),
        };
        assert_eq!(word_info.word, Some("你好".to_string()));
        assert_eq!(word_info.start_time, Some(100));
        assert_eq!(word_info.end_time, Some(400));
    }
}
