//! 删除访问控制
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar-acl/delete>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

use crate::endpoints::CALENDAR_V4_CALENDARS;

/// 删除访问控制请求
pub struct DeleteCalendarAclRequest {
    config: Config,
    calendar_id: String,
    acl_id: String,
}

impl DeleteCalendarAclRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            calendar_id: String::new(),
            acl_id: String::new(),
        }
    }

    /// 日历 ID（路径参数）
    pub fn calendar_id(mut self, calendar_id: impl Into<String>) -> Self {
        self.calendar_id = calendar_id.into();
        self
    }

    /// ACL ID（路径参数）
    pub fn acl_id(mut self, acl_id: impl Into<String>) -> Self {
        self.acl_id = acl_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/calendar-acl/delete>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.calendar_id, "calendar_id 不能为空");
        validate_required!(self.acl_id, "acl_id 不能为空");

        // url: DELETE:/open-apis/calendar/v4/calendars/:calendar_id/acls/:acl_id
        let req: ApiRequest<serde_json::Value> = ApiRequest::delete(format!(
            "{}/{}/acls/{}",
            CALENDAR_V4_CALENDARS, self.calendar_id, self.acl_id
        ));

        Transport::request_typed(req, &self.config, Some(option), "删除访问控制").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：DELETE .../calendars/{calendar_id}/acls/{acl_id} → 裸 Value 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_delete_calendar_acl_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/open-apis/calendar/v4/calendars/cal_001/acls/acl_001",
            ))
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

        let resp = DeleteCalendarAclRequest::new(config)
            .calendar_id("cal_001")
            .acl_id("acl_001")
            .execute()
            .await
            .expect("删除访问控制应成功");
        assert_eq!(resp["deleted"], json!(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/calendar/v4/calendars/cal_001/acls/acl_001"
        );
    }
}
