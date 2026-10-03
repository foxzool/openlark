//! 创建客服技能
//!
//! 创建新的客服技能。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-function/agent_skill/create>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::serialize_params;

/// 创建客服技能请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateAgentSkillBody {
    /// 技能名称。
    pub name: String,
    /// 技能规则。
    pub rules: Vec<AgentSkillRule>,
    /// 绑定的客服 ID 列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<Vec<String>>,
}

/// 客服技能规则。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSkillRule {
    /// 规则 ID。
    pub id: String,
    /// 规则操作符。
    pub selected_operator: i32,
    /// 规则操作数。
    pub operand: Value,
    /// 规则分类。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<i32>,
}

impl CreateAgentSkillBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.name, "name 不能为空");
        validate_required!(self.rules, "rules 不能为空");
        for rule in &self.rules {
            validate_required!(rule.id, "规则 id 不能为空");
        }
        Ok(())
    }
}

/// 创建客服技能响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentSkillResponse {
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

impl openlark_core::api::ApiResponseTrait for CreateAgentSkillResponse {}

/// 创建客服技能结果。
pub type CreateAgentSkillResult = CreateAgentSkillResponse;

/// 创建客服技能请求
#[derive(Debug, Clone)]
pub struct CreateAgentSkillRequest {
    config: Config,
}

impl CreateAgentSkillRequest {
    /// 创建新的创建客服技能请求
    pub fn new(config: Config) -> Self {
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

        Transport::request_typed(req, &self.config, Some(option), "创建客服技能").await
    }
}

/// 创建客服技能请求构建器
#[derive(Debug, Clone)]
pub struct CreateAgentSkillRequestBuilder {
    config: Config,
    name: Option<String>,
    rules: Vec<AgentSkillRule>,
    agent_ids: Option<Vec<String>>,
}

impl CreateAgentSkillRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Config) -> Self {
        Self {
            config,
            name: None,
            rules: Vec::new(),
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
        self.rules = rules;
        self
    }

    /// 添加一条技能规则。
    pub fn rule(mut self, rule: AgentSkillRule) -> Self {
        self.rules.push(rule);
        self
    }

    /// 设置绑定的客服 ID 列表。
    pub fn agent_ids(mut self, agent_ids: Vec<String>) -> Self {
        self.agent_ids = Some(agent_ids);
        self
    }

    /// 构建请求体
    pub fn body(&self) -> Result<CreateAgentSkillBody, String> {
        let name = self.name.clone().ok_or("name is required")?;

        Ok(CreateAgentSkillBody {
            name,
            rules: self.rules.clone(),
            agent_ids: self.agent_ids.clone(),
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

    Transport::request_typed(req, config, Some(option), "创建客服技能").await
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_body_serialization_matches_official_schema() {
        let body = CreateAgentSkillBody {
            name: "技术支持".to_string(),
            rules: vec![AgentSkillRule {
                id: "rule_001".to_string(),
                selected_operator: 1,
                operand: json!("vip"),
                category: Some(2),
            }],
            agent_ids: Some(vec!["agent_001".to_string()]),
        };
        assert!(body.validate().is_ok());
        assert_eq!(
            serde_json::to_value(body).expect("序列化请求体失败"),
            json!({
                "name": "技术支持",
                "rules": [{
                    "id": "rule_001",
                    "selected_operator": 1,
                    "operand": "vip",
                    "category": 2
                }],
                "agent_ids": ["agent_001"]
            })
        );
    }

    #[test]
    fn test_body_validation_rejects_empty_name() {
        let body = CreateAgentSkillBody {
            name: " ".to_string(),
            rules: vec![AgentSkillRule {
                id: "rule_001".to_string(),
                selected_operator: 1,
                operand: json!("vip"),
                category: None,
            }],
            agent_ids: None,
        };
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_builder_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = CreateAgentSkillRequestBuilder::new(config);

        assert!(builder.name.is_none());
    }

    /// 端到端：POST .../agent_skills → 强类型响应解析。
    #[tokio::test]
    async fn test_create_agent_skill_returns_data_on_success() {
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/helpdesk/v1/agent_skills"))
            .and(body_json(json!({
                "name": "技术支持",
                "rules": [{
                    "id": "rule_001",
                    "selected_operator": 1,
                    "operand": "vip",
                    "category": 2
                }],
                "agent_ids": ["agent_001"]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "id": "skl_001", "name": "技术支持" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let body = CreateAgentSkillBody {
            name: "技术支持".to_string(),
            rules: vec![AgentSkillRule {
                id: "rule_001".to_string(),
                selected_operator: 1,
                operand: json!("vip"),
                category: Some(2),
            }],
            agent_ids: Some(vec!["agent_001".to_string()]),
        };
        let resp = CreateAgentSkillRequest::new(config)
            .execute(body)
            .await
            .expect("创建客服技能应成功");
        assert_eq!(resp.id.as_deref(), Some("skl_001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/agent_skills"
        );
    }
}
