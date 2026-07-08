//! 查询导出任务结果
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/export/get>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::extract_response_data;

/// 查询导出任务结果请求
pub struct GetExportTaskRequest {
    config: Config,
    task_id: String,
    query_params: Vec<(String, String)>,
}

impl GetExportTaskRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            task_id: String::new(),
            query_params: Vec::new(),
        }
    }

    /// 任务 ID（路径参数）
    pub fn task_id(mut self, task_id: impl Into<String>) -> Self {
        self.task_id = task_id.into();
        self
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/export/get>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.task_id, "task_id 不能为空");

        // url: GET:/open-apis/vc/v1/exports/:task_id
        let api_endpoint = VcApiV1::ExportGet(self.task_id.clone());
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(api_endpoint.to_url());
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询导出任务结果")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../exports/{task_id} + query 拼装 + 裸 Value 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_get_export_task_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/vc/v1/exports/task_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "task_id": "task_001", "status": "done", "file_url": "https://files.example.com/export.zip" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = GetExportTaskRequest::new(config)
            .task_id("task_001")
            .query_param("need_file_url", "true")
            .execute()
            .await
            .expect("查询导出任务结果应成功");
        assert_eq!(resp["task_id"], "task_001");
        assert_eq!(resp["status"], "done");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/vc/v1/exports/task_001");
        assert!(
            received[0]
                .url
                .query()
                .unwrap_or("")
                .contains("need_file_url=true")
        );
    }
}
