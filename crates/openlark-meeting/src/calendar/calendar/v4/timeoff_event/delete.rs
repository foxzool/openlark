//! 删除请假日程
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/timeoff_event/delete>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

/// 删除请假日程请求
pub struct DeleteTimeoffEventRequest {
    config: Config,
    timeoff_event_id: String,
}

impl DeleteTimeoffEventRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            timeoff_event_id: String::new(),
        }
    }

    /// 请假日程 ID（路径参数）
    pub fn timeoff_event_id(mut self, timeoff_event_id: impl Into<String>) -> Self {
        self.timeoff_event_id = timeoff_event_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/timeoff_event/delete>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.timeoff_event_id, "timeoff_event_id 不能为空");

        // url: DELETE:/open-apis/calendar/v4/timeoff_events/:timeoff_event_id
        let req: ApiRequest<serde_json::Value> = ApiRequest::delete(format!(
            "/open-apis/calendar/v4/timeoff_events/{}",
            self.timeoff_event_id
        ));

        Transport::request_typed(req, &self.config, Some(option), "删除请假日程").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：DELETE .../calendar/v4/timeoff_events/{id} → 裸 Value 解析（单层 resp["deleted"]）。
    #[tokio::test]
    async fn test_delete_timeoff_event_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/calendar/v4/timeoff_events/event_001"))
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

        let resp = DeleteTimeoffEventRequest::new(config)
            .timeoff_event_id("event_001")
            .execute()
            .await
            .expect("删除请假日程应成功");
        assert_eq!(resp["deleted"], json!(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/calendar/v4/timeoff_events/event_001"
        );
        assert_eq!(received[0].method, "DELETE");
    }
}
