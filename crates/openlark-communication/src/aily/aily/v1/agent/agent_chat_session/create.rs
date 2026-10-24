//! 创建会话
//!
//! docPath: <https://open.feishu.cn/document/server-docs/aily-v1/agent-agent_chat_session/create>

use crate::endpoints::AILY_V1_AGENT_SESSIONS;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 创建会话请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateAgentChatSessionBody {
    /// 会话名，长度范围 0~200 字符。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CreateAgentChatSessionBody {
    /// 创建新的会话请求体（name 可选，不设置时创建空白会话）。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置会话名。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

/// 创建会话响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentChatSessionResponse {
    /// 会话 ID。
    pub session_id: String,
    /// 会话名。
    pub name: String,
    /// 会话创建时间（毫秒时间戳字符串）。
    pub created_at: String,
}

impl ApiResponseTrait for CreateAgentChatSessionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建智能体会话请求。
#[derive(Debug, Clone)]
pub struct CreateAgentChatSessionRequest {
    config: Arc<Config>,
    agent_id: String,
}

impl CreateAgentChatSessionRequest {
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
    pub async fn execute(
        self,
        body: CreateAgentChatSessionBody,
    ) -> SDKResult<CreateAgentChatSessionResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: CreateAgentChatSessionBody,
        option: RequestOption,
    ) -> SDKResult<CreateAgentChatSessionResponse> {
        validate_required!(self.agent_id, "agent_id 不能为空");

        let url = AILY_V1_AGENT_SESSIONS.replace("{agent_id}", &self.agent_id);
        let req: ApiRequest<CreateAgentChatSessionResponse> =
            ApiRequest::post(&url).json_body(&body);

        Transport::request_typed(req, &self.config, Some(option), "创建会话").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    #[test]
    fn builder_sets_agent_id() {
        let config = Arc::new(Config::default());
        let request = CreateAgentChatSessionRequest::new(config).agent_id("agent_123");
        assert_eq!(request.agent_id, "agent_123");
    }

    #[test]
    fn body_name_optional_serializes_to_empty_object() {
        // 官方 name 可选：不设置时请求体为空对象，创建空白会话
        let json = serde_json::to_value(CreateAgentChatSessionBody::new()).unwrap();
        assert_eq!(json, json!({}));
        let json =
            serde_json::to_value(CreateAgentChatSessionBody::new().name("计算问题")).unwrap();
        assert_eq!(json, json!({"name": "计算问题"}));
    }

    #[test]
    fn test_create_agent_chat_session_request_url_construction() {
        use crate::endpoints::aily::AILY_V1_AGENT_SESSIONS;
        let url = AILY_V1_AGENT_SESSIONS.replace("{agent_id}", "agent_1");
        assert_eq!(url, "/open-apis/aily/v1/agents/agent_1/sessions");
        assert!(
            !url.contains("{"),
            "URL should not contain unreplaced placeholders"
        );
    }

    /// 端到端：POST /open-apis/aily/v1/agents/agent_1/sessions
    #[tokio::test]
    async fn test_create_agent_chat_session_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/aily/v1/agents/agent_1/sessions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "session_id": "conversation_4kj6xxrp2jqk2",
                    "name": "新会话",
                    "created_at": "1783424638072"
                }
            })))
            .mount(&server)
            .await;

        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let resp = CreateAgentChatSessionRequest::new(config)
            .agent_id("agent_1")
            .execute(CreateAgentChatSessionBody::new().name("计算问题"))
            .await
            .expect("请求应成功");
        assert_eq!(resp.session_id, "conversation_4kj6xxrp2jqk2");
        assert_eq!(resp.name, "新会话");
        assert_eq!(resp.created_at, "1783424638072");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
