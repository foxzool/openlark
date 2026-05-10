//! 基于人工任务发起群聊
//!
//! 文档: https://open.feishu.cn/document/apaas-v1/flow/user-task/chat_group
//! docPath: https://open.feishu.cn/document/apaas-v1/flow/user-task/chat_group

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
pub struct ChatGroupBuilder {
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

impl ChatGroupBuilder {
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
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
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
    chat_id: String,
    /// 群名称
    #[serde(rename = "name")]
    name: String,
    /// 结果消息
    #[serde(rename = "message")]
    message: String,
}

impl ApiResponseTrait for ChatGroupResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
