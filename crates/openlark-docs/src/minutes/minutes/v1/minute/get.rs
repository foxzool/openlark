//! 获取妙记信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/minutes-v1/minute/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::MinutesApiV1, api_utils::*};

#[derive(Debug, Clone)]
pub struct GetMinuteRequest {
    config: Config,
    minute_token: Option<String>,
    user_id_type: Option<String>,
}

impl GetMinuteRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            minute_token: None,
            user_id_type: None,
        }
    }

    pub fn minute_token(mut self, minute_token: impl Into<String>) -> Self {
        self.minute_token = Some(minute_token.into());
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<GetMinuteResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetMinuteResponse> {
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
        if let Some(user_id_type) = &self.user_id_type {
            match user_id_type.as_str() {
                "open_id" | "union_id" | "user_id" => {}
                _ => {
                    return Err(openlark_core::error::validation_error(
                        "user_id_type",
                        "user_id_type 仅支持 open_id/union_id/user_id",
                    ));
                }
            }
        }

        // ===== 构建请求 =====
        let api_endpoint = MinutesApiV1::Get(minute_token);
        let mut api_request: ApiRequest<GetMinuteResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type);
        }

        // ===== 发送请求 =====
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_minute_request_builder() {
        let config = Config::default();
        let request = GetMinuteRequest::new(config).minute_token("123456789012345678901234");

        assert_eq!(
            request.minute_token,
            Some("123456789012345678901234".to_string())
        );
    }

    #[test]
    fn test_get_minute_request_with_user_id_type() {
        let config = Config::default();
        let request = GetMinuteRequest::new(config)
            .minute_token("123456789012345678901234")
            .user_id_type("union_id");

        assert_eq!(request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_minute_info_structure() {
        let info = MinuteInfo {
            token: "token_123".to_string(),
            owner_id: "owner_456".to_string(),
            create_time: "1234567890".to_string(),
            title: "会议标题".to_string(),
            cover: "cover_url".to_string(),
            duration: "3600000".to_string(),
            url: "https://example.com".to_string(),
        };

        assert_eq!(info.token, "token_123");
        assert_eq!(info.title, "会议标题");
    }

    #[test]
    fn test_get_minute_response_structure() {
        let info = MinuteInfo {
            token: "token_123".to_string(),
            owner_id: "owner_456".to_string(),
            create_time: "1234567890".to_string(),
            title: "会议标题".to_string(),
            cover: "cover_url".to_string(),
            duration: "3600000".to_string(),
            url: "https://example.com".to_string(),
        };
        let response = GetMinuteResponse { minute: info };

        assert_eq!(response.minute.token, "token_123");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(GetMinuteResponse::data_format(), ResponseFormat::Data);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMinuteResponse {
    pub minute: MinuteInfo,
}

impl ApiResponseTrait for GetMinuteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinuteInfo {
    pub token: String,
    pub owner_id: String,
    /// 妙记创建时间 timestamp（ms 级别）
    pub create_time: String,
    pub title: String,
    pub cover: String,
    /// 妙记时长（ms 级别）
    pub duration: String,
    pub url: String,
}
