//! 获取会话结果
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/aily-v1/agent-chat/get>

use crate::common::api_utils::extract_response_data;
use crate::endpoints::AILY_V1_AGENT_CHAT;
use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use std::sync::Arc;

/// 获取会话结果请求。
#[derive(Debug, Clone)]
pub struct GetAgentChatRequest {
    config: Arc<Config>,
    agent_id: String,
    agent_chat_id: String,
}

impl GetAgentChatRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            agent_id: String::new(),
            agent_chat_id: String::new(),
        }
    }

    /// 设置路径参数 `agent_id`。
    pub fn agent_id(mut self, agent_id: impl Into<String>) -> Self {
        self.agent_id = agent_id.into();
        self
    }

    /// 设置路径参数 `agent_chat_id`。
    pub fn agent_chat_id(mut self, agent_chat_id: impl Into<String>) -> Self {
        self.agent_chat_id = agent_chat_id.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.agent_id, "agent_id 不能为空");
        validate_required!(self.agent_chat_id, "agent_chat_id 不能为空");

        let url = AILY_V1_AGENT_CHAT
            .replace("{agent_id}", &self.agent_id)
            .replace("{agent_chat_id}", &self.agent_chat_id);

        let req: ApiRequest<serde_json::Value> = ApiRequest::get(&url);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取会话结果")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = GetAgentChatRequest::new(config);
    }

    #[test]
    fn builder_sets_params() {
        let config = Arc::new(Config::default());
        let request = GetAgentChatRequest::new(config)
            .agent_id("agent_123")
            .agent_chat_id("chat_456");
        assert_eq!(request.agent_id, "agent_123");
        assert_eq!(request.agent_chat_id, "chat_456");
    }

    #[test]
    fn test_get_agent_chat_request_url_construction() {
        use crate::endpoints::aily::AILY_V1_AGENT_CHAT;
        let url = AILY_V1_AGENT_CHAT
            .replace("{agent_id}", "agent_1")
            .replace("{agent_chat_id}", "chat_1");
        assert_eq!(url, "/open-apis/aily/v1/agents/agent_1/chats/chat_1");
        assert!(
            !url.contains("{"),
            "URL should not contain unreplaced placeholders"
        );
    }
}
