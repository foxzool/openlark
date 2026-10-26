//! 创建收信规则

use crate::common::api_utils::serialize_params;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Create Mailbox Rule Request。
#[derive(Debug, Clone)]
pub struct CreateMailboxRuleRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
    body: CreateMailboxRuleBody,
}

/// 创建收信规则的请求体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateMailboxRuleBody {
    /// 匹配条件。
    pub condition: MailboxRuleCondition,
    /// 匹配命中后的操作。
    pub action: MailboxRuleAction,
    /// 是否终止规则。
    pub ignore_the_rest_of_rules: bool,
    /// 规则名称（1～255 字符）。
    pub name: String,
    /// 是否启用。
    pub is_enable: bool,
}

/// 收信规则匹配条件（rule_condition）。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MailboxRuleCondition {
    /// 匹配类型（1：满足所有条件；2：满足任意条件）。
    pub match_type: i32,
    /// 匹配规则列表（1～32 个）。
    pub items: Vec<MailboxRuleConditionItem>,
}

/// 收信规则匹配条件项（rule_condition_item）。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MailboxRuleConditionItem {
    /// 匹配条件左值（1：发件人地址 … 16：有附件）。
    #[serde(rename = "type")]
    pub condition_type: i32,
    /// 匹配条件操作符（1：包含 … 10：为空）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<i32>,
    /// 匹配条件右值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
}

/// 收信规则命中后的操作（rule_action）。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MailboxRuleAction {
    /// 匹配命中规则后的操作列表（1～32 个）。
    pub items: Vec<MailboxRuleActionItem>,
}

/// 收信规则命中后的操作项（rule_action_item）。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MailboxRuleActionItem {
    /// 操作类型（1：归档 … 13：分享到会话）。
    #[serde(rename = "type")]
    pub action_type: i32,
    /// 当 type 为移至用户文件夹（11）时，该字段填文件夹的 id。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
}

/// Create Mailbox Rule Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMailboxRuleResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for CreateMailboxRuleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateMailboxRuleRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, user_mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            user_mailbox_id: user_mailbox_id.into(),
            body: CreateMailboxRuleBody::default(),
        }
    }

    /// 设置匹配条件（必填）。
    pub fn condition(mut self, condition: MailboxRuleCondition) -> Self {
        self.body.condition = condition;
        self
    }

    /// 设置匹配命中后的操作（必填）。
    pub fn action(mut self, action: MailboxRuleAction) -> Self {
        self.body.action = action;
        self
    }

    /// 设置是否终止规则（必填）。
    pub fn ignore_the_rest_of_rules(mut self, ignore_the_rest_of_rules: bool) -> Self {
        self.body.ignore_the_rest_of_rules = ignore_the_rest_of_rules;
        self
    }

    /// 设置规则名称（必填，1～255 字符）。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = name.into();
        self
    }

    /// 设置是否启用（必填）。
    pub fn is_enable(mut self, is_enable: bool) -> Self {
        self.body.is_enable = is_enable;
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<CreateMailboxRuleResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateMailboxRuleResponse> {
        validate_required!(self.user_mailbox_id, "user_mailbox_id 不能为空");
        validate_required!(self.body.name, "name 不能为空");
        validate_required!(self.body.condition.items, "condition.items 不能为空");
        validate_required!(self.body.action.items, "action.items 不能为空");
        // match_type 必填且取值范围 1～2（数值字段无 Validatable 实现，按 logout 范式手动校验）
        if !(1..=2).contains(&self.body.condition.match_type) {
            return Err(error::validation_error(
                "match_type",
                "match_type 不能为空，可选值：1（满足所有条件）、2（满足任意条件）",
            ));
        }
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/rules",
            self.user_mailbox_id
        );
        let req: ApiRequest<CreateMailboxRuleResponse> =
            ApiRequest::post(&path).body(serialize_params(&self.body, "创建收信规则")?);

        Transport::request_typed(req, &self.config, Some(option), "创建收信规则").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：POST .../user_mailboxes/{umb}/rules → CreateMailboxRuleResponse 解析，请求体对齐官方示例。
    #[tokio::test]
    async fn test_create_mailbox_rule_sends_official_body() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/mail/v1/user_mailboxes/umb_001/rules"))
            .and(body_json(json!({
                "condition": {
                    "match_type": 1,
                    "items": [
                        { "type": 1, "operator": 1, "input": "hello@world.com" }
                    ]
                },
                "action": {
                    "items": [
                        { "type": 1, "input": "283412371233" }
                    ]
                },
                "ignore_the_rest_of_rules": false,
                "name": "将李三的邮件标记为垃圾邮件",
                "is_enable": false
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "rule_id": "r_001" } }
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

        let resp = CreateMailboxRuleRequest::new(config, "umb_001")
            .condition(MailboxRuleCondition {
                match_type: 1,
                items: vec![MailboxRuleConditionItem {
                    condition_type: 1,
                    operator: Some(1),
                    input: Some("hello@world.com".to_string()),
                }],
            })
            .action(MailboxRuleAction {
                items: vec![MailboxRuleActionItem {
                    action_type: 1,
                    input: Some("283412371233".to_string()),
                }],
            })
            .ignore_the_rest_of_rules(false)
            .name("将李三的邮件标记为垃圾邮件")
            .is_enable(false)
            .execute()
            .await
            .expect("创建收信规则应成功");
        assert_eq!(resp.data.unwrap()["rule_id"], "r_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/mail/v1/user_mailboxes/umb_001/rules"
        );
    }

    /// 失败路径：未设置 name 时本地校验报错，不发起请求。
    #[tokio::test]
    async fn test_create_mailbox_rule_rejects_missing_name() {
        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .build(),
        );

        let result = CreateMailboxRuleRequest::new(config, "umb_001")
            .execute()
            .await;
        assert!(result.is_err());
    }
}
