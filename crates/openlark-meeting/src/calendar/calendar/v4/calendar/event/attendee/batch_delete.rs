//! 删除日程参与人
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar-event-attendee/batch_delete>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::CALENDAR_V4_CALENDARS,
};

/// 删除日程参与人请求
pub struct BatchDeleteCalendarEventAttendeeRequest {
    config: Config,
    calendar_id: String,
    event_id: String,
}

impl BatchDeleteCalendarEventAttendeeRequest {
    /// 创建请求实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            calendar_id: String::new(),
            event_id: String::new(),
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

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar-event-attendee/batch_delete>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.calendar_id, "calendar_id 不能为空");
        validate_required!(self.event_id, "event_id 不能为空");

        // url: POST:/open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/attendees/batch_delete
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(format!(
            "{}/{}/events/{}/attendees/batch_delete",
            CALENDAR_V4_CALENDARS, self.calendar_id, self.event_id
        ))
        .body(serialize_params(&body, "删除日程参与人")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "删除日程参与人")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../events/{event_id}/attendees/batch_delete → 裸 Value 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_batch_delete_calendar_event_attendee_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/calendar/v4/calendars/cal_001/events/evt_001/attendees/batch_delete",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "deleted": true }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = BatchDeleteCalendarEventAttendeeRequest::new(config)
            .calendar_id("cal_001")
            .event_id("evt_001")
            .execute(json!({ "need_notification": false }))
            .await
            .expect("删除日程参与人应成功");
        assert_eq!(resp["deleted"], json!(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/calendar/v4/calendars/cal_001/events/evt_001/attendees/batch_delete"
        );
    }
}
