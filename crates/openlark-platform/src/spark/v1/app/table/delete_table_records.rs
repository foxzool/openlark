//! 删除数据表中的记录
//!
//! docPath:

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use std::sync::Arc;

/// 删除数据表中的记录请求。
#[derive(Debug, Clone)]
pub struct AppTableDeleteTableRecordsRequest {
    config: Arc<Config>,
    app_id: String,
    table_name: String,
    /// 筛选条件，遵循 PostgREST 语法（如 age=gt.10）
    filter: String,
    /// 访问的 database 环境（online 线上环境默认值 / dev 开发环境）
    env: Option<String>,
}

impl AppTableDeleteTableRecordsRequest {
    /// 创建请求。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            app_id: String::new(),
            table_name: String::new(),
            filter: String::new(),
            env: None,
        }
    }

    /// 设置路径参数 `app_id`。
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = app_id.into();
        self
    }

    /// 设置路径参数 `table_name`。
    pub fn table_name(mut self, table_name: impl Into<String>) -> Self {
        self.table_name = table_name.into();
        self
    }

    /// 设置查询参数 `filter`（筛选条件，遵循 PostgREST 语法，必填）。
    pub fn filter(mut self, filter: impl Into<String>) -> Self {
        self.filter = filter.into();
        self
    }

    /// 设置查询参数 `env`（访问的 database 环境：online / dev，默认 online）。
    pub fn env(mut self, env: impl Into<String>) -> Self {
        self.env = Some(env.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.app_id, "app_id 不能为空");
        validate_required!(self.table_name, "table_name 不能为空");
        validate_required!(self.filter, "filter 不能为空");
        let path = format!(
            "/open-apis/spark/v1/apps/{}/tables/{}/records",
            self.app_id, self.table_name
        );
        let req: ApiRequest<serde_json::Value> = ApiRequest::delete(path)
            .query("filter", self.filter)
            .query_opt("env", self.env);
        Transport::request_typed(req, &self.config, Some(option), "删除数据表中的记录").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = AppTableDeleteTableRecordsRequest::new(config);
    }

    /// 端到端：DELETE .../spark/v1/apps/{app_id}/tables/{table_name}/records →
    /// 必填查询参数 filter 按官方文档发送。
    #[tokio::test]
    async fn test_delete_table_records_sends_filter_query_param() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/open-apis/spark/v1/apps/app_4jcn5n11bpf5v/tables/student_table/records",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {}
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

        let resp = AppTableDeleteTableRecordsRequest::new(config)
            .app_id("app_4jcn5n11bpf5v")
            .table_name("student_table")
            .filter("age=gt.10")
            .execute()
            .await
            .expect("删除数据表中的记录应成功");
        assert_eq!(resp, json!({}));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].method, "DELETE");
        let query = received[0].url.query().unwrap_or("");
        // 官方必填查询参数：filter（PostgREST 语法）
        assert!(query.contains("filter=age%3Dgt.10"));
    }
}
