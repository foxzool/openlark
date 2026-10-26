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
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 催办人工任务 Builder
#[derive(Debug, Clone)]
pub struct ExpeditingRequestBuilder {
    config: Config,
    /// 任务 ID
    task_id: String,
    /// 请求体
    body: ExpeditingBody,
}

impl ExpeditingRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, task_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
            body: ExpeditingBody {
                operator_user_id: String::new(),
                expediting_user_ids: Vec::new(),
                opinion: None,
            },
        }
    }

    /// 设置操作人 kunlunUserID
    pub fn operator_user_id(mut self, operator_user_id: impl Into<String>) -> Self {
        self.body.operator_user_id = operator_user_id.into();
        self
    }

    /// 添加催办用户 ID
    pub fn expediting_user_id(mut self, expediting_user_id: impl Into<String>) -> Self {
        self.body
            .expediting_user_ids
            .push(expediting_user_id.into());
        self
    }

    /// 添加多个催办用户 ID（长度范围 1～20）
    pub fn expediting_user_ids(
        mut self,
        expediting_user_ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.body
            .expediting_user_ids
            .extend(expediting_user_ids.into_iter().map(Into::into));
        self
    }

    /// 设置催办理由
    pub fn opinion(mut self, opinion: impl Into<String>) -> Self {
        self.body.opinion = Some(opinion.into());
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
        validate_required!(self.body.operator_user_id, "操作人ID不能为空");
        validate_required_list!(
            self.body.expediting_user_ids,
            20,
            "催办用户ID列表不能为空且不能超过 20 个"
        );

        let url = format!("/open-apis/apaas/v1/user_tasks/{}/expediting", self.task_id);

        let req: ApiRequest<ExpeditingResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&self.body)?);
        Transport::request_typed(req, &self.config, Some(option), "Operation").await
    }
}

/// 催办请求体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExpeditingBody {
    /// 操作人 kunlunUserID，可通过 aPaaS 用户管理页面获取
    #[serde(rename = "operator_user_id")]
    pub operator_user_id: String,
    /// 催办人的 kunlunID 列表，可通过 aPaaS 用户管理页面获取（长度范围 1～20）
    #[serde(rename = "expediting_user_ids")]
    pub expediting_user_ids: Vec<String>,
    /// 催办理由
    #[serde(rename = "opinion", skip_serializing_if = "Option::is_none")]
    pub opinion: Option<String>,
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
            .operator_user_id("u_000")
            .expediting_user_ids(vec!["u_001".to_string(), "u_002".to_string()])
            .opinion("请尽快处理")
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
        // 请求体须与官方文档一致：operator_user_id + expediting_user_ids + 可选 opinion
        let sent: serde_json::Value = received[0].body_json().expect("请求体应为 JSON");
        assert_eq!(
            sent,
            json!({
                "operator_user_id": "u_000",
                "expediting_user_ids": ["u_001", "u_002"],
                "opinion": "请尽快处理"
            })
        );
    }
}
