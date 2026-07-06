//! 删除会话
//!
//! docPath: <https://open.feishu.cn/document/server-docs/aily-v1/agent-agent_chat_session/delete>

use crate::common::api_utils::extract_response_data;
use crate::endpoints::AILY_V1_AGENT_SESSION;
use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use std::sync::Arc;

/// 删除智能体会话请求。
#[derive(Debug, Clone)]
pub struct DeleteAgentChatSessionRequest {
    config: Arc<Config>,
    agent_id: String,
    agent_chat_session_id: String,
}

impl DeleteAgentChatSessionRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            agent_id: String::new(),
            agent_chat_session_id: String::new(),
        }
    }

    /// 设置路径参数 `agent_id`。
    pub fn agent_id(mut self, agent_id: impl Into<String>) -> Self {
        self.agent_id = agent_id.into();
        self
    }

    /// 设置路径参数 `agent_chat_session_id`。
    pub fn agent_chat_session_id(mut self, agent_chat_session_id: impl Into<String>) -> Self {
        self.agent_chat_session_id = agent_chat_session_id.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.agent_id, "agent_id 不能为空");
        validate_required!(self.agent_chat_session_id, "agent_chat_session_id 不能为空");

        let url = AILY_V1_AGENT_SESSION
            .replace("{agent_id}", &self.agent_id)
            .replace("{agent_chat_session_id}", &self.agent_chat_session_id);

        let req: ApiRequest<()> = ApiRequest::delete(&url);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "删除会话")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_sets_params() {
        let config = Arc::new(Config::default());
        let request = DeleteAgentChatSessionRequest::new(config)
            .agent_id("agent_123")
            .agent_chat_session_id("session_456");
        assert_eq!(request.agent_id, "agent_123");
        assert_eq!(request.agent_chat_session_id, "session_456");
    }

    #[test]
    fn test_delete_agent_chat_session_request_url_construction() {
        use crate::endpoints::aily::AILY_V1_AGENT_SESSION;
        let url = AILY_V1_AGENT_SESSION
            .replace("{agent_id}", "agent_1")
            .replace("{agent_chat_session_id}", "sess_1");
        assert_eq!(url, "/open-apis/aily/v1/agents/agent_1/sessions/sess_1");
        assert!(
            !url.contains("{"),
            "URL should not contain unreplaced placeholders"
        );
    }
}
