//! 基于人工任务发起群聊
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/flow/user-task/chat_group>
//! docPath: <https://open.feishu.cn/document/apaas-v1/flow/user-task/chat_group>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 基于人工任务发起群聊 Builder
#[derive(Debug, Clone)]
pub struct ChatGroupRequestBuilder {
    config: Config,
    /// 任务 ID
    task_id: String,
    /// 请求体
    body: ChatGroupBody,
}

impl ChatGroupRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, task_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
            body: ChatGroupBody {
                operator_user_id: String::new(),
                invite_user_ids: None,
                chat_id: None,
                chat_name: None,
            },
        }
    }

    /// 设置操作人 kunlunUserID
    pub fn operator_user_id(mut self, operator_user_id: impl Into<String>) -> Self {
        self.body.operator_user_id = operator_user_id.into();
        self
    }

    /// 添加邀请进群用户 kunlunID
    pub fn invite_user_id(mut self, invite_user_id: impl Into<String>) -> Self {
        self.body
            .invite_user_ids
            .get_or_insert_with(Vec::new)
            .push(invite_user_id.into());
        self
    }

    /// 添加多个邀请进群用户 kunlunID（长度范围 1～20）
    pub fn invite_user_ids(
        mut self,
        invite_user_ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.body
            .invite_user_ids
            .get_or_insert_with(Vec::new)
            .extend(invite_user_ids.into_iter().map(Into::into));
        self
    }

    /// 设置要拉入的群 openID（为空则新建群）
    pub fn chat_id(mut self, chat_id: impl Into<String>) -> Self {
        self.body.chat_id = Some(chat_id.into());
        self
    }

    /// 设置群名称（当 chat_id 为空时用该名称创建群聊）
    pub fn chat_name(mut self, chat_name: impl Into<String>) -> Self {
        self.body.chat_name = Some(chat_name.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ChatGroupResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<ChatGroupResponse> {
        validate_required!(self.body.operator_user_id, "操作人ID不能为空");

        let url = format!("/open-apis/apaas/v1/user_tasks/{}/chat_group", self.task_id);

        let req: ApiRequest<ChatGroupResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&self.body)?);
        Transport::request_typed(req, &self.config, Some(option), "Operation").await
    }
}

/// 发起群聊请求体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatGroupBody {
    /// 操作人 kunlunUserID，可通过 aPaaS 用户管理页面获取
    #[serde(rename = "operator_user_id")]
    pub operator_user_id: String,
    /// 邀请进群用户 kunlunID 列表（长度范围 1～20）
    #[serde(rename = "invite_user_ids", skip_serializing_if = "Option::is_none")]
    pub invite_user_ids: Option<Vec<String>>,
    /// 要拉入的群 openID，为空则新建群
    #[serde(rename = "chat_id", skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    /// 群名称，当 chat_id 为空时用该名称创建群聊
    #[serde(rename = "chat_name", skip_serializing_if = "Option::is_none")]
    pub chat_name: Option<String>,
}

/// 发起群聊响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatGroupResponse {
    /// 群聊 ID
    #[serde(rename = "chat_id")]
    pub chat_id: String,
    /// 群名称
    #[serde(rename = "name")]
    pub name: String,
    /// 结果消息
    #[serde(rename = "message")]
    pub message: String,
}

impl ApiResponseTrait for ChatGroupResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to ChatGroupRequestBuilder, will be removed in v1.0 (#271)")]
pub type ChatGroupBuilder = ChatGroupRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../user_tasks/{id}/chat_group → 强类型 ChatGroupResponse。
    #[tokio::test]
    async fn test_chat_group_user_task_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/apaas/v1/user_tasks/task_001/chat_group"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "chat_id": "oc_001",
                    "name": "任务协作群",
                    "message": "建群成功"
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

        let resp = ChatGroupRequestBuilder::new(config, "task_001")
            .operator_user_id("u_001")
            .invite_user_ids(vec!["u_002".to_string(), "u_003".to_string()])
            .chat_name("任务协作群")
            .execute()
            .await
            .expect("发起群聊应成功");
        assert_eq!(resp.chat_id, "oc_001");
        assert_eq!(resp.name, "任务协作群");
        assert_eq!(resp.message, "建群成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/user_tasks/task_001/chat_group"
        );
        // 请求体须与官方文档一致：operator_user_id + 可选 invite_user_ids/chat_id/chat_name
        let sent: serde_json::Value = received[0].body_json().expect("请求体应为 JSON");
        assert_eq!(
            sent,
            json!({
                "operator_user_id": "u_001",
                "invite_user_ids": ["u_002", "u_003"],
                "chat_name": "任务协作群"
            })
        );
    }
}
