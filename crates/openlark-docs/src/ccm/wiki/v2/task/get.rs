//! 获取任务结果
//!
//! 该方法用于获取 wiki 异步任务的结果。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/wiki-v2/task/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use super::super::models::WikiTask;
use crate::common::{api_endpoints::WikiApiV2, api_utils::*};

/// 获取任务结果请求
///
/// 用于查询 wiki 异步任务结果。
pub struct GetWikiTaskRequest {
    task_id: String,
    task_type: Option<String>,
    config: Config,
}

/// 获取任务结果响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWikiTaskResponse {
    /// 任务信息
    pub task: Option<WikiTask>,
}

impl ApiResponseTrait for GetWikiTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetWikiTaskRequest {
    /// 创建获取任务结果请求
    pub fn new(config: Config) -> Self {
        Self {
            task_id: String::new(),
            task_type: None,
            config,
        }
    }

    /// 设置任务ID
    pub fn task_id(mut self, task_id: impl Into<String>) -> Self {
        self.task_id = task_id.into();
        self
    }

    /// 设置任务类型（可选，例如 move）
    pub fn task_type(mut self, task_type: impl Into<String>) -> Self {
        self.task_type = Some(task_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetWikiTaskResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetWikiTaskResponse> {
        // 验证必填字段
        validate_required!(self.task_id, "任务ID不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint = WikiApiV2::TaskGet(self.task_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<GetWikiTaskResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        if let Some(task_type) = self.task_type {
            api_request = api_request.query("task_type", &task_type);
        }

        // 发送请求

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/wiki/v2/tasks/{task_id} → GetWikiTaskResponse。
    #[tokio::test]
    async fn test_get_wiki_task_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/wiki/v2/tasks/tk001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {}
            })))
            .mount(&server)
            .await;
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();
        let resp = GetWikiTaskRequest::new(config)
            .task_id("tk001")
            .execute()
            .await
            .expect("获取任务结果应成功");
        assert!(resp.task.is_none());
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/wiki/v2/tasks/tk001");
    }
}
