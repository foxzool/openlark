//! 查询任务
//! docPath: <https://open.feishu.cn/document/server-docs/report-v1/task/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 查询任务请求。
#[derive(Debug, Clone)]
pub struct QueryReportTaskRequest {
    config: Arc<Config>,
}

/// 查询任务响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryReportTaskResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for QueryReportTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl QueryReportTaskRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行查询任务请求。
    pub async fn execute(self) -> SDKResult<QueryReportTaskResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行查询任务请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<QueryReportTaskResponse> {
        let path = "/open-apis/report/v1/tasks/query".to_string();
        let req: ApiRequest<QueryReportTaskResponse> = ApiRequest::post(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("查询任务", "响应数据为空"))
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/report/v1/tasks/query → 响应解析。
    #[tokio::test]
    async fn test_query_report_task_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/report/v1/tasks/query"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": {} }
            })))
            .mount(&server)
            .await;

        let config = std::sync::Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let resp = QueryReportTaskRequest::new(config)
            .execute()
            .await
            .expect("查询任务应成功");
        assert!(resp.data.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/report/v1/tasks/query");
    }
}
