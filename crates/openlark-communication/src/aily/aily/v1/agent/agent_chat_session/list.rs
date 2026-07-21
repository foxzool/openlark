//! 查询会话列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/aily-v1/agent-agent_chat_session/list>

use std::collections::HashMap;

use crate::endpoints::AILY_V1_AGENT_SESSIONS;
use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use std::sync::Arc;

/// 查询会话列表请求。
#[derive(Debug, Clone)]
pub struct ListAgentChatSessionRequest {
    config: Arc<Config>,
    agent_id: String,
    query: HashMap<String, String>,
}

impl ListAgentChatSessionRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            agent_id: String::new(),
            query: HashMap::new(),
        }
    }

    /// 设置路径参数 `agent_id`。
    pub fn agent_id(mut self, agent_id: impl Into<String>) -> Self {
        self.agent_id = agent_id.into();
        self
    }

    /// 添加查询参数。
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.insert(key.into(), value.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.agent_id, "agent_id 不能为空");

        let url = AILY_V1_AGENT_SESSIONS.replace("{agent_id}", &self.agent_id);
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(&url);
        for (key, value) in self.query {
            req = req.query(key, value);
        }

        Transport::request_typed(req, &self.config, Some(option), "查询会话列表").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_sets_agent_id() {
        let config = Arc::new(Config::default());
        let request = ListAgentChatSessionRequest::new(config).agent_id("agent_123");
        assert_eq!(request.agent_id, "agent_123");
    }

    #[test]
    fn test_list_agent_chat_session_request_url_construction() {
        use crate::endpoints::aily::AILY_V1_AGENT_SESSIONS;
        let url = AILY_V1_AGENT_SESSIONS.replace("{agent_id}", "agent_1");
        assert_eq!(url, "/open-apis/aily/v1/agents/agent_1/sessions");
    }
}
