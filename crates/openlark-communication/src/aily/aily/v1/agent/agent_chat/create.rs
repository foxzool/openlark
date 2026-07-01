//! 发起智能体会话
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/aily-v1/agent-chat/create>

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::AILY_V1_AGENT_CHATS;
use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use std::sync::Arc;

/// 发起智能体会话请求。
#[derive(Debug, Clone)]
pub struct CreateAgentChatRequest {
    config: Arc<Config>,
    agent_id: String,
}

impl CreateAgentChatRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            agent_id: String::new(),
        }
    }

    /// 设置路径参数 `agent_id`。
    pub fn agent_id(mut self, agent_id: impl Into<String>) -> Self {
        self.agent_id = agent_id.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.agent_id, "agent_id 不能为空");

        if body.is_null() {
            return Err(openlark_core::error::validation_error(
                "body",
                "请求体不能为空",
            ));
        }

        let url = AILY_V1_AGENT_CHATS.replace("{agent_id}", &self.agent_id);
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(&url).body(serialize_params(&body, "发起智能体会话")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "发起智能体会话")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = CreateAgentChatRequest::new(config);
    }

    #[test]
    fn builder_sets_agent_id() {
        let config = Arc::new(Config::default());
        let request = CreateAgentChatRequest::new(config).agent_id("agent_123");
        assert_eq!(request.agent_id, "agent_123");
    }

    #[test]
    fn test_create_agent_chat_request_url_construction() {
        use crate::endpoints::aily::AILY_V1_AGENT_CHATS;
        let url = AILY_V1_AGENT_CHATS.replace("{agent_id}", "agent_1");
        assert_eq!(url, "/open-apis/aily/v1/agents/agent_1/chats");
        assert!(
            !url.contains("{"),
            "URL should not contain unreplaced placeholders"
        );
    }
}
