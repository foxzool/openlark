//! 搜索日历
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar/search>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use crate::{common::api_endpoints::CalendarApiV4, common::api_utils::serialize_params};

/// 搜索日历请求
pub struct SearchCalendarRequest {
    config: Config,
}

impl SearchCalendarRequest {
    /// 创建请求实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar/search>
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
        let api_endpoint = CalendarApiV4::CalendarSearch;
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(api_endpoint.to_url()).body(serialize_params(&body, "搜索日历")?);

        Transport::request_typed(req, &self.config, Some(option), "搜索日历").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../calendars/search → 裸 Value 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_search_calendar_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/calendar/v4/calendars/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "calendars": [
                        { "calendar_id": "cal_001", "summary": "团队日历" }
                    ]
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

        let resp = SearchCalendarRequest::new(config)
            .execute(json!({ "query": "团队", "page_size": 20 }))
            .await
            .expect("搜索日历应成功");
        assert_eq!(resp["calendars"][0]["calendar_id"], json!("cal_001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/calendar/v4/calendars/search"
        );
    }
}
