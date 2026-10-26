//! 获取消息推送概览

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取消息推送概览的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMessagePushOverviewBody {
    /// 查询日期，格式 yyyy-mm-dd（cycle_type=2 须为周一；=3 须为每月 1 号）。
    pub date: String,
    /// 活跃周期统计类型：1-日活 / 2-周活 / 3-月活。
    pub cycle_type: i32,
    /// 需要查询的部门 id（为空返回租户数据；id 类型由 department_id_type 决定）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
}

impl GetMessagePushOverviewBody {
    fn validate(&self) -> SDKResult<()> {
        validate_required!(self.date, "date 不能为空");
        Ok(())
    }
}

/// 获取消息推送概览的请求。
#[derive(Debug, Clone)]
pub struct GetMessagePushOverviewRequest {
    config: Arc<Config>,
    app_id: String,
    /// 查询参数 `department_id_type`（可选，部门 ID 类型，默认 open_department_id）。
    department_id_type: Option<String>,
    body: GetMessagePushOverviewBody,
}

/// 获取消息推送概览的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMessagePushOverviewResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetMessagePushOverviewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetMessagePushOverviewRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
            department_id_type: None,
            body: GetMessagePushOverviewBody::default(),
        }
    }

    /// 设置部门 ID 类型（可选，默认 open_department_id）。
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.department_id_type = Some(department_id_type.into());
        self
    }

    /// 设置查询日期（必填，格式 yyyy-mm-dd）。
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.body.date = date.into();
        self
    }

    /// 设置活跃周期统计类型（必填：1-日活 / 2-周活 / 3-月活）。
    pub fn cycle_type(mut self, cycle_type: i32) -> Self {
        self.body.cycle_type = cycle_type;
        self
    }

    /// 设置需要查询的部门 id（可选，为空返回租户数据）。
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.body.department_id = Some(department_id.into());
        self
    }

    /// 执行获取消息推送概览请求。
    pub async fn execute(self) -> SDKResult<GetMessagePushOverviewResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetMessagePushOverviewResponse> {
        self.body.validate()?;
        let body = serde_json::to_value(&self.body)?;
        let path = format!(
            "/open-apis/application/v6/applications/{}/app_usage/message_push_overview",
            self.app_id
        );
        let req: ApiRequest<GetMessagePushOverviewResponse> = ApiRequest::post(&path)
            .query_opt("department_id_type", self.department_id_type)
            .body(body);

        Transport::request_typed(req, &self.config, Some(option), "获取消息推送概览").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：POST .../applications/{app_id}/app_usage/message_push_overview → 强类型响应解析（双层 data 信封）。
    #[tokio::test]
    async fn test_get_message_push_overview_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/application/v6/applications/cli_test_app/app_usage/message_push_overview",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "push_count": 128, "read_count": 96 } }
            })))
            .mount(&server)
            .await;

        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let resp = GetMessagePushOverviewRequest::new(config, "cli_test_app")
            .date("2021-11-18")
            .cycle_type(1)
            .department_id("od-bb009b4df5431400dc31d3bc8a37e069")
            .execute()
            .await
            .expect("获取消息推送概览应成功");
        let data = resp.data.expect("响应数据不应为空");
        assert_eq!(data["push_count"], 128);
        assert_eq!(data["read_count"], 96);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/application/v6/applications/cli_test_app/app_usage/message_push_overview"
        );
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["date"], "2021-11-18");
        assert_eq!(sent["cycle_type"], 1);
        assert_eq!(sent["department_id"], "od-bb009b4df5431400dc31d3bc8a37e069");
    }
}
