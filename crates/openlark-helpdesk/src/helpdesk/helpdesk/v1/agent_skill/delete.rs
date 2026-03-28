//! 删除指定客服技能
//!
//! 删除指定的客服技能。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-function/agent_skill/delete>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::extract_response_data;

/// 删除客服技能响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAgentSkillResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DeleteAgentSkillResult>,
}

impl openlark_core::api::ApiResponseTrait for DeleteAgentSkillResponse {}

/// 删除客服技能结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAgentSkillResult {
    /// 是否删除成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

/// 删除客服技能请求
#[derive(Debug, Clone)]
pub struct DeleteAgentSkillRequest {
    config: Arc<Config>,
    agent_skill_id: String,
}

impl DeleteAgentSkillRequest {
    /// 创建新的删除客服技能请求
    pub fn new(config: Arc<Config>, agent_skill_id: String) -> Self {
        Self {
            config,
            agent_skill_id,
        }
    }

    /// 执行删除客服技能请求
    pub async fn execute(self) -> SDKResult<DeleteAgentSkillResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行删除客服技能请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteAgentSkillResponse> {
        let req: ApiRequest<DeleteAgentSkillResponse> = ApiRequest::delete(
            HelpdeskApiV1::AgentSkillDelete(self.agent_skill_id.clone()).to_url(),
        );

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "删除客服技能")
    }
}

/// 删除客服技能请求构建器
#[derive(Debug, Clone)]
pub struct DeleteAgentSkillRequestBuilder {
    config: Arc<Config>,
    agent_skill_id: String,
}

impl DeleteAgentSkillRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, agent_skill_id: String) -> Self {
        Self {
            config,
            agent_skill_id,
        }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<DeleteAgentSkillResponse> {
        let request =
            DeleteAgentSkillRequest::new(self.config.clone(), self.agent_skill_id.clone());
        request.execute().await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        &self,
        option: RequestOption,
    ) -> SDKResult<DeleteAgentSkillResponse> {
        let request =
            DeleteAgentSkillRequest::new(self.config.clone(), self.agent_skill_id.clone());
        request.execute_with_options(option).await
    }
}

/// 执行删除客服技能
pub async fn delete_agent_skill(
    config: &Config,
    agent_skill_id: String,
) -> SDKResult<DeleteAgentSkillResponse> {
    delete_agent_skill_with_options(config, agent_skill_id, RequestOption::default()).await
}

/// 执行删除客服技能（支持自定义选项）
pub async fn delete_agent_skill_with_options(
    config: &Config,
    agent_skill_id: String,
    option: RequestOption,
) -> SDKResult<DeleteAgentSkillResponse> {
    let req: ApiRequest<DeleteAgentSkillResponse> =
        ApiRequest::delete(HelpdeskApiV1::AgentSkillDelete(agent_skill_id).to_url());

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "删除客服技能")
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
        let builder =
            DeleteAgentSkillRequestBuilder::new(Arc::new(config), "skill_123".to_string());

        assert_eq!(builder.agent_skill_id, "skill_123");
    }
}
