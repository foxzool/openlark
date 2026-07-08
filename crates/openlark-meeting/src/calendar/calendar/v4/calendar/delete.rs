//! 删除共享日历
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar/delete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};

use crate::common::api_utils::{extract_response_data, validate_required_field};
use serde::{Deserialize, Serialize};

use crate::endpoints::CALENDAR_V4_CALENDARS;

/// 删除共享日历请求
pub struct DeleteCalendarRequest {
    config: Config,
    calendar_id: String,
}

/// 删除共享日历响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteCalendarResponse {}

impl ApiResponseTrait for DeleteCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DeleteCalendarRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar/delete>
    pub async fn execute(self) -> SDKResult<DeleteCalendarResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteCalendarResponse> {
        validate_required_field("calendar_id", Some(&self.calendar_id), "日历 ID 不能为空")?;

        let url = format!("{}/{}", CALENDAR_V4_CALENDARS, self.calendar_id);
        let api_request: ApiRequest<DeleteCalendarResponse> = ApiRequest::delete(&url);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除共享日历")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：DELETE .../calendars/{calendar_id} → DeleteCalendarResponse（空 data 信封）。
    #[tokio::test]
    async fn test_delete_calendar_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/calendar/v4/calendars/cal_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {}
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let _resp = DeleteCalendarRequest::new(config)
            .calendar_id("cal_001")
            .execute()
            .await
            .expect("删除共享日历应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/calendar/v4/calendars/cal_001"
        );
        assert_eq!(received[0].method, "DELETE");
    }
}
