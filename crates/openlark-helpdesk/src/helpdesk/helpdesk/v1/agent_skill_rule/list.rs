//! 获取客服技能规则列表
//!
//! 获取全部客服技能规则。
//!
//! docPath: https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-function/agent_skill_rule/list

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::extract_response_data;

/// 获取客服技能规则列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAgentSkillRuleResponse {
    /// 客服技能规则列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<AgentSkillRuleItem>>,
}

impl ApiResponseTrait for ListAgentSkillRuleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 客服技能规则项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSkillRuleItem {
    /// 规则ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 规则名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 技能ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_id: Option<String>,
    /// 优先级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

/// 获取客服技能规则列表请求
#[derive(Debug, Clone)]
pub struct ListAgentSkillRuleRequest {
    config: Arc<Config>,
}

impl ListAgentSkillRuleRequest {
    /// 创建新的获取客服技能规则列表请求
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行获取客服技能规则列表请求
    pub async fn execute(self) -> SDKResult<ListAgentSkillRuleResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListAgentSkillRuleResponse> {
        let api_endpoint = HelpdeskApiV1::AgentSkillRuleList;
        let request = ApiRequest::<ListAgentSkillRuleResponse>::get(api_endpoint.to_url());

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取客服技能规则列表")
    }
}

/// 获取客服技能规则列表请求构建器
#[derive(Debug, Clone)]
pub struct ListAgentSkillRuleRequestBuilder {
    config: Arc<Config>,
}

impl ListAgentSkillRuleRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<ListAgentSkillRuleResponse> {
        let api_endpoint = HelpdeskApiV1::AgentSkillRuleList;
        let request = ApiRequest::<ListAgentSkillRuleResponse>::get(api_endpoint.to_url());

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "获取客服技能规则列表")
    }
}

/// 执行获取客服技能规则列表
pub async fn list_agent_skill_rules(config: &Config) -> SDKResult<ListAgentSkillRuleResponse> {
    let api_endpoint = HelpdeskApiV1::AgentSkillRuleList;
    let request = ApiRequest::<ListAgentSkillRuleResponse>::get(api_endpoint.to_url());

    let response = Transport::request(request, config, None).await?;
    extract_response_data(response, "获取客服技能规则列表")
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
        let _builder = ListAgentSkillRuleRequestBuilder::new(Arc::new(config));
    }
}
