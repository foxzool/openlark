//! 取消订阅日程变更事件
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar-event/unsubscription>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::CALENDAR_V4_CALENDARS,
};

/// 取消订阅日程变更事件请求
pub struct UnsubscriptionCalendarEventRequest {
    config: Config,
    calendar_id: String,
}

impl UnsubscriptionCalendarEventRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar-event/unsubscription>
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

        // url: POST:/open-apis/calendar/v4/calendars/:calendar_id/events/unsubscription
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(format!(
            "{}/{}/events/unsubscription",
            CALENDAR_V4_CALENDARS, self.calendar_id
        ))
        .body(serialize_params(&body, "取消订阅日程变更事件")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "取消订阅日程变更事件")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../events/unsubscription → 裸 Value 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_unsubscription_calendar_event_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/calendar/v4/calendars/cal_001/events/unsubscription",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "unsubscribed": true }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = UnsubscriptionCalendarEventRequest::new(config)
            .calendar_id("cal_001")
            .execute(json!({ "subscription_id": "sub_001" }))
            .await
            .expect("取消订阅日程变更事件应成功");
        assert_eq!(resp["unsubscribed"], json!(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/calendar/v4/calendars/cal_001/events/unsubscription"
        );
    }
}
