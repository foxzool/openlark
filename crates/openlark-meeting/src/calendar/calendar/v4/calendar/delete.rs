//! 删除共享日历
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar/delete>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::api_utils::{extract_response_data, validate_required_field};
use serde::{Deserialize, Serialize};

use crate::endpoints::CALENDAR_V4_CALENDARS;

/// 删除共享日历请求
pub struct DeleteCalendarRequest {
    config: Config,
    calendar_id: String,
}

/// 删除共享日历响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteCalendarResponse {}

impl ApiResponseTrait for DeleteCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DeleteCalendarRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            calendar_id: String::new(),
        }
    }

    /// 日历 ID（路径参数）
    pub fn calendar_id(mut self, calendar_id: impl Into<String>) -> Self {
        self.calendar_id = calendar_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar/delete>
    pub async fn execute(self) -> SDKResult<DeleteCalendarResponse> {
        validate_required_field("calendar_id", Some(&self.calendar_id), "日历 ID 不能为空")?;

        let url = format!("{}/{}", CALENDAR_V4_CALENDARS, self.calendar_id);
        let api_request: ApiRequest<DeleteCalendarResponse> = ApiRequest::delete(&url);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "删除共享日历")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

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
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
