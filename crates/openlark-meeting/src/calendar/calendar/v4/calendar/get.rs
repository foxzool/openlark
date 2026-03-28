//! 查询日历信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

use crate::common::api_utils::{extract_response_data, validate_required_field};
use serde::{Deserialize, Serialize};

use crate::endpoints::CALENDAR_V4_CALENDARS;

/// 查询日历信息请求
pub struct GetCalendarRequest {
    config: Config,
    calendar_id: String,
    query_params: Vec<(String, String)>,
}

/// 查询日历信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetCalendarResponse {
    pub calendar: CalendarData,
}

/// 日历数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalendarData {
    /// 日历 ID
    pub calendar_id: String,
    /// 日历摘要
    pub summary: String,
    /// 日历描述
    pub description: Option<String>,
    /// 日历颜色
    pub color: Option<String>,
    /// 权限
    pub permissions: Option<CalendarPermissions>,
    /// 是否为主日历
    pub primary: Option<bool>,
    /// 日历类型
    pub calendar_type: Option<String>,
}

/// 日历权限
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalendarPermissions {
    /// 是否可读
    pub is_readable: Option<bool>,
    /// 是否可写
    pub is_writable: Option<bool>,
}

impl ApiResponseTrait for GetCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetCalendarRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            calendar_id: String::new(),
            query_params: Vec::new(),
        }
    }

    /// 日历 ID（路径参数）
    pub fn calendar_id(mut self, calendar_id: impl Into<String>) -> Self {
        self.calendar_id = calendar_id.into();
        self
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar/get>
    pub async fn execute(self) -> SDKResult<GetCalendarResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<GetCalendarResponse> {
        validate_required_field("calendar_id", Some(&self.calendar_id), "日历 ID 不能为空")?;

        let url = format!("{}/{}", CALENDAR_V4_CALENDARS, self.calendar_id);
        let mut api_request: ApiRequest<GetCalendarResponse> = ApiRequest::get(&url);

        for (key, value) in self.query_params {
            api_request = api_request.query(key, value);
        }

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "查询日历信息")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_calendar_request_builder() {
        let config = Config::default();
        let request = GetCalendarRequest::new(config)
            .calendar_id("cal_123")
            .query_param("user_id_type", "open_id");

        assert_eq!(request.calendar_id, "cal_123");
        assert_eq!(request.query_params.len(), 1);
        assert_eq!(request.query_params[0], ("user_id_type".to_string(), "open_id".to_string()));
    }

    #[test]
    fn test_get_calendar_request_minimal() {
        let config = Config::default();
        let request = GetCalendarRequest::new(config);

        assert!(request.calendar_id.is_empty());
        assert!(request.query_params.is_empty());
    }

    #[test]
    fn test_get_calendar_request_only_id() {
        let config = Config::default();
        let request = GetCalendarRequest::new(config)
            .calendar_id("cal_abc");

        assert_eq!(request.calendar_id, "cal_abc");
        assert!(request.query_params.is_empty());
    }

    #[test]
    fn test_get_calendar_request_multiple_params() {
        let config = Config::default();
        let request = GetCalendarRequest::new(config)
            .calendar_id("cal_xyz")
            .query_param("key1", "value1")
            .query_param("key2", "value2");

        assert_eq!(request.calendar_id, "cal_xyz");
        assert_eq!(request.query_params.len(), 2);
    }
}
