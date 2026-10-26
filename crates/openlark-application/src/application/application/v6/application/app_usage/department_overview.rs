//! 获取多部门应用使用概览

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

/// 获取多部门应用使用概览的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetApplicationUsageDepartmentOverviewBody {
    /// 查询日期，格式 yyyy-mm-dd（cycle_type=2 须为周一；=3 须为每月 1 号）。
    pub date: String,
    /// 活跃周期统计类型：1-日活 / 2-周活 / 3-月活。
    pub cycle_type: i32,
    /// 查询的部门 id（为空返回租户数据；id 类型由 department_id_type 决定）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 是否查询多层子部门数据（0～4，仅 department_id 传递时有效）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursion: Option<i32>,
    /// 分页大小（1～20，默认 10）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记，首次请求不填。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl GetApplicationUsageDepartmentOverviewBody {
    fn validate(&self) -> SDKResult<()> {
        validate_required!(self.date, "date 不能为空");
        Ok(())
    }
}

/// 获取多部门应用使用概览的请求。
#[derive(Debug, Clone)]
pub struct GetApplicationUsageDepartmentOverviewRequest {
    config: Arc<Config>,
    app_id: String,
    /// 查询参数 `department_id_type`（可选，部门 ID 类型，默认 open_department_id）。
    department_id_type: Option<String>,
    body: GetApplicationUsageDepartmentOverviewBody,
}

/// 获取多部门应用使用概览的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetApplicationUsageDepartmentOverviewResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetApplicationUsageDepartmentOverviewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetApplicationUsageDepartmentOverviewRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
            department_id_type: None,
            body: GetApplicationUsageDepartmentOverviewBody::default(),
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

    /// 设置子部门递归层级（可选，0～4，仅 department_id 传递时有效）。
    pub fn recursion(mut self, recursion: i32) -> Self {
        self.body.recursion = Some(recursion);
        self
    }

    /// 设置分页大小（可选，1～20，默认 10）。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.body.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（可选，首次请求不填）。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.body.page_token = Some(page_token.into());
        self
    }

    /// 执行获取多部门应用使用概览请求。
    pub async fn execute(self) -> SDKResult<GetApplicationUsageDepartmentOverviewResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetApplicationUsageDepartmentOverviewResponse> {
        self.body.validate()?;
        let body = serde_json::to_value(&self.body)?;
        let path = format!(
            "/open-apis/application/v6/applications/{}/app_usage/department_overview",
            self.app_id
        );
        let req: ApiRequest<GetApplicationUsageDepartmentOverviewResponse> =
            ApiRequest::post(&path)
                .query_opt("department_id_type", self.department_id_type)
                .body(body);

        Transport::request_typed(req, &self.config, Some(option), "获取多部门应用使用概览").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：POST .../applications/{app_id}/app_usage/department_overview → 强类型响应解析（双层 data 信封）。
    #[tokio::test]
    async fn test_get_application_usage_department_overview_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/application/v6/applications/cli_test_app/app_usage/department_overview",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "department_id": "od_test", "active_users": 42 } }
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

        let resp = GetApplicationUsageDepartmentOverviewRequest::new(config, "cli_test_app")
            .date("2021-10-11")
            .cycle_type(1)
            .department_id("dfd2f9g16dd75ggd")
            .recursion(2)
            .page_size(10)
            .execute()
            .await
            .expect("获取多部门应用使用概览应成功");
        let data = resp.data.expect("响应数据不应为空");
        assert_eq!(data["department_id"], "od_test");
        assert_eq!(data["active_users"], 42);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/application/v6/applications/cli_test_app/app_usage/department_overview"
        );
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["date"], "2021-10-11");
        assert_eq!(sent["cycle_type"], 1);
        assert_eq!(sent["department_id"], "dfd2f9g16dd75ggd");
        assert_eq!(sent["recursion"], 2);
        assert_eq!(sent["page_size"], 10);
    }
}
