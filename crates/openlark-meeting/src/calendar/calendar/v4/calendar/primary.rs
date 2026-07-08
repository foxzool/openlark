//! 查询主日历信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar/primary>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use crate::common::{api_endpoints::CalendarApiV4, api_utils::extract_response_data};

/// 查询主日历信息请求
pub struct PrimaryCalendarRequest {
    config: Config,
}

impl PrimaryCalendarRequest {
    /// 创建请求实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar/primary>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/calendar/v4/calendars/primary
        let api_endpoint = CalendarApiV4::CalendarPrimaryGet;
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(api_endpoint.to_url());

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询主日历信息")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../calendars/primary → 裸 Value 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_primary_calendar_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/calendar/v4/calendars/primary"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "primary_calendar_id": "cal_primary_001" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = PrimaryCalendarRequest::new(config)
            .execute()
            .await
            .expect("查询主日历信息应成功");
        assert_eq!(resp["primary_calendar_id"], json!("cal_primary_001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/calendar/v4/calendars/primary"
        );
    }
}
