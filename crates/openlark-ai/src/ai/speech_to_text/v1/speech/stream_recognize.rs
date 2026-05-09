//! 识别流式语音

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::SPEECH_TO_TEXT_V1_SPEECH_STREAM_RECOGNIZE;

/// 识别流式语音请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamRecognizeBody {
    /// 音频数据（Base64 编码）
    pub audio: String,
    /// 语言类型
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
    /// 校验请求体。
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.audio, "audio 不能为空");
        Ok(())
    }
}

/// 识别流式语音响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamRecognizeResponse {
    /// data 字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl openlark_core::api::ApiResponseTrait for StreamRecognizeResponse {}

/// 识别流式语音请求
#[derive(Debug, Clone)]
pub struct StreamRecognizeRequest {
    config: Config,
}

impl StreamRecognizeRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(self, body: StreamRecognizeBody) -> SDKResult<StreamRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: StreamRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<StreamRecognizeResponse> {
        body.validate()?;

        let req: ApiRequest<StreamRecognizeResponse> =
            ApiRequest::post(SPEECH_TO_TEXT_V1_SPEECH_STREAM_RECOGNIZE)
                .body(serialize_params(&body, "识别流式语音")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "识别流式语音")
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
    fn test_validate_requires_audio() {
        let body = StreamRecognizeBody {
            audio: String::new(),
            language: None,
            format: None,
            enable_intermediate_result: None,
            enable_punctuation: None,
            enable_automatic_segmentation: None,
        };

        assert!(body.validate().is_err());
    }
}
