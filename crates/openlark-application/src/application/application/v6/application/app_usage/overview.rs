//! 获取应用使用概览

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

/// 获取应用使用概览的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetApplicationUsageOverviewBody {
    /// 查询日期，格式 yyyy-mm-dd（cycle_type=2 须为周一；=3 须为每月 1 号）。
    pub date: String,
    /// 活跃周期统计类型：1-日活 / 2-周活 / 3-月活。
    pub cycle_type: i32,
    /// 查询的部门 id（为空返回租户数据；id 类型由 department_id_type 决定）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 能力类型：app-应用整体 / mp-小程序 / h5-网页 / bot-机器人。
    pub ability: String,
}

impl GetApplicationUsageOverviewBody {
    fn validate(&self) -> SDKResult<()> {
        validate_required!(self.date, "date 不能为空");
        validate_required!(self.ability, "ability 不能为空");
        Ok(())
    }
}

/// 获取应用使用概览的请求。
#[derive(Debug, Clone)]
pub struct GetApplicationUsageOverviewRequest {
    config: Arc<Config>,
    app_id: String,
    /// 查询参数 `department_id_type`（可选，部门 ID 类型，默认 open_department_id）。
    department_id_type: Option<String>,
    body: GetApplicationUsageOverviewBody,
}

/// 获取应用使用概览的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetApplicationUsageOverviewResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetApplicationUsageOverviewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetApplicationUsageOverviewRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
            department_id_type: None,
            body: GetApplicationUsageOverviewBody::default(),
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

    /// 设置查询的部门 id（可选，为空返回租户数据）。
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.body.department_id = Some(department_id.into());
        self
    }

    /// 设置能力类型（必填：app-应用整体 / mp-小程序 / h5-网页 / bot-机器人）。
    pub fn ability(mut self, ability: impl Into<String>) -> Self {
        self.body.ability = ability.into();
        self
    }

    /// 执行获取应用使用概览请求。
    pub async fn execute(self) -> SDKResult<GetApplicationUsageOverviewResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetApplicationUsageOverviewResponse> {
        self.body.validate()?;
        let body = serde_json::to_value(&self.body)?;
        let path = format!(
            "/open-apis/application/v6/applications/{}/app_usage/overview",
            self.app_id
        );
        let req: ApiRequest<GetApplicationUsageOverviewResponse> = ApiRequest::post(&path)
            .query_opt("department_id_type", self.department_id_type)
            .body(body);

        Transport::request_typed(req, &self.config, Some(option), "获取应用使用概览").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：POST .../applications/{app_id}/app_usage/overview → 强类型 GetApplicationUsageOverviewResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_get_application_usage_overview_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/application/v6/applications/cli_test_app/app_usage/overview",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "active_users": 256, "total_users": 1024 } }
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

        let resp = GetApplicationUsageOverviewRequest::new(config, "cli_test_app")
            .date("2021-07-26")
            .cycle_type(1)
            .department_id("od-bb009b4df5431400dc31d3bc8a37e069")
            .ability("app")
            .execute()
            .await
            .expect("获取应用使用概览应成功");
        let data = resp.data.expect("响应数据不应为空");
        assert_eq!(data["active_users"], 256);
        assert_eq!(data["total_users"], 1024);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/application/v6/applications/cli_test_app/app_usage/overview"
        );
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["date"], "2021-07-26");
        assert_eq!(sent["cycle_type"], 1);
        assert_eq!(sent["department_id"], "od-bb009b4df5431400dc31d3bc8a37e069");
        assert_eq!(sent["ability"], "app");
    }
}
