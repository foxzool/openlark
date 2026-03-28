//! 创建共享日历
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::endpoints::CALENDAR_V4_CALENDARS;

/// 创建共享日历请求
pub struct CreateCalendarRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

/// 创建共享日历响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateCalendarResponse {
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

impl ApiResponseTrait for CreateCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateCalendarRequest {
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
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar/create>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<CreateCalendarResponse> {
        self.execute_with_options(body, RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<CreateCalendarResponse> {
        let api_request: ApiRequest<serde_json::Value> =
            ApiRequest::post(CALENDAR_V4_CALENDARS).body(body);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        let data: CreateCalendarResponse =
            serde_json::from_value(response.data.unwrap_or_default())?;
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_calendar_request_builder() {
        let config = Config::default();
        let request = CreateCalendarRequest::new(config)
            .query_param("user_id_type", "open_id");

        assert_eq!(request.query_params.len(), 1);
        assert_eq!(request.query_params[0], ("user_id_type".to_string(), "open_id".to_string()));
    }

    #[test]
    fn test_create_calendar_request_minimal() {
        let config = Config::default();
        let request = CreateCalendarRequest::new(config);

        assert!(request.query_params.is_empty());
    }

    #[test]
    fn test_create_calendar_request_multiple_params() {
        let config = Config::default();
        let request = CreateCalendarRequest::new(config)
            .query_param("key1", "value1")
            .query_param("key2", "value2");

        assert_eq!(request.query_params.len(), 2);
        assert_eq!(request.query_params[0], ("key1".to_string(), "value1".to_string()));
        assert_eq!(request.query_params[1], ("key2".to_string(), "value2".to_string()));
    }
}
