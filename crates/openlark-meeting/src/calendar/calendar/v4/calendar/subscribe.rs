//! 订阅日历
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar/subscribe>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

use crate::{common::api_utils::extract_response_data, endpoints::CALENDAR_V4_CALENDARS};

/// 订阅日历请求
pub struct SubscribeCalendarRequest {
    config: Config,
    calendar_id: String,
}

impl SubscribeCalendarRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar/subscribe>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.calendar_id, "calendar_id 不能为空");

        // url: POST:/open-apis/calendar/v4/calendars/:calendar_id/subscribe
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(format!(
            "{}/{}/subscribe",
            CALENDAR_V4_CALENDARS, self.calendar_id
        ));

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "订阅日历")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../calendars/{calendar_id}/subscribe → 裸 Value 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_subscribe_calendar_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/calendar/v4/calendars/cal_001/subscribe"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "subscribed": true }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = SubscribeCalendarRequest::new(config)
            .calendar_id("cal_001")
            .execute()
            .await
            .expect("订阅日历应成功");
        assert_eq!(resp["subscribed"], json!(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/calendar/v4/calendars/cal_001/subscribe"
        );
    }
}
