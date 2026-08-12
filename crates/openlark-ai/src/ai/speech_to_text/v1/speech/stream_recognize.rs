//! 识别流式语音
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/speech_to_text-v1/speech/stream_recognize>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::serialize_params;
use crate::endpoints::SPEECH_TO_TEXT_V1_SPEECH_STREAM_RECOGNIZE;

use super::file_recognize::SpeechContent;

/// 流式语音识别配置（`stream_config`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StreamRecognizeConfig {
    /// 流标识（同一会话保持一致）。
    pub stream_id: String,
    /// 分片序号（从 1 递增）。
    pub sequence_id: i32,
    /// 分片动作：1=中间分片，2=最后分片等（以官方文档为准）。
    pub action: i32,
    /// 音频格式，如 `pcm`。
    pub format: String,
    /// 引擎类型，如 `16k_auto`。
    pub engine_type: String,
}

/// 流式语音识别请求体。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamRecognizeBody {
    /// 语音内容。
    pub speech: SpeechContent,
    /// 流式识别配置。
    pub config: StreamRecognizeConfig,
}

impl StreamRecognizeBody {
    /// 验证请求参数。
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.speech.speech, "speech.speech 不能为空");
        validate_required!(self.config.stream_id, "config.stream_id 不能为空");
        validate_required!(self.config.format, "config.format 不能为空");
        validate_required!(self.config.engine_type, "config.engine_type 不能为空");
        Ok(())
    }
}

/// 流式语音识别响应 `data`。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StreamRecognizeResponse {
    /// 识别出的文本。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recognition_text: Option<String>,
    /// 流标识。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    /// 分片序号。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_id: Option<i32>,
}

impl openlark_core::api::ApiResponseTrait for StreamRecognizeResponse {
    fn empty_success() -> Option<Self> {
        Some(Self::default())
    }
}

/// 流式语音识别请求。
#[derive(Debug, Clone)]
pub struct StreamRecognizeRequest {
    config: Config,
}

impl StreamRecognizeRequest {
    /// 创建新的流式语音识别请求。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行流式语音识别请求。
    pub async fn execute(self, body: StreamRecognizeBody) -> SDKResult<StreamRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行流式语音识别请求（支持自定义选项）。
    pub async fn execute_with_options(
        self,
        body: StreamRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<StreamRecognizeResponse> {
        body.validate()?;

        let req: ApiRequest<StreamRecognizeResponse> =
            ApiRequest::post(SPEECH_TO_TEXT_V1_SPEECH_STREAM_RECOGNIZE)
                .body(serialize_params(&body, "流式语音识别")?);

        Transport::request_typed(req, &self.config, Some(option), "流式语音识别").await
    }
}

/// 流式语音识别请求构建器。
#[derive(Debug, Clone)]
pub struct StreamRecognizeRequestBuilder {
    request: StreamRecognizeRequest,
    speech: Option<String>,
    stream_id: Option<String>,
    sequence_id: Option<i32>,
    action: Option<i32>,
    format: Option<String>,
    engine_type: Option<String>,
}

impl StreamRecognizeRequestBuilder {
    /// 创建新的构建器。
    pub fn new(config: Config) -> Self {
        Self {
            request: StreamRecognizeRequest::new(config),
            speech: None,
            stream_id: None,
            sequence_id: None,
            action: None,
            format: None,
            engine_type: None,
        }
    }

    /// 设置音频 base64 内容。
    pub fn speech(mut self, speech: impl Into<String>) -> Self {
        self.speech = Some(speech.into());
        self
    }

    /// 设置流标识。
    pub fn stream_id(mut self, stream_id: impl Into<String>) -> Self {
        self.stream_id = Some(stream_id.into());
        self
    }

    /// 设置分片序号。
    pub fn sequence_id(mut self, sequence_id: i32) -> Self {
        self.sequence_id = Some(sequence_id);
        self
    }

    /// 设置分片动作。
    pub fn action(mut self, action: i32) -> Self {
        self.action = Some(action);
        self
    }

    /// 设置音频格式。
    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }

    /// 设置引擎类型。
    pub fn engine_type(mut self, engine_type: impl Into<String>) -> Self {
        self.engine_type = Some(engine_type.into());
        self
    }

    /// 构建请求体。
    pub fn body(self) -> StreamRecognizeBody {
        StreamRecognizeBody {
            speech: SpeechContent {
                speech: self.speech.unwrap_or_default(),
            },
            config: StreamRecognizeConfig {
                stream_id: self.stream_id.unwrap_or_default(),
                sequence_id: self.sequence_id.unwrap_or(1),
                action: self.action.unwrap_or(1),
                format: self.format.unwrap_or_default(),
                engine_type: self.engine_type.unwrap_or_default(),
            },
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<StreamRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 执行请求（支持自定义选项）。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<StreamRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行流式语音识别。
pub async fn stream_recognize(
    config: &Config,
    body: StreamRecognizeBody,
) -> SDKResult<StreamRecognizeResponse> {
    stream_recognize_with_options(config, body, RequestOption::default()).await
}

/// 执行流式语音识别（支持自定义选项）。
pub async fn stream_recognize_with_options(
    config: &Config,
    body: StreamRecognizeBody,
    option: RequestOption,
) -> SDKResult<StreamRecognizeResponse> {
    body.validate()?;

    let req: ApiRequest<StreamRecognizeResponse> =
        ApiRequest::post(SPEECH_TO_TEXT_V1_SPEECH_STREAM_RECOGNIZE)
            .body(serialize_params(&body, "流式语音识别")?);

    Transport::request_typed(req, config, Some(option), "流式语音识别").await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_body_serialization_official_fields() {
        let body = StreamRecognizeBody {
            speech: SpeechContent {
                speech: "PdmrfE==".into(),
            },
            config: StreamRecognizeConfig {
                stream_id: "asd123".into(),
                sequence_id: 1,
                action: 1,
                format: "pcm".into(),
                engine_type: "16k_auto".into(),
            },
        };
        let v = serde_json::to_value(&body).unwrap();
        assert_eq!(v["speech"]["speech"], "PdmrfE==");
        assert_eq!(v["config"]["stream_id"], "asd123");
        assert_eq!(v["config"]["sequence_id"], 1);
        assert!(v.get("audio").is_none());
        assert!(v.get("language").is_none());
    }

    #[test]
    fn test_builder() {
        let config = Config::builder()
            .app_id("a")
            .app_secret("s")
            .build();
        let body = StreamRecognizeRequestBuilder::new(config)
            .speech("b64")
            .stream_id("sid")
            .sequence_id(2)
            .action(1)
            .format("pcm")
            .engine_type("16k_auto")
            .body();
        assert_eq!(body.config.sequence_id, 2);
        assert_eq!(body.speech.speech, "b64");
    }
}
