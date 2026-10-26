//! 查询日程视图
//!
//! docPath: <https://open.feishu.cn/document/calendar-v4/calendar-event/instance_view>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 查询日程视图请求
pub struct InstanceViewCalendarEventRequest {
    config: Config,
    calendar_id: String,
    /// 查询参数
    query: InstanceViewCalendarEventQuery,
}

/// 查询日程视图查询参数
///
/// start_time/end_time 必填（区间需小于 40 天）；user_id_type 可选。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceViewCalendarEventQuery {
    /// 开始时间（Unix 时间戳，单位秒），与 end_time 构成查询区间
    pub start_time: String,
    /// 结束时间（Unix 时间戳，单位秒），与 start_time 构成查询区间
    pub end_time: String,
    /// 用户 ID 类型：open_id / union_id / user_id（默认 open_id）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl InstanceViewCalendarEventRequest {
    /// 创建请求实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            calendar_id: String::new(),
            query: InstanceViewCalendarEventQuery {
                start_time: String::new(),
                end_time: String::new(),
                user_id_type: None,
            },
        }
    }

    /// 日历 ID（路径参数）
    pub fn calendar_id(mut self, calendar_id: impl Into<String>) -> Self {
        self.calendar_id = calendar_id.into();
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

    /// 设置用户 ID 类型：open_id / union_id / user_id
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.query.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/calendar-v4/calendar-event/instance_view>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.calendar_id, "calendar_id 不能为空");
        validate_required!(self.query.start_time, "start_time 不能为空");
        validate_required!(self.query.end_time, "end_time 不能为空");

        // url: GET:/open-apis/calendar/v4/calendars/:calendar_id/events/instance_view
        let q = self.query;
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(format!(
            "/open-apis/calendar/v4/calendars/{}/events/instance_view",
            self.calendar_id
        ))
        .query("start_time", q.start_time)
        .query("end_time", q.end_time)
        .query_opt("user_id_type", q.user_id_type);

        Transport::request_typed(req, &self.config, Some(option), "查询日程视图").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../events/instance_view → 裸 Value 解析（单层 data 信封）+ 官方必填查询参数。
    #[tokio::test]
    async fn test_instance_view_calendar_event_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/calendar/v4/calendars/cal_001/events/instance_view",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "events": [{ "event_id": "evt_001" }] }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = InstanceViewCalendarEventRequest::new(config)
            .calendar_id("cal_001")
            .start_time("1700000000")
            .end_time("1700345600")
            .execute()
            .await
            .expect("查询日程视图应成功");
        assert_eq!(resp["events"][0]["event_id"], json!("evt_001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/calendar/v4/calendars/cal_001/events/instance_view"
        );
        // 官方必填查询参数：start_time/end_time
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("start_time=1700000000"));
        assert!(query.contains("end_time=1700345600"));
    }
}
