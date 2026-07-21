//! 催办人工任务
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/flow/user-task/expediting>
//! docPath: <https://open.feishu.cn/document/apaas-v1/flow/user-task/expediting>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 催办人工任务 Builder
#[derive(Debug, Clone)]
pub struct ExpeditingRequestBuilder {
    config: Config,
    /// 任务 ID
    task_id: String,
    /// 用户 ID
    user_ids: Vec<String>,
}

impl ExpeditingRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, task_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
            user_ids: Vec::new(),
        }
    }

    /// 添加用户 ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_ids.push(user_id.into());
        self
    }

    /// 添加多个用户 ID
    pub fn user_ids(mut self, user_ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.user_ids.extend(user_ids.into_iter().map(Into::into));
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ExpeditingResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ExpeditingResponse> {
        let url = format!("/open-apis/apaas/v1/user_tasks/{}/expediting", self.task_id);

        let request = ExpeditingRequest {
            user_ids: self.user_ids,
        };

        let req: ApiRequest<ExpeditingResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        Transport::request_typed(req, &self.config, Some(option), "Operation").await
    }
}

/// 催办请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct ExpeditingRequest {
    /// 用户 ID 列表
    #[serde(rename = "user_ids")]
    user_ids: Vec<String>,
}

/// 催办响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExpeditingResponse {
    /// 任务 ID
    #[serde(rename = "task_id")]
    pub task_id: String,
    /// 结果消息
    #[serde(rename = "message")]
    pub message: String,
}

impl ApiResponseTrait for ExpeditingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to ExpeditingRequestBuilder, will be removed in v1.0 (#271)")]
pub type ExpeditingBuilder = ExpeditingRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../user_tasks/{id}/expediting → 强类型 ExpeditingResponse。
    #[tokio::test]
    async fn test_expediting_user_task_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/apaas/v1/user_tasks/task_001/expediting"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "task_id": "task_001",
                    "message": "已催办"
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = ExpeditingRequestBuilder::new(config, "task_001")
            .user_ids(vec!["u_001".to_string(), "u_002".to_string()])
            .execute()
            .await
            .expect("催办人工任务应成功");
        assert_eq!(resp.task_id, "task_001");
        assert_eq!(resp.message, "已催办");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/user_tasks/task_001/expediting"
        );
    }
}
