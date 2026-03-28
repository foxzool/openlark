//! 获取客服技能列表
//!
//! 获取全部客服技能列表。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-function/agent_skill/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::extract_response_data;

/// 获取客服技能列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAgentSkillResponse {
    /// 客服技能列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<AgentSkillItem>>,
}

impl ApiResponseTrait for ListAgentSkillResponse {
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

/// 获取客服技能列表请求
#[derive(Debug, Clone)]
pub struct ListAgentSkillRequest {
    config: Arc<Config>,
}

impl ListAgentSkillRequest {
    /// 创建新的获取客服技能列表请求
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行获取客服技能列表请求
    pub async fn execute(self) -> SDKResult<ListAgentSkillResponse> {
        let api_endpoint = HelpdeskApiV1::AgentSkillList;
        let request = ApiRequest::<ListAgentSkillResponse>::get(api_endpoint.to_url());

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "获取客服技能列表")
    }
}

/// 获取客服技能列表请求构建器
#[derive(Debug, Clone)]
pub struct ListAgentSkillRequestBuilder {
    config: Arc<Config>,
}

impl ListAgentSkillRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<ListAgentSkillResponse> {
        let api_endpoint = HelpdeskApiV1::AgentSkillList;
        let request = ApiRequest::<ListAgentSkillResponse>::get(api_endpoint.to_url());

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "获取客服技能列表")
    }
}

/// 执行获取客服技能列表
pub async fn list_agent_skills(config: &Config) -> SDKResult<ListAgentSkillResponse> {
    let api_endpoint = HelpdeskApiV1::AgentSkillList;
    let request = ApiRequest::<ListAgentSkillResponse>::get(api_endpoint.to_url());

    let response = Transport::request(request, config, None).await?;
    extract_response_data(response, "获取客服技能列表")
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
        let _builder = ListAgentSkillRequestBuilder::new(Arc::new(config));

        // Builder created successfully
    }
}
