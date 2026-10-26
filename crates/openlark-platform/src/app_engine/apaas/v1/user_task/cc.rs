//! 抄送人工任务
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/flow/user-task/cc>
//! docPath: <https://open.feishu.cn/document/apaas-v1/flow/user-task/cc>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 抄送人工任务 Builder
#[derive(Debug, Clone)]
pub struct CcTaskRequestBuilder {
    config: Config,
    /// 任务 ID
    task_id: String,
    /// 请求体
    body: CcTaskBody,
}

impl CcTaskRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, task_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
            body: CcTaskBody {
                cc_user_ids: Vec::new(),
                operator_user_id: String::new(),
            },
        }
    }

    /// 添加抄送用户 ID
    pub fn cc_user_id(mut self, cc_user_id: impl Into<String>) -> Self {
        self.body.cc_user_ids.push(cc_user_id.into());
        self
    }

    /// 添加多个抄送用户 ID（长度范围 1～10）
    pub fn cc_user_ids(mut self, cc_user_ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.body
            .cc_user_ids
            .extend(cc_user_ids.into_iter().map(Into::into));
        self
    }

    /// 设置操作人 kunlunUserID
    pub fn operator_user_id(mut self, operator_user_id: impl Into<String>) -> Self {
        self.body.operator_user_id = operator_user_id.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CcTaskResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<CcTaskResponse> {
        validate_required!(self.body.operator_user_id, "操作人ID不能为空");
        validate_required_list!(
            self.body.cc_user_ids,
            10,
            "抄送用户ID列表不能为空且不能超过 10 个"
        );

        let url = format!("/open-apis/apaas/v1/user_tasks/{}/cc", self.task_id);

        let req: ApiRequest<CcTaskResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&self.body)?);
        Transport::request_typed(req, &self.config, Some(option), "Operation").await
    }
}

/// 抄送请求体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CcTaskBody {
    /// 抄送人的 kunlunID 列表，可通过 aPaaS 用户管理页面获取（长度范围 1～10）
    #[serde(rename = "cc_user_ids")]
    pub cc_user_ids: Vec<String>,
    /// 操作人 kunlunUserID，可通过 aPaaS 用户管理页面获取
    #[serde(rename = "operator_user_id")]
    pub operator_user_id: String,
}

/// 抄送响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CcTaskResponse {
    /// 任务 ID
    #[serde(rename = "task_id")]
    pub task_id: String,
    /// 抄送记录 ID
    #[serde(rename = "cc_id")]
    pub cc_id: String,
    /// 结果消息
    #[serde(rename = "message")]
    pub message: String,
}

impl ApiResponseTrait for CcTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to CcTaskRequestBuilder, will be removed in v1.0 (#271)")]
pub type CcTaskBuilder = CcTaskRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../user_tasks/{id}/cc → 强类型 CcTaskResponse。
    #[tokio::test]
    async fn test_cc_user_task_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/apaas/v1/user_tasks/task_001/cc"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "task_id": "task_001",
                    "cc_id": "cc_001",
                    "message": "抄送成功"
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

        let resp = CcTaskRequestBuilder::new(config, "task_001")
            .cc_user_ids(vec!["u_001".to_string(), "u_002".to_string()])
            .operator_user_id("u_000")
            .execute()
            .await
            .expect("抄送人工任务应成功");
        assert_eq!(resp.task_id, "task_001");
        assert_eq!(resp.cc_id, "cc_001");
        assert_eq!(resp.message, "抄送成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/user_tasks/task_001/cc"
        );
        // 请求体须与官方文档一致：cc_user_ids + operator_user_id
        let sent: serde_json::Value = received[0].body_json().expect("请求体应为 JSON");
        assert_eq!(
            sent,
            json!({
                "cc_user_ids": ["u_001", "u_002"],
                "operator_user_id": "u_000"
            })
        );
    }
}
