//! 获取指定客服技能
//!
//! 获取指定客服技能的详情。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-function/agent_skill/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;

/// 获取指定客服技能响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAgentSkillResponse {
    /// 响应数据。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<AgentSkillItem>,
}

impl ApiResponseTrait for GetAgentSkillResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 客服技能项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSkillItem {
    /// 技能ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 技能名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 技能描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}

/// 获取指定客服技能请求
#[derive(Debug, Clone)]
pub struct GetAgentSkillRequest {
    config: Arc<Config>,
    agent_skill_id: String,
}

impl GetAgentSkillRequest {
    /// 创建新的获取指定客服技能请求
    pub fn new(config: Arc<Config>, agent_skill_id: String) -> Self {
        Self {
            config,
            agent_skill_id,
        }
    }

    /// 执行获取指定客服技能请求
    pub async fn execute(self) -> SDKResult<GetAgentSkillResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetAgentSkillResponse> {
        let api_endpoint = HelpdeskApiV1::AgentSkillGet(self.agent_skill_id.clone());
        let request = ApiRequest::<GetAgentSkillResponse>::get(api_endpoint.to_url());

        Transport::request_typed(request, &self.config, Some(option), "获取指定客服技能").await
    }
}

/// 获取指定客服技能请求构建器
#[derive(Debug, Clone)]
pub struct GetAgentSkillRequestBuilder {
    config: Arc<Config>,
    agent_skill_id: String,
}

impl GetAgentSkillRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, agent_skill_id: String) -> Self {
        Self {
            config,
            agent_skill_id,
        }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<GetAgentSkillResponse> {
        let api_endpoint = HelpdeskApiV1::AgentSkillGet(self.agent_skill_id.clone());
        let request = ApiRequest::<GetAgentSkillResponse>::get(api_endpoint.to_url());

        Transport::request_typed(request, &self.config, None, "获取指定客服技能").await
    }
}

/// 执行获取指定客服技能
pub async fn get_agent_skill(
    config: &Config,
    agent_skill_id: String,
) -> SDKResult<GetAgentSkillResponse> {
    let api_endpoint = HelpdeskApiV1::AgentSkillGet(agent_skill_id);
    let request = ApiRequest::<GetAgentSkillResponse>::get(api_endpoint.to_url());

    Transport::request_typed(request, config, None, "获取指定客服技能").await
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = GetAgentSkillRequestBuilder::new(Arc::new(config), "skill_123".to_string());

        assert_eq!(builder.agent_skill_id, "skill_123");
    }

    /// 端到端：GET .../agent_skills/{agent_skill_id} → 强类型 GetAgentSkillResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_get_agent_skill_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/helpdesk/v1/agent_skills/skl_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "id": "skl_001", "name": "技术支持", "enable": true } }
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

        let resp = GetAgentSkillRequest::new(config, "skl_001".to_string())
            .execute()
            .await
            .expect("获取指定客服技能应成功");
        assert!(resp.data.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/agent_skills/skl_001"
        );
    }
}
