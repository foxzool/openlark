//! 获取日程参与群成员列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar-event-attendee/list>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, validate_required,
    SDKResult,
};

use crate::{common::api_utils::extract_response_data, endpoints::CALENDAR_V4_CALENDARS};

/// 获取日程参与群成员列表请求
pub struct ListCalendarEventAttendeeChatMemberRequest {
    config: Config,
    calendar_id: String,
    event_id: String,
    attendee_id: String,
    query_params: Vec<(String, String)>,
}

impl ListCalendarEventAttendeeChatMemberRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            calendar_id: String::new(),
            event_id: String::new(),
            attendee_id: String::new(),
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

    /// 参与人 ID（路径参数）
    pub fn attendee_id(mut self, attendee_id: impl Into<String>) -> Self {
        self.attendee_id = attendee_id.into();
        self
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar-event-attendee/list>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.calendar_id, "calendar_id 不能为空");
        validate_required!(self.event_id, "event_id 不能为空");
        validate_required!(self.attendee_id, "attendee_id 不能为空");

        // url: GET:/open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/attendees/:attendee_id/chat_members
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(format!(
            "{}/{}/events/{}/attendees/{}/chat_members",
            CALENDAR_V4_CALENDARS, self.calendar_id, self.event_id, self.attendee_id
        ));
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取日程参与群成员列表")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = ListCalendarEventAttendeeChatMemberRequest::new(config.clone())
            .calendar_id("test".to_string())
            .event_id("test".to_string());
        let _ = request;
    }
}
