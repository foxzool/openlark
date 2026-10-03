//! 获取日程
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar-event/get>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

use crate::common::api_endpoints::CalendarApiV4;

/// 获取日程请求
pub struct GetCalendarEventRequest {
    config: Config,
    calendar_id: String,
    event_id: String,
    query_params: Vec<(String, String)>,
}

impl GetCalendarEventRequest {
    /// 创建请求实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            calendar_id: String::new(),
            event_id: String::new(),
            query_params: Vec::new(),
        }
    }

    /// 日历 ID（路径参数）
    pub fn calendar_id(mut self, calendar_id: impl Into<String>) -> Self {
        self.calendar_id = calendar_id.into();
        self
    }

    /// 日程 ID（路径参数）
    pub fn event_id(mut self, event_id: impl Into<String>) -> Self {
        self.event_id = event_id.into();
        self
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar-event/get>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.calendar_id, "calendar_id 不能为空");
        validate_required!(self.event_id, "event_id 不能为空");

        // url: GET:/open-apis/calendar/v4/calendars/:calendar_id/events/:event_id
        let api_endpoint = CalendarApiV4::EventGet(self.calendar_id, self.event_id);
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(api_endpoint.to_url());
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        Transport::request_typed(req, &self.config, Some(option), "获取日程").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../events/{event_id} → 裸 Value 解析（单层 data 信封，带 user_id_type 查询参数）。
    #[tokio::test]
    async fn test_get_calendar_event_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/calendar/v4/calendars/cal_001/events/evt_001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "event_id": "evt_001", "summary": "周会" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = GetCalendarEventRequest::new(config)
            .calendar_id("cal_001")
            .event_id("evt_001")
            .query_param("user_id_type", "open_id")
            .execute()
            .await
            .expect("获取日程应成功");
        assert_eq!(resp["event_id"], json!("evt_001"));
        assert_eq!(resp["summary"], json!("周会"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/calendar/v4/calendars/cal_001/events/evt_001"
        );
        assert_eq!(received[0].url.query(), Some("user_id_type=open_id"));
    }
}
