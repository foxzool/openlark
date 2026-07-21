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
};
use serde::{Deserialize, Serialize};

/// 基于人工任务发起群聊 Builder
#[derive(Debug, Clone)]
pub struct ChatGroupRequestBuilder {
    config: Config,
    /// 任务 ID
    task_id: String,
    /// 群名称
    name: Option<String>,
    /// 群成员 ID 列表
    owner_ids: Vec<String>,
    /// 群成员 ID 列表
    member_ids: Vec<String>,
}

impl ChatGroupRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, task_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
            name: None,
            owner_ids: Vec::new(),
            member_ids: Vec::new(),
        }
    }

    /// 设置群名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 添加群主 ID
    pub fn owner_id(mut self, owner_id: impl Into<String>) -> Self {
        self.owner_ids.push(owner_id.into());
        self
    }

    /// 添加多个群主 ID
    pub fn owner_ids(mut self, owner_ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.owner_ids.extend(owner_ids.into_iter().map(Into::into));
        self
    }

    /// 添加群成员 ID
    pub fn member_id(mut self, member_id: impl Into<String>) -> Self {
        self.member_ids.push(member_id.into());
        self
    }

    /// 添加多个群成员 ID
    pub fn member_ids(mut self, member_ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.member_ids
            .extend(member_ids.into_iter().map(Into::into));
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ChatGroupResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<ChatGroupResponse> {
        let url = format!("/open-apis/apaas/v1/user_tasks/{}/chat_group", self.task_id);

        let request = ChatGroupRequest {
            name: self.name,
            owner_ids: self.owner_ids,
            member_ids: self.member_ids,
        };

        let req: ApiRequest<ChatGroupResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        Transport::request_typed(req, &self.config, Some(option), "Operation").await
    }
}

/// 发起群聊请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct ChatGroupRequest {
    /// 群名称
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// 群主 ID 列表
    #[serde(rename = "owner_ids", skip_serializing_if = "Vec::is_empty")]
    owner_ids: Vec<String>,
    /// 群成员 ID 列表
    #[serde(rename = "member_ids", skip_serializing_if = "Vec::is_empty")]
    member_ids: Vec<String>,
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
            .name("任务协作群")
            .owner_id("u_001")
            .member_ids(vec!["u_002".to_string(), "u_003".to_string()])
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
    }
}
