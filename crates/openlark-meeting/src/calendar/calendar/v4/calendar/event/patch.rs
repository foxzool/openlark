//! 更新日程
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar-event/patch>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

use crate::{common::api_endpoints::CalendarApiV4, common::api_utils::serialize_params};

/// 更新日程请求
pub struct PatchCalendarEventRequest {
    config: Config,
    calendar_id: String,
    event_id: String,
}

impl PatchCalendarEventRequest {
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
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar-event/patch>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.calendar_id, "calendar_id 不能为空");
        validate_required!(self.event_id, "event_id 不能为空");

        let api_endpoint =
            CalendarApiV4::EventPatch(self.calendar_id.clone(), self.event_id.clone());
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::patch(api_endpoint.to_url()).body(serialize_params(&body, "更新日程")?);

        Transport::request_typed(req, &self.config, Some(option), "更新日程").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：PATCH .../events/{event_id} → 裸 Value 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_patch_calendar_event_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path(
                "/open-apis/calendar/v4/calendars/cal_001/events/evt_001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "event_id": "evt_001" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = PatchCalendarEventRequest::new(config)
            .calendar_id("cal_001")
            .event_id("evt_001")
            .execute(json!({ "summary": "更新后的主题" }))
            .await
            .expect("更新日程应成功");
        assert_eq!(resp["event_id"], json!("evt_001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/calendar/v4/calendars/cal_001/events/evt_001"
        );
    }
}
