//! 识别流式语音
//!
//! 语音流式接口，将整个音频文件分片进行传入模型。
//! 能够实时返回数据。建议每个音频分片的大小为 100-200ms。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/ai/speech_to_text-v1/stream_recognize>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::SPEECH_TO_TEXT_V1_STREAM_RECOGNIZE;

/// 流式语音识别请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamRecognizeBody {
    /// 音频数据（Base64 编码）
    pub audio: String,
    /// 语言类型，默认为中文
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// 识别格式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// 是否开启中间结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_intermediate_result: Option<bool>,
    /// 是否开启标点符号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_punctuation: Option<bool>,
    /// 是否开启自动分段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_automatic_segmentation: Option<bool>,
}

impl StreamRecognizeBody {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.audio.trim().is_empty() {
            return Err("audio 不能为空".to_string());
        }
        Ok(())
    }
}

/// 流式语音识别响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamRecognizeResponse {
    /// 识别结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<StreamRecognizeResult>,
}

impl openlark_core::api::ApiResponseTrait for StreamRecognizeResponse {}

/// 流式识别结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamRecognizeResult {
    /// 识别出的文本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// 是否为最终结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_final: Option<bool>,
    /// 句子序号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentence_id: Option<i32>,
    /// 开始时间（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}

/// 流式语音识别请求
#[derive(Debug, Clone)]
pub struct StreamRecognizeRequest {
    config: Config,
}

impl StreamRecognizeRequest {
    /// 创建新的流式语音识别请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行流式语音识别请求
    pub async fn execute(self, body: StreamRecognizeBody) -> SDKResult<StreamRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行流式语音识别请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: StreamRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<StreamRecognizeResponse> {
        body.validate()
            .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

        let req: ApiRequest<StreamRecognizeResponse> =
            ApiRequest::post(SPEECH_TO_TEXT_V1_STREAM_RECOGNIZE)
                .body(serialize_params(&body, "流式语音识别")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "流式语音识别")
    }
}

/// 流式语音识别请求构建器
#[derive(Debug, Clone)]
pub struct StreamRecognizeRequestBuilder {
    request: StreamRecognizeRequest,
    audio: Option<String>,
    language: Option<String>,
    format: Option<String>,
    enable_intermediate_result: Option<bool>,
    enable_punctuation: Option<bool>,
    enable_automatic_segmentation: Option<bool>,
}

impl StreamRecognizeRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Config) -> Self {
        Self {
            request: StreamRecognizeRequest::new(config),
            audio: None,
            language: None,
            format: None,
            enable_intermediate_result: None,
            enable_punctuation: None,
            enable_automatic_segmentation: None,
        }
    }

    /// 设置音频数据（Base64 编码）
    pub fn audio(mut self, audio: impl Into<String>) -> Self {
        self.audio = Some(audio.into());
        self
    }

    /// 设置语言
    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// 设置识别格式
    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }

    /// 设置是否开启中间结果
    pub fn enable_intermediate_result(mut self, enable: impl Into<bool>) -> Self {
        self.enable_intermediate_result = Some(enable.into());
        self
    }

    /// 设置是否开启标点符号
    pub fn enable_punctuation(mut self, enable: impl Into<bool>) -> Self {
        self.enable_punctuation = Some(enable.into());
        self
    }

    /// 设置是否开启自动分段
    pub fn enable_automatic_segmentation(mut self, enable: impl Into<bool>) -> Self {
        self.enable_automatic_segmentation = Some(enable.into());
        self
    }

    /// 构建请求体
    pub fn body(self) -> StreamRecognizeBody {
        StreamRecognizeBody {
            audio: self.audio.unwrap_or_default(),
            language: self.language,
            format: self.format,
            enable_intermediate_result: self.enable_intermediate_result,
            enable_punctuation: self.enable_punctuation,
            enable_automatic_segmentation: self.enable_automatic_segmentation,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<StreamRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<StreamRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行流式语音识别
///
/// docPath: <https://open.feishu.cn/document/server-docs/ai/speech_to_text-v1/stream_recognize>
pub async fn stream_recognize(
    config: &Config,
    body: StreamRecognizeBody,
) -> SDKResult<StreamRecognizeResponse> {
    stream_recognize_with_options(config, body, RequestOption::default()).await
}

/// 执行流式语音识别（支持自定义选项）
pub async fn stream_recognize_with_options(
    config: &Config,
    body: StreamRecognizeBody,
    option: RequestOption,
) -> SDKResult<StreamRecognizeResponse> {
    body.validate()
        .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

    let req: ApiRequest<StreamRecognizeResponse> =
        ApiRequest::post(SPEECH_TO_TEXT_V1_STREAM_RECOGNIZE)
            .body(serialize_params(&body, "流式语音识别")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "流式语音识别")
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
        let builder = StreamRecognizeRequestBuilder::new(config.clone());

        assert!(builder.audio.is_none());
        assert!(builder.language.is_none());
        assert!(builder.format.is_none());
        assert!(builder.enable_intermediate_result.is_none());
        assert!(builder.enable_punctuation.is_none());
        assert!(builder.enable_automatic_segmentation.is_none());
    }

    #[test]
    fn test_builder_audio() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = StreamRecognizeRequestBuilder::new(config.clone()).audio("base64_audio_data");

        assert_eq!(builder.audio, Some("base64_audio_data".to_string()));
    }

    #[test]
    fn test_builder_enable_intermediate_result() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder =
            StreamRecognizeRequestBuilder::new(config.clone()).enable_intermediate_result(true);

        assert_eq!(builder.enable_intermediate_result, Some(true));
    }

    #[test]
    fn test_body_validation_empty_audio() {
        let body = StreamRecognizeBody {
            audio: "".to_string(),
            language: None,
            format: None,
            enable_intermediate_result: None,
            enable_punctuation: None,
            enable_automatic_segmentation: None,
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = StreamRecognizeBody {
            audio: "base64_audio_data".to_string(),
            language: Some("zh".to_string()),
            format: Some("pcm".to_string()),
            enable_intermediate_result: Some(true),
            enable_punctuation: Some(true),
            enable_automatic_segmentation: Some(false),
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
        let body = StreamRecognizeRequestBuilder::new(config.clone())
            .audio("base64_data")
            .language("zh")
            .format("pcm")
            .enable_intermediate_result(true)
            .enable_punctuation(true)
            .enable_automatic_segmentation(false)
            .body();

        assert_eq!(body.audio, "base64_data");
        assert_eq!(body.language, Some("zh".to_string()));
        assert_eq!(body.format, Some("pcm".to_string()));
        assert_eq!(body.enable_intermediate_result, Some(true));
        assert_eq!(body.enable_punctuation, Some(true));
        assert_eq!(body.enable_automatic_segmentation, Some(false));
    }

    #[test]
    fn test_body_validation_whitespace() {
        let body = StreamRecognizeBody {
            audio: "   ".to_string(),
            language: None,
            format: None,
            enable_intermediate_result: None,
            enable_punctuation: None,
            enable_automatic_segmentation: None,
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_serialization() {
        let body = StreamRecognizeBody {
            audio: "base64_audio_data".to_string(),
            language: Some("zh".to_string()),
            format: None,
            enable_intermediate_result: Some(true),
            enable_punctuation: None,
            enable_automatic_segmentation: None,
        };
        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(json_str.contains("audio"));
        assert!(json_str.contains("base64_audio_data"));
        assert!(!json_str.contains("format")); // None 不应序列化
    }

    #[test]
    fn test_response_struct() {
        let response = StreamRecognizeResponse { data: None };
        assert!(response.data.is_none());

        let result = StreamRecognizeResult {
            text: Some("你好世界".to_string()),
            is_final: Some(true),
            sentence_id: Some(1),
            start_time: Some(0),
            end_time: Some(1500),
        };
        let response_with_data = StreamRecognizeResponse { data: Some(result) };
        assert!(response_with_data.data.is_some());
        assert_eq!(
            response_with_data.data.unwrap().text,
            Some("你好世界".to_string())
        );
    }

    #[test]
    fn test_result_struct() {
        let result = StreamRecognizeResult {
            text: Some("测试识别结果".to_string()),
            is_final: Some(true),
            sentence_id: Some(2),
            start_time: Some(100),
            end_time: Some(500),
        };
        assert_eq!(result.text, Some("测试识别结果".to_string()));
        assert_eq!(result.is_final, Some(true));
        assert_eq!(result.sentence_id, Some(2));
    }
}
