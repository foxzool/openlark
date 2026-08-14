//! 更新指定客服技能
//!
//! 更新指定客服技能的信息。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-function/agent_skill/patch>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::serialize_params;
use crate::helpdesk::helpdesk::v1::agent_skill::create::AgentSkillRule;

/// 更新客服技能请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchAgentSkillBody {
    /// 客服技能。
    pub agent_skill: AgentSkill,
}

/// 待更新的客服技能。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentSkill {
    /// 技能名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 技能规则。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<AgentSkillRule>>,
    /// 绑定的客服 ID 列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<Vec<String>>,
}

impl PatchAgentSkillBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        if let Some(name) = &self.agent_skill.name {
            validate_required!(name, "name 不能为空");
        }
        if let Some(rules) = &self.agent_skill.rules {
            for rule in rules {
                validate_required!(rule.id, "规则 id 不能为空");
            }
        }
        Ok(())
    }
}

/// 更新客服技能响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchAgentSkillResponse {
    /// 技能 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 技能名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 技能规则。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<AgentSkillRule>>,
    /// 绑定的客服 ID 列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<Vec<String>>,
}

impl openlark_core::api::ApiResponseTrait for PatchAgentSkillResponse {}

/// 更新客服技能结果。
pub type PatchAgentSkillResult = PatchAgentSkillResponse;

/// 更新客服技能请求
#[derive(Debug, Clone)]
pub struct PatchAgentSkillRequest {
    config: Config,
    agent_skill_id: String,
}

impl PatchAgentSkillRequest {
    /// 创建新的更新客服技能请求
    pub fn new(config: Config, agent_skill_id: String) -> Self {
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
    config: Config,
    agent_skill_id: String,
    name: Option<String>,
    rules: Option<Vec<AgentSkillRule>>,
    agent_ids: Option<Vec<String>>,
}

impl PatchAgentSkillRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Config, agent_skill_id: String) -> Self {
        Self {
            config,
            agent_skill_id,
            name: None,
            rules: None,
            agent_ids: None,
        }
    }

    /// 设置技能名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置技能规则。
    pub fn rules(mut self, rules: Vec<AgentSkillRule>) -> Self {
        self.rules = Some(rules);
        self
    }

    /// 设置绑定的客服 ID 列表。
    pub fn agent_ids(mut self, agent_ids: Vec<String>) -> Self {
        self.agent_ids = Some(agent_ids);
        self
    }

    /// 构建请求体
    pub fn body(&self) -> PatchAgentSkillBody {
        PatchAgentSkillBody {
            agent_skill: AgentSkill {
                name: self.name.clone(),
                rules: self.rules.clone(),
                agent_ids: self.agent_ids.clone(),
            },
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
    use serde_json::json;

    #[test]
    fn test_body_validation_empty() {
        let body = PatchAgentSkillBody::default();
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_serialization_matches_official_schema() {
        let body = PatchAgentSkillBody {
            agent_skill: AgentSkill {
                name: Some("新技能名称".to_string()),
                rules: Some(vec![AgentSkillRule {
                    id: "rule_001".to_string(),
                    selected_operator: 1,
                    operand: json!("vip"),
                    category: None,
                }]),
                agent_ids: Some(vec!["agent_001".to_string()]),
            },
        };
        assert!(body.validate().is_ok());
        assert_eq!(
            serde_json::to_value(body).expect("序列化请求体失败"),
            json!({
                "agent_skill": {
                    "name": "新技能名称",
                    "rules": [{
                        "id": "rule_001",
                        "selected_operator": 1,
                        "operand": "vip"
                    }],
                    "agent_ids": ["agent_001"]
                }
            })
        );
    }

    #[test]
    fn test_body_validation_empty_name() {
        let body = PatchAgentSkillBody {
            agent_skill: AgentSkill {
                name: Some(" ".to_string()),
                rules: None,
                agent_ids: None,
            },
        };
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_builder_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = PatchAgentSkillRequestBuilder::new(config, "skill_123".to_string());

        assert_eq!(builder.agent_skill_id, "skill_123");
        assert!(builder.name.is_none());
    }

    /// 端到端：PATCH .../agent_skills/{agent_skill_id} → 强类型响应解析。
    #[tokio::test]
    async fn test_patch_agent_skill_returns_data_on_success() {
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/helpdesk/v1/agent_skills/skl_001"))
            .and(body_json(json!({
                "agent_skill": {
                    "name": "更新后技能",
                    "rules": [{
                        "id": "rule_001",
                        "selected_operator": 1,
                        "operand": "vip"
                    }],
                    "agent_ids": ["agent_001"]
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "id": "skl_001", "name": "更新后技能" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let body = PatchAgentSkillBody {
            agent_skill: AgentSkill {
                name: Some("更新后技能".to_string()),
                rules: Some(vec![AgentSkillRule {
                    id: "rule_001".to_string(),
                    selected_operator: 1,
                    operand: json!("vip"),
                    category: None,
                }]),
                agent_ids: Some(vec!["agent_001".to_string()]),
            },
        };
        let resp = PatchAgentSkillRequest::new(config, "skl_001".to_string())
            .execute(body)
            .await
            .expect("更新客服技能应成功");
        assert_eq!(resp.id.as_deref(), Some("skl_001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/agent_skills/skl_001"
        );
    }
}
