//! 查询日历列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar/list-2>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::api_utils::extract_response_data;
use serde::{Deserialize, Serialize};

use crate::endpoints::CALENDAR_V4_CALENDARS;

/// 查询日历列表请求
pub struct ListCalendarRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

/// 查询日历列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListCalendarResponse {
    /// 日历列表
    pub calendar: Vec<CalendarData>,
    /// 分页令牌
    pub page_token: Option<String>,
    /// 是否有更多数据
    pub has_more: Option<bool>,
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

impl ApiResponseTrait for ListCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListCalendarRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query_params: Vec::new(),
        }
    }

    /// 追加查询参数（用于覆盖文档中可选 query）
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar/list-2>
    pub async fn execute(self) -> SDKResult<ListCalendarResponse> {
        let mut api_request: ApiRequest<ListCalendarResponse> =
            ApiRequest::get(CALENDAR_V4_CALENDARS);

        for (key, value) in self.query_params {
            api_request = api_request.query(key, value);
        }

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "查询日历列表")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_calendar_request_builder() {
        let config = Config::default();
        let request = ListCalendarRequest::new(config)
            .query_param("page_size", "20")
            .query_param("page_token", "token123");

        assert_eq!(request.query_params.len(), 2);
        assert_eq!(request.query_params[0], ("page_size".to_string(), "20".to_string()));
        assert_eq!(request.query_params[1], ("page_token".to_string(), "token123".to_string()));
    }

    #[test]
    fn test_list_calendar_request_minimal() {
        let config = Config::default();
        let request = ListCalendarRequest::new(config);

        assert!(request.query_params.is_empty());
    }

    #[test]
    fn test_list_calendar_request_single_param() {
        let config = Config::default();
        let request = ListCalendarRequest::new(config)
            .query_param("user_id_type", "union_id");

        assert_eq!(request.query_params.len(), 1);
        assert_eq!(request.query_params[0], ("user_id_type".to_string(), "union_id".to_string()));
    }
}
