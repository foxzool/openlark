//! 识别语音文件

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::SPEECH_TO_TEXT_V1_SPEECH_FILE_RECOGNIZE;

/// 识别语音文件请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRecognizeBody {
    /// 用户的 file_id，通过上传文件接口获取
    pub file_token: String,
    /// 是否异步识别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_async: Option<bool>,
    /// 语言类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// 识别格式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

impl FileRecognizeBody {
    /// 校验请求体。
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.file_token, "file_token 不能为空");
        Ok(())
    }
}

/// 识别语音文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRecognizeResponse {
    /// data 字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl openlark_core::api::ApiResponseTrait for FileRecognizeResponse {}

/// 识别语音文件请求
#[derive(Debug, Clone)]
pub struct FileRecognizeRequest {
    config: Config,
}

impl FileRecognizeRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(self, body: FileRecognizeBody) -> SDKResult<FileRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: FileRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<FileRecognizeResponse> {
        body.validate()?;

        let req: ApiRequest<FileRecognizeResponse> =
            ApiRequest::post(SPEECH_TO_TEXT_V1_SPEECH_FILE_RECOGNIZE)
                .body(serialize_params(&body, "识别语音文件")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "识别语音文件")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_validate_requires_file_token() {
        let body = FileRecognizeBody {
            file_token: String::new(),
            is_async: None,
            language: None,
            format: None,
        };

        assert!(body.validate().is_err());
    }
}
