//! 更新日历信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};

use crate::common::api_utils::validate_required_field;
use serde::{Deserialize, Serialize};

use crate::endpoints::CALENDAR_V4_CALENDARS;

/// 更新日历信息请求
pub struct PatchCalendarRequest {
    config: Config,
    calendar_id: String,
}

/// 更新日历信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchCalendarResponse {
    /// 日历。
    pub calendar: CalendarData,
}

impl ApiResponseTrait for PatchCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
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

impl PatchCalendarRequest {
    /// 创建请求实例。
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
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar/patch>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<PatchCalendarResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<PatchCalendarResponse> {
        validate_required_field("calendar_id", Some(&self.calendar_id), "日历 ID 不能为空")?;

        let url = format!("{}/{}", CALENDAR_V4_CALENDARS, self.calendar_id);
        let api_request: ApiRequest<PatchCalendarResponse> = ApiRequest::patch(&url).body(body);

        Transport::request_typed(api_request, &self.config, Some(option), "更新日历信息").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：PATCH .../calendars/{calendar_id} → PatchCalendarResponse（强类型无 inner data，单层 resp.calendar）。
    #[tokio::test]
    async fn test_patch_calendar_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/calendar/v4/calendars/cal_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "calendar": {
                        "calendar_id": "cal_001",
                        "summary": "更新后的日历",
                        "description": "新描述",
                        "color": "-1",
                        "permissions": { "is_readable": true, "is_writable": true },
                        "primary": false,
                        "calendar_type": "primary"
                    }
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = PatchCalendarRequest::new(config)
            .calendar_id("cal_001")
            .execute(json!({ "summary": "更新后的日历" }))
            .await
            .expect("更新日历信息应成功");
        assert_eq!(resp.calendar.calendar_id, "cal_001");
        assert_eq!(resp.calendar.summary, "更新后的日历");
        assert_eq!(resp.calendar.description.as_deref(), Some("新描述"));
        assert_eq!(resp.calendar.primary, Some(false));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/calendar/v4/calendars/cal_001"
        );
        assert_eq!(received[0].method, "PATCH");
    }
}
