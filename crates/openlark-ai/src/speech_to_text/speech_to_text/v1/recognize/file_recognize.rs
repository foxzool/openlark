//! 识别语音文件
//!
//! 语音文件识别接口，上传整段语音文件进行一次性识别。
//! 接口适合 60 秒以内音频识别。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/ai/speech_to_text-v1/file_recognize>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::SPEECH_TO_TEXT_V1_FILE_RECOGNIZE;

/// 语音文件识别请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRecognizeBody {
    /// 用户的 file_id，通过上传文件接口获取
    pub file_token: String,
    /// 是否异步识别，默认为 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_async: Option<bool>,
    /// 语言类型，默认为中文
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// 识别格式，默认为完整识别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

impl FileRecognizeBody {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.file_token.trim().is_empty() {
            return Err("file_token 不能为空".to_string());
        }
        Ok(())
    }
}

/// 语音文件识别响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRecognizeResponse {
    /// 识别结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<FileRecognizeResult>,
}

impl openlark_core::api::ApiResponseTrait for FileRecognizeResponse {}

/// 语音识别结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRecognizeResult {
    /// 识别出的文本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// 任务 ID（异步模式下使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 识别状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// 语音文件识别请求
#[derive(Debug, Clone)]
pub struct FileRecognizeRequest {
    config: Config,
}

impl FileRecognizeRequest {
    /// 创建新的语音文件识别请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行语音文件识别请求
    pub async fn execute(self, body: FileRecognizeBody) -> SDKResult<FileRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行语音文件识别请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: FileRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<FileRecognizeResponse> {
        body.validate()
            .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

        let req: ApiRequest<FileRecognizeResponse> =
            ApiRequest::post(SPEECH_TO_TEXT_V1_FILE_RECOGNIZE)
                .body(serialize_params(&body, "语音文件识别")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "语音文件识别")
    }
}

/// 语音文件识别请求构建器
#[derive(Debug, Clone)]
pub struct FileRecognizeRequestBuilder {
    request: FileRecognizeRequest,
    file_token: Option<String>,
    is_async: Option<bool>,
    language: Option<String>,
    format: Option<String>,
}

impl FileRecognizeRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Config) -> Self {
        Self {
            request: FileRecognizeRequest::new(config),
            file_token: None,
            is_async: None,
            language: None,
            format: None,
        }
    }

    /// 设置文件 token
    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.file_token = Some(file_token.into());
        self
    }

    /// 设置是否异步
    pub fn is_async(mut self, is_async: impl Into<bool>) -> Self {
        self.is_async = Some(is_async.into());
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

    /// 构建请求体
    pub fn body(self) -> FileRecognizeBody {
        FileRecognizeBody {
            file_token: self.file_token.unwrap_or_default(),
            is_async: self.is_async,
            language: self.language,
            format: self.format,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<FileRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<FileRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行语音文件识别
///
/// docPath: <https://open.feishu.cn/document/server-docs/ai/speech_to_text-v1/file_recognize>
pub async fn file_recognize(
    config: &Config,
    body: FileRecognizeBody,
) -> SDKResult<FileRecognizeResponse> {
    file_recognize_with_options(config, body, RequestOption::default()).await
}

/// 执行语音文件识别（支持自定义选项）
pub async fn file_recognize_with_options(
    config: &Config,
    body: FileRecognizeBody,
    option: RequestOption,
) -> SDKResult<FileRecognizeResponse> {
    body.validate()
        .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

    let req: ApiRequest<FileRecognizeResponse> = ApiRequest::post(SPEECH_TO_TEXT_V1_FILE_RECOGNIZE)
        .body(serialize_params(&body, "语音文件识别")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "语音文件识别")
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
        let builder = FileRecognizeRequestBuilder::new(config.clone());

        assert!(builder.file_token.is_none());
        assert!(builder.is_async.is_none());
        assert!(builder.language.is_none());
        assert!(builder.format.is_none());
    }

    #[test]
    fn test_builder_file_token() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = FileRecognizeRequestBuilder::new(config.clone()).file_token("file_token_123");

        assert_eq!(builder.file_token, Some("file_token_123".to_string()));
    }

    #[test]
    fn test_builder_is_async() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = FileRecognizeRequestBuilder::new(config.clone()).is_async(true);

        assert_eq!(builder.is_async, Some(true));
    }

    #[test]
    fn test_body_validation_empty_file_token() {
        let body = FileRecognizeBody {
            file_token: "".to_string(),
            is_async: None,
            language: None,
            format: None,
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = FileRecognizeBody {
            file_token: "valid_token".to_string(),
            is_async: Some(true),
            language: Some("zh".to_string()),
            format: Some("full".to_string()),
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
        let body = FileRecognizeRequestBuilder::new(config.clone())
            .file_token("token_123")
            .is_async(true)
            .language("zh")
            .format("full")
            .body();

        assert_eq!(body.file_token, "token_123");
        assert_eq!(body.is_async, Some(true));
        assert_eq!(body.language, Some("zh".to_string()));
        assert_eq!(body.format, Some("full".to_string()));
    }

    #[test]
    fn test_body_validation_whitespace() {
        let body = FileRecognizeBody {
            file_token: "   ".to_string(),
            is_async: None,
            language: None,
            format: None,
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_serialization() {
        let body = FileRecognizeBody {
            file_token: "token_123".to_string(),
            is_async: Some(true),
            language: Some("zh".to_string()),
            format: None,
        };
        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(json_str.contains("file_token"));
        assert!(json_str.contains("token_123"));
        assert!(!json_str.contains("format")); // None 不应序列化
    }

    #[test]
    fn test_response_struct() {
        let response = FileRecognizeResponse { data: None };
        assert!(response.data.is_none());

        let result = FileRecognizeResult {
            text: Some("你好世界".to_string()),
            task_id: Some("task_123".to_string()),
            status: Some("completed".to_string()),
        };
        let response_with_data = FileRecognizeResponse { data: Some(result) };
        assert!(response_with_data.data.is_some());
        assert_eq!(
            response_with_data.data.unwrap().text,
            Some("你好世界".to_string())
        );
    }
}
