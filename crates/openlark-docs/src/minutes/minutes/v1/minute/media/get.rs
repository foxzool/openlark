//! 下载妙记音视频文件
//!
//! docPath: <https://open.feishu.cn/document/minutes-v1/minute-media/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::MinutesApiV1, api_utils::*};

#[derive(Debug, Clone)]
pub struct GetMinuteMediaRequest {
    config: Config,
    minute_token: Option<String>,
}

impl GetMinuteMediaRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            minute_token: None,
        }
    }

    pub fn minute_token(mut self, minute_token: impl Into<String>) -> Self {
        self.minute_token = Some(minute_token.into());
        self
    }

    pub async fn execute(self) -> SDKResult<GetMinuteMediaResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetMinuteMediaResponse> {
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

        // ===== 构建请求 =====
        let api_endpoint = MinutesApiV1::MediaGet(minute_token);
        let api_request: ApiRequest<GetMinuteMediaResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // ===== 发送请求 =====
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试构建器模式
    #[test]
    fn test_get_minute_media_builder() {
        let config = Config::default();
        let request = GetMinuteMediaRequest::new(config).minute_token("123456789012345678901234");

        assert_eq!(
            request.minute_token,
            Some("123456789012345678901234".to_string())
        );
    }

    /// 测试响应数据结构
    #[test]
    fn test_get_minute_media_response() {
        let response = GetMinuteMediaResponse {
            download_url: "https://example.com/media.mp4".to_string(),
        };

        assert_eq!(response.download_url, "https://example.com/media.mp4");
    }

    /// 测试响应trait实现
    #[test]
    fn test_response_trait() {
        assert_eq!(GetMinuteMediaResponse::data_format(), ResponseFormat::Data);
    }

    /// 测试有效的minute_token
    #[test]
    fn test_valid_minute_token() {
        let config = Config::default();
        let valid_token = "a".repeat(24);
        let request = GetMinuteMediaRequest::new(config).minute_token(&valid_token);

        assert_eq!(request.minute_token.unwrap().len(), 24);
    }

    /// 测试空minute_token
    #[test]
    fn test_empty_minute_token() {
        let config = Config::default();
        let request = GetMinuteMediaRequest::new(config);

        assert!(request.minute_token.is_none());
    }

    /// 测试下载URL格式
    #[test]
    fn test_download_url_format() {
        let url1 = "https://cdn.example.com/files/abc123.mp4";
        let url2 = "https://storage.example.com/bucket/media.wav";

        let response1 = GetMinuteMediaResponse {
            download_url: url1.to_string(),
        };
        let response2 = GetMinuteMediaResponse {
            download_url: url2.to_string(),
        };

        assert!(response1.download_url.starts_with("https://"));
        assert!(response2.download_url.ends_with(".wav"));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMinuteMediaResponse {
    pub download_url: String,
}

impl ApiResponseTrait for GetMinuteMediaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
