//! 导出妙记文字记录
//!
//! docPath: <https://open.feishu.cn/document/minutes-v1/minute-transcript/get>

use openlark_core::{
    api::{ApiRequest, Response},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

use crate::common::api_endpoints::MinutesApiV1;

#[derive(Debug, Clone)]
pub struct GetMinuteTranscriptRequest {
    config: Config,
    minute_token: Option<String>,
    need_speaker: Option<bool>,
    need_timestamp: Option<bool>,
    file_format: Option<String>,
}

impl GetMinuteTranscriptRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            minute_token: None,
            need_speaker: None,
            need_timestamp: None,
            file_format: None,
        }
    }

    pub fn minute_token(mut self, minute_token: impl Into<String>) -> Self {
        self.minute_token = Some(minute_token.into());
        self
    }

    pub fn need_speaker(mut self, need_speaker: bool) -> Self {
        self.need_speaker = Some(need_speaker);
        self
    }

    pub fn need_timestamp(mut self, need_timestamp: bool) -> Self {
        self.need_timestamp = Some(need_timestamp);
        self
    }

    pub fn file_format(mut self, file_format: impl Into<String>) -> Self {
        self.file_format = Some(file_format.into());
        self
    }

    /// 成功时返回文件二进制内容（`Response<Vec<u8>>`）。
    pub async fn execute(self) -> SDKResult<Response<Vec<u8>>> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 成功时返回文件二进制内容（支持自定义选项）。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<Response<Vec<u8>>> {
        // ===== 参数校验 =====
        let minute_token = self.minute_token.ok_or_else(|| {
            openlark_core::error::validation_error("minute_token", "minute_token 不能为空")
        })?;
        if minute_token.chars().count() != 24 {
            return Err(openlark_core::error::validation_error(
                "minute_token",
                "minute_token 长度必须为 24 字符",
            ));
        }
        if let Some(fmt) = &self.file_format {
            match fmt.as_str() {
                "txt" | "srt" => {}
                _ => {
                    return Err(openlark_core::error::validation_error(
                        "file_format",
                        "file_format 仅支持 txt/srt",
                    ));
                }
            }
        }

        // ===== 构建请求 =====
        let api_endpoint = MinutesApiV1::TranscriptGet(minute_token);
        let mut api_request: ApiRequest<Vec<u8>> = ApiRequest::get(&api_endpoint.to_url());

        if let Some(v) = self.need_speaker {
            api_request = api_request.query("need_speaker", v.to_string());
        }
        if let Some(v) = self.need_timestamp {
            api_request = api_request.query("need_timestamp", v.to_string());
        }
        if let Some(fmt) = &self.file_format {
            api_request = api_request.query("file_format", fmt);
        }

        // ===== 发送请求 =====
        Transport::request(api_request, &self.config, Some(option)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试构建器模式
    #[test]
    fn test_get_minute_transcript_builder() {
        let config = Config::default();
        let request = GetMinuteTranscriptRequest::new(config)
            .minute_token("123456789012345678901234")
            .need_speaker(true)
            .need_timestamp(true)
            .file_format("txt");

        assert_eq!(
            request.minute_token,
            Some("123456789012345678901234".to_string())
        );
        assert_eq!(request.need_speaker, Some(true));
        assert_eq!(request.need_timestamp, Some(true));
        assert_eq!(request.file_format, Some("txt".to_string()));
    }

    /// 测试有效的minute_token
    #[test]
    fn test_valid_minute_token() {
        let config = Config::default();
        let valid_token = "a".repeat(24);
        let request = GetMinuteTranscriptRequest::new(config).minute_token(&valid_token);

        assert_eq!(request.minute_token.unwrap().len(), 24);
    }

    /// 测试不同file_format
    #[test]
    fn test_different_file_formats() {
        let config = Config::default();

        let txt_request = GetMinuteTranscriptRequest::new(config.clone())
            .minute_token("123456789012345678901234")
            .file_format("txt");
        assert_eq!(txt_request.file_format, Some("txt".to_string()));

        let srt_request = GetMinuteTranscriptRequest::new(config)
            .minute_token("123456789012345678901234")
            .file_format("srt");
        assert_eq!(srt_request.file_format, Some("srt".to_string()));
    }

    /// 测试need_speaker和need_timestamp
    #[test]
    fn test_speaker_and_timestamp_flags() {
        let config = Config::default();
        let request = GetMinuteTranscriptRequest::new(config)
            .minute_token("123456789012345678901234")
            .need_speaker(false)
            .need_timestamp(false);

        assert_eq!(request.need_speaker, Some(false));
        assert_eq!(request.need_timestamp, Some(false));
    }

    /// 测试空参数场景
    #[test]
    fn test_request_without_optional_params() {
        let config = Config::default();
        let request =
            GetMinuteTranscriptRequest::new(config).minute_token("123456789012345678901234");

        assert!(request.need_speaker.is_none());
        assert!(request.need_timestamp.is_none());
        assert!(request.file_format.is_none());
    }

    /// 测试空minute_token
    #[test]
    fn test_empty_minute_token() {
        let config = Config::default();
        let request = GetMinuteTranscriptRequest::new(config);

        assert!(request.minute_token.is_none());
    }

    /// 测试设置所有参数
    #[test]
    fn test_request_with_all_params() {
        let config = Config::default();
        let request = GetMinuteTranscriptRequest::new(config)
            .minute_token("123456789012345678901234")
            .need_speaker(true)
            .need_timestamp(true)
            .file_format("srt");

        assert!(request.minute_token.is_some());
        assert!(request.need_speaker.is_some());
        assert!(request.need_timestamp.is_some());
        assert!(request.file_format.is_some());
    }
}
