//! 识别语音文件
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/speech_to_text-v1/speech/file_recognize>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::serialize_params;
use crate::endpoints::SPEECH_TO_TEXT_V1_SPEECH_FILE_RECOGNIZE;

/// 语音内容（`speech` 对象）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SpeechContent {
    /// 音频内容（pcm 等格式的 base64）。
    pub speech: String,
}

/// 语音文件识别配置（`file_config`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileRecognizeConfig {
    /// 文件标识（调用方生成，用于去重/追踪）。
    pub file_id: String,
    /// 音频格式，如 `pcm`。
    pub format: String,
    /// 引擎类型，如 `16k_auto`。
    pub engine_type: String,
}

/// 语音文件识别请求体。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRecognizeBody {
    /// 语音内容。
    pub speech: SpeechContent,
    /// 识别配置。
    pub config: FileRecognizeConfig,
}

impl FileRecognizeBody {
    /// 验证请求参数。
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.speech.speech, "speech.speech 不能为空");
        validate_required!(self.config.file_id, "config.file_id 不能为空");
        validate_required!(self.config.format, "config.format 不能为空");
        validate_required!(self.config.engine_type, "config.engine_type 不能为空");
        Ok(())
    }
}

/// 语音文件识别响应 `data`。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileRecognizeResponse {
    /// 识别出的文本。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recognition_text: Option<String>,
}

impl openlark_core::api::ApiResponseTrait for FileRecognizeResponse {
    fn empty_success() -> Option<Self> {
        Some(Self::default())
    }
}

/// 语音文件识别请求。
#[derive(Debug, Clone)]
pub struct FileRecognizeRequest {
    config: Config,
}

impl FileRecognizeRequest {
    /// 创建新的语音文件识别请求。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行语音文件识别请求。
    pub async fn execute(self, body: FileRecognizeBody) -> SDKResult<FileRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行语音文件识别请求（支持自定义选项）。
    pub async fn execute_with_options(
        self,
        body: FileRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<FileRecognizeResponse> {
        body.validate()?;

        let req: ApiRequest<FileRecognizeResponse> =
            ApiRequest::post(SPEECH_TO_TEXT_V1_SPEECH_FILE_RECOGNIZE)
                .body(serialize_params(&body, "语音文件识别")?);

        Transport::request_typed(req, &self.config, Some(option), "语音文件识别").await
    }
}

/// 语音文件识别请求构建器。
#[derive(Debug, Clone)]
pub struct FileRecognizeRequestBuilder {
    request: FileRecognizeRequest,
    speech: Option<String>,
    file_id: Option<String>,
    format: Option<String>,
    engine_type: Option<String>,
}

impl FileRecognizeRequestBuilder {
    /// 创建新的构建器。
    pub fn new(config: Config) -> Self {
        Self {
            request: FileRecognizeRequest::new(config),
            speech: None,
            file_id: None,
            format: None,
            engine_type: None,
        }
    }

    /// 设置音频 base64 内容。
    pub fn speech(mut self, speech: impl Into<String>) -> Self {
        self.speech = Some(speech.into());
        self
    }

    /// 设置文件标识。
    pub fn file_id(mut self, file_id: impl Into<String>) -> Self {
        self.file_id = Some(file_id.into());
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
    pub fn body(self) -> FileRecognizeBody {
        FileRecognizeBody {
            speech: SpeechContent {
                speech: self.speech.unwrap_or_default(),
            },
            config: FileRecognizeConfig {
                file_id: self.file_id.unwrap_or_default(),
                format: self.format.unwrap_or_default(),
                engine_type: self.engine_type.unwrap_or_default(),
            },
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<FileRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 执行请求（支持自定义选项）。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<FileRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行语音文件识别。
pub async fn file_recognize(
    config: &Config,
    body: FileRecognizeBody,
) -> SDKResult<FileRecognizeResponse> {
    file_recognize_with_options(config, body, RequestOption::default()).await
}

/// 执行语音文件识别（支持自定义选项）。
pub async fn file_recognize_with_options(
    config: &Config,
    body: FileRecognizeBody,
    option: RequestOption,
) -> SDKResult<FileRecognizeResponse> {
    body.validate()?;

    let req: ApiRequest<FileRecognizeResponse> =
        ApiRequest::post(SPEECH_TO_TEXT_V1_SPEECH_FILE_RECOGNIZE)
            .body(serialize_params(&body, "语音文件识别")?);

    Transport::request_typed(req, config, Some(option), "语音文件识别").await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_body_serialization_official_fields() {
        let body = FileRecognizeBody {
            speech: SpeechContent {
                speech: "PdmrfE==".into(),
            },
            config: FileRecognizeConfig {
                file_id: "qwe12".into(),
                format: "pcm".into(),
                engine_type: "16k_auto".into(),
            },
        };
        let v = serde_json::to_value(&body).unwrap();
        assert_eq!(v["speech"]["speech"], "PdmrfE==");
        assert_eq!(v["config"]["file_id"], "qwe12");
        assert!(v.get("file_token").is_none());
        assert!(v.get("is_async").is_none());
        assert!(v.get("language").is_none());
    }

    #[test]
    fn test_body_validation() {
        let mut body = FileRecognizeBody {
            speech: SpeechContent {
                speech: "x".into(),
            },
            config: FileRecognizeConfig {
                file_id: "id".into(),
                format: "pcm".into(),
                engine_type: "16k_auto".into(),
            },
        };
        assert!(body.validate().is_ok());
        body.speech.speech.clear();
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_builder() {
        let config = Config::builder()
            .app_id("a")
            .app_secret("s")
            .build();
        let body = FileRecognizeRequestBuilder::new(config)
            .speech("b64")
            .file_id("fid")
            .format("pcm")
            .engine_type("16k_auto")
            .body();
        assert_eq!(body.speech.speech, "b64");
        assert_eq!(body.config.file_id, "fid");
    }
}
