//! 更新指定客服技能
//!
//! 更新指定客服技能的信息。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-function/agent_skill/patch>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::serialize_params;

/// 更新客服技能请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchAgentSkillBody {
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

impl PatchAgentSkillBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        if let Some(name) = &self.name
            && name.is_empty()
        {
            return Err(openlark_core::CoreError::validation_msg(
                "name cannot be empty",
            ));
        }
        Ok(())
    }
}

/// 更新客服技能响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchAgentSkillResponse {
    /// 响应数据。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<PatchAgentSkillResult>,
}

impl openlark_core::api::ApiResponseTrait for PatchAgentSkillResponse {}

/// 更新客服技能结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchAgentSkillResult {
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

/// 更新客服技能请求
#[derive(Debug, Clone)]
pub struct PatchAgentSkillRequest {
    config: Arc<Config>,
    agent_skill_id: String,
}

impl PatchAgentSkillRequest {
    /// 创建新的更新客服技能请求
    pub fn new(config: Arc<Config>, agent_skill_id: String) -> Self {
        Self {
            config,
            agent_skill_id,
        }
    }

    /// 执行更新客服技能请求
    pub async fn execute(self, body: PatchAgentSkillBody) -> SDKResult<PatchAgentSkillResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行更新客服技能请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: PatchAgentSkillBody,
        option: RequestOption,
    ) -> SDKResult<PatchAgentSkillResponse> {
        body.validate()?;

        let req: ApiRequest<PatchAgentSkillResponse> =
            ApiRequest::patch(HelpdeskApiV1::AgentSkillPatch(self.agent_skill_id.clone()).to_url())
                .body(serialize_params(&body, "更新客服技能")?);

        Transport::request_typed(req, &self.config, Some(option), "更新客服技能").await
    }
}

/// 更新客服技能请求构建器
#[derive(Debug, Clone)]
pub struct PatchAgentSkillRequestBuilder {
    config: Arc<Config>,
    agent_skill_id: String,
    name: Option<String>,
    description: Option<String>,
    enable: Option<bool>,
}

impl PatchAgentSkillRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, agent_skill_id: String) -> Self {
        Self {
            config,
            agent_skill_id,
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
    pub fn body(&self) -> PatchAgentSkillBody {
        PatchAgentSkillBody {
            name: self.name.clone(),
            description: self.description.clone(),
            enable: self.enable,
        }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<PatchAgentSkillResponse> {
        let body = self.body();
        let request = PatchAgentSkillRequest::new(self.config.clone(), self.agent_skill_id.clone());
        request.execute(body).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        &self,
        option: RequestOption,
    ) -> SDKResult<PatchAgentSkillResponse> {
        let body = self.body();
        let request = PatchAgentSkillRequest::new(self.config.clone(), self.agent_skill_id.clone());
        request.execute_with_options(body, option).await
    }
}

/// 执行更新客服技能
pub async fn patch_agent_skill(
    config: &Config,
    agent_skill_id: String,
    body: PatchAgentSkillBody,
) -> SDKResult<PatchAgentSkillResponse> {
    patch_agent_skill_with_options(config, agent_skill_id, body, RequestOption::default()).await
}

/// 执行更新客服技能（支持自定义选项）
pub async fn patch_agent_skill_with_options(
    config: &Config,
    agent_skill_id: String,
    body: PatchAgentSkillBody,
    option: RequestOption,
) -> SDKResult<PatchAgentSkillResponse> {
    body.validate()?;

    let req: ApiRequest<PatchAgentSkillResponse> =
        ApiRequest::patch(HelpdeskApiV1::AgentSkillPatch(agent_skill_id).to_url())
            .body(serialize_params(&body, "更新客服技能")?);

    Transport::request_typed(req, config, Some(option), "更新客服技能").await
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_empty() {
        let body = PatchAgentSkillBody::default();
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = PatchAgentSkillBody {
            name: Some("新技能名称".to_string()),
            description: Some("更新描述".to_string()),
            enable: Some(true),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_validation_empty_name() {
        let body = PatchAgentSkillBody {
            name: Some("".to_string()),
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
        let builder = PatchAgentSkillRequestBuilder::new(Arc::new(config), "skill_123".to_string());

        assert_eq!(builder.agent_skill_id, "skill_123");
        assert!(builder.name.is_none());
    }

    /// 端到端：PATCH .../agent_skills/{agent_skill_id} → 强类型 PatchAgentSkillResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_patch_agent_skill_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/helpdesk/v1/agent_skills/skl_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "id": "skl_001", "name": "更新后技能", "enable": true } }
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

        let body = PatchAgentSkillBody {
            name: Some("更新后技能".to_string()),
            enable: Some(true),
            description: None,
        };
        let resp = PatchAgentSkillRequest::new(config, "skl_001".to_string())
            .execute(body)
            .await
            .expect("更新客服技能应成功");
        assert!(resp.data.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/agent_skills/skl_001"
        );
    }
}
