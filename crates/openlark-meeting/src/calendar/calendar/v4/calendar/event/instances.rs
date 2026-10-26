//! 获取重复日程实例
//!
//! docPath: <https://open.feishu.cn/document/calendar-v4/calendar-event/instances>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 获取重复日程实例请求
pub struct InstancesCalendarEventRequest {
    config: Config,
    calendar_id: String,
    event_id: String,
    /// 查询参数
    query: InstancesCalendarEventQuery,
}

/// 获取重复日程实例查询参数
///
/// start_time/end_time 必填（区间不能超过 2 年）；page_size/page_token 可选。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstancesCalendarEventQuery {
    /// 开始时间（Unix 时间戳，单位秒），与 end_time 构成查询区间
    pub start_time: String,
    /// 结束时间（Unix 时间戳，单位秒），与 start_time 构成查询区间
    pub end_time: String,
    /// 一次调用返回的日程数量上限（默认 50，取值 10 ～ 500）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记，第一次请求不填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl InstancesCalendarEventRequest {
    /// 创建请求实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            calendar_id: String::new(),
            event_id: String::new(),
            query: InstancesCalendarEventQuery {
                start_time: String::new(),
                end_time: String::new(),
                page_size: None,
                page_token: None,
            },
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

    /// 设置开始时间（Unix 时间戳，单位秒）
    pub fn start_time(mut self, start_time: impl Into<String>) -> Self {
        self.query.start_time = start_time.into();
        self
    }

    /// 设置结束时间（Unix 时间戳，单位秒）
    pub fn end_time(mut self, end_time: impl Into<String>) -> Self {
        self.query.end_time = end_time.into();
        self
    }

    /// 设置一次调用返回的日程数量上限（默认 50，取值 10 ～ 500）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.query.page_size = Some(page_size);
        self
    }

    /// 设置分页标记，第一次请求不填
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.query.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/calendar-v4/calendar-event/instances>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.calendar_id, "calendar_id 不能为空");
        validate_required!(self.event_id, "event_id 不能为空");
        validate_required!(self.query.start_time, "start_time 不能为空");
        validate_required!(self.query.end_time, "end_time 不能为空");

        // url: GET:/open-apis/calendar/v4/calendars/:calendar_id/events/:event_id/instances
        let q = self.query;
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(format!(
            "/open-apis/calendar/v4/calendars/{}/events/{}/instances",
            self.calendar_id, self.event_id
        ))
        .query("start_time", q.start_time)
        .query("end_time", q.end_time)
        .query_opt("page_size", q.page_size.map(|size| size.to_string()))
        .query_opt("page_token", q.page_token);

        Transport::request_typed(req, &self.config, Some(option), "获取重复日程实例").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../events/{event_id}/instances → 裸 Value 解析（单层 data 信封）+ 官方必填查询参数。
    #[tokio::test]
    async fn test_instances_calendar_event_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/calendar/v4/calendars/cal_001/events/evt_001/instances",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "items": [{ "event_id": "evt_001" }] }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = InstancesCalendarEventRequest::new(config)
            .calendar_id("cal_001")
            .event_id("evt_001")
            .start_time("1631777271")
            .end_time("1634455671")
            .page_size(20)
            .execute()
            .await
            .expect("获取重复日程实例应成功");
        assert_eq!(resp["items"][0]["event_id"], json!("evt_001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/calendar/v4/calendars/cal_001/events/evt_001/instances"
        );
        // 官方必填查询参数：start_time/end_time；可选：page_size
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("start_time=1631777271"));
        assert!(query.contains("end_time=1634455671"));
        assert!(query.contains("page_size=20"));
    }
}
