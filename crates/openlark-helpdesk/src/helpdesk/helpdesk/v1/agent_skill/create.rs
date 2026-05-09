//! 创建客服技能
//!
//! 创建新的客服技能。
//!
//! docPath: https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-function/agent_skill/create

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::{extract_response_data, serialize_params};

/// 创建客服技能请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateAgentSkillBody {
    /// 技能名称
    pub name: String,
    /// 技能描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}

impl CreateAgentSkillBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.name, "name is required");
        Ok(())
    }
}

/// 创建客服技能响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentSkillResponse {
    /// 响应数据。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CreateAgentSkillResult>,
}

impl openlark_core::api::ApiResponseTrait for CreateAgentSkillResponse {}

/// 创建客服技能结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentSkillResult {
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

/// 创建客服技能请求
#[derive(Debug, Clone)]
pub struct CreateAgentSkillRequest {
    config: Arc<Config>,
}

impl CreateAgentSkillRequest {
    /// 创建新的创建客服技能请求
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行创建客服技能请求
    pub async fn execute(self, body: CreateAgentSkillBody) -> SDKResult<CreateAgentSkillResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行创建客服技能请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: CreateAgentSkillBody,
        option: RequestOption,
    ) -> SDKResult<CreateAgentSkillResponse> {
        body.validate()?;

        let req: ApiRequest<CreateAgentSkillResponse> =
            ApiRequest::post(HelpdeskApiV1::AgentSkillCreate.to_url())
                .body(serialize_params(&body, "创建客服技能")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "创建客服技能")
    }
}

/// 创建客服技能请求构建器
#[derive(Debug, Clone)]
pub struct CreateAgentSkillRequestBuilder {
    config: Arc<Config>,
    name: Option<String>,
    description: Option<String>,
    enable: Option<bool>,
}

impl CreateAgentSkillRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            name: None,
            description: None,
            enable: None,
        }
    }

    /// 设置技能名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置技能描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// 设置是否启用
    pub fn enable(mut self, enable: bool) -> Self {
        self.enable = Some(enable);
        self
    }

    /// 构建请求体
    pub fn body(&self) -> Result<CreateAgentSkillBody, String> {
        let name = self.name.clone().ok_or("name is required")?;

        Ok(CreateAgentSkillBody {
            name,
            description: self.description.clone(),
            enable: self.enable,
        })
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<CreateAgentSkillResponse> {
        let body = self
            .body()
            .map_err(|reason| openlark_core::error::validation_error("body", reason))?;
        let request = CreateAgentSkillRequest::new(self.config.clone());
        request.execute(body).await
    }
}

/// 执行创建客服技能
pub async fn create_agent_skill(
    config: &Config,
    body: CreateAgentSkillBody,
) -> SDKResult<CreateAgentSkillResponse> {
    create_agent_skill_with_options(config, body, RequestOption::default()).await
}

/// 执行创建客服技能（支持自定义选项）
pub async fn create_agent_skill_with_options(
    config: &Config,
    body: CreateAgentSkillBody,
    option: RequestOption,
) -> SDKResult<CreateAgentSkillResponse> {
    body.validate()?;

    let req: ApiRequest<CreateAgentSkillResponse> =
        ApiRequest::post(HelpdeskApiV1::AgentSkillCreate.to_url())
            .body(serialize_params(&body, "创建客服技能")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "创建客服技能")
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_valid() {
        let body = CreateAgentSkillBody {
            name: "技术支持".to_string(),
            description: Some("提供技术支持服务".to_string()),
            enable: Some(true),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_validation_empty_name() {
        let body = CreateAgentSkillBody {
            name: "".to_string(),
            description: None,
            enable: None,
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_builder_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = CreateAgentSkillRequestBuilder::new(Arc::new(config));

        assert!(builder.name.is_none());
    }
}
