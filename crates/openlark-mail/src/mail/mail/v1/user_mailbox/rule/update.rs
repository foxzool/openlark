//! 更新收信规则

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 更新收信规则的请求。
#[derive(Debug, Clone)]
pub struct UpdateMailboxRuleRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
    rule_id: String,
    body: UpdateMailboxRuleBody,
}

/// 更新收信规则的请求体（官方必填：condition/action/ignore_the_rest_of_rules/name/is_enable）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMailboxRuleBody {
    /// 匹配条件。
    pub condition: MailboxRuleCondition,
    /// 匹配命中后的操作。
    pub action: MailboxRuleAction,
    /// 是否终止规则。
    pub ignore_the_rest_of_rules: bool,
    /// 规则名称（1-255 字符）。
    pub name: String,
    /// 是否启用。
    pub is_enable: bool,
}

/// 收信规则匹配条件（rule_condition）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MailboxRuleCondition {
    /// 匹配类型：1-满足所有条件；2-满足任意条件。
    pub match_type: i32,
    /// 匹配规则列表（1-32 条）。
    pub items: Vec<MailboxRuleConditionItem>,
}

/// 收信规则匹配条件项（rule_condition_item）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MailboxRuleConditionItem {
    /// 匹配条件左值：1-发件人地址；2-收件人地址；3-抄送地址；4-收件人或抄送地址；6-主题；7-正文；8-附件名字；9-附件类型；10-任意地址；12-所有邮件；13-是外部邮件；14-是垃圾邮件；15-不是垃圾邮件；16-有附件。
    pub r#type: i32,
    /// 匹配条件操作符：1-包含；2-不包含；3-开头是；4-结尾是；5-是；6-不是；7-包含自己；10-为空。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<i32>,
    /// 匹配条件右值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
}

/// 收信规则命中操作（rule_action）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MailboxRuleAction {
    /// 匹配命中规则后的操作列表（1-32 条）。
    pub items: Vec<MailboxRuleActionItem>,
}

/// 收信规则命中操作项（rule_action_item）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MailboxRuleActionItem {
    /// 操作类型：1-归档；2-删除邮件；3-标记为已读；4-移至垃圾邮件；5-不移至垃圾邮件；9-添加旗标；10-不弹出通知；11-移至用户文件夹。
    pub r#type: i32,
    /// 当 type 为移至用户文件夹（11）时，该字段填文件夹的 id。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
}

/// 更新收信规则的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMailboxRuleResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for UpdateMailboxRuleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UpdateMailboxRuleRequest {
    /// 创建请求实例。
    pub fn new(
        config: Arc<Config>,
        user_mailbox_id: impl Into<String>,
        rule_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            user_mailbox_id: user_mailbox_id.into(),
            rule_id: rule_id.into(),
            body: UpdateMailboxRuleBody::default(),
        }
    }

    /// 设置规则名称（必填，1-255 字符）。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = name.into();
        self
    }

    /// 设置是否启用（必填）。
    pub fn is_enable(mut self, is_enable: bool) -> Self {
        self.body.is_enable = is_enable;
        self
    }

    /// 设置是否终止规则（必填）。
    pub fn ignore_the_rest_of_rules(mut self, ignore_the_rest_of_rules: bool) -> Self {
        self.body.ignore_the_rest_of_rules = ignore_the_rest_of_rules;
        self
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

    /// 执行更新收信规则请求。
    pub async fn execute(self) -> SDKResult<UpdateMailboxRuleResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UpdateMailboxRuleResponse> {
        validate_required!(self.body.name, "规则名称不能为空");
        validate_required_list!(
            self.body.condition.items,
            32,
            "condition.items 不能为空且不能超过 32 条"
        );
        validate_required_list!(
            self.body.action.items,
            32,
            "action.items 不能为空且不能超过 32 条"
        );
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/rules/{}",
            self.user_mailbox_id, self.rule_id
        );
        let body = serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::validation_error("更新收信规则", format!("请求体序列化失败: {e}"))
        })?;
        let req: ApiRequest<UpdateMailboxRuleResponse> = ApiRequest::put(&path).body(body);

        Transport::request_typed(req, &self.config, Some(option), "更新收信规则").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：PUT .../user_mailboxes/{umb}/rules/{rule} → 官方请求体形状（condition/action/ignore_the_rest_of_rules/name/is_enable）。
    #[tokio::test]
    async fn test_update_mailbox_rule_sends_official_body() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_partial_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path(
                "/open-apis/mail/v1/user_mailboxes/umb_001/rules/r_001",
            ))
            .and(body_partial_json(json!({
                "condition": {
                    "match_type": 1,
                    "items": [{ "type": 1, "operator": 1, "input": "hello@world.com" }]
                },
                "action": {
                    "items": [{ "type": 11, "input": "283412371233" }]
                },
                "ignore_the_rest_of_rules": false,
                "name": "将李三的邮件标记为垃圾邮件",
                "is_enable": false
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {}
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

        let resp = UpdateMailboxRuleRequest::new(config, "umb_001", "r_001")
            .name("将李三的邮件标记为垃圾邮件")
            .is_enable(false)
            .ignore_the_rest_of_rules(false)
            .condition(MailboxRuleCondition {
                match_type: 1,
                items: vec![MailboxRuleConditionItem {
                    r#type: 1,
                    operator: Some(1),
                    input: Some("hello@world.com".to_string()),
                }],
            })
            .action(MailboxRuleAction {
                items: vec![MailboxRuleActionItem {
                    r#type: 11,
                    input: Some("283412371233".to_string()),
                }],
            })
            .execute()
            .await
            .expect("更新收信规则应成功");
        assert!(resp.data.is_none());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/mail/v1/user_mailboxes/umb_001/rules/r_001"
        );
        // 可选字段 input/operator 按 skip_serializing_if 序列化，必填字段全部下发
        let sent: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为合法 JSON");
        assert_eq!(sent["condition"]["match_type"], 1);
        assert_eq!(sent["condition"]["items"][0]["type"], 1);
        assert_eq!(sent["action"]["items"][0]["type"], 11);
        assert_eq!(sent["action"]["items"][0]["input"], "283412371233");
        assert_eq!(sent["ignore_the_rest_of_rules"], false);
        assert_eq!(sent["name"], "将李三的邮件标记为垃圾邮件");
        assert_eq!(sent["is_enable"], false);
    }

    /// 缺必填字段（name / condition.items / action.items）应快速失败，不发请求。
    #[tokio::test]
    async fn test_update_mailbox_rule_rejects_missing_required_fields() {
        let config = Arc::new(Config::default());
        let err = UpdateMailboxRuleRequest::new(config, "umb_001", "r_001")
            .execute()
            .await
            .expect_err("缺少必填字段应报错");
        assert!(err.to_string().contains("规则名称不能为空"));

        let config = Arc::new(Config::default());
        let err = UpdateMailboxRuleRequest::new(config, "umb_001", "r_001")
            .name("规则")
            .execute()
            .await
            .expect_err("缺少匹配条件应报错");
        assert!(err.to_string().contains("condition.items"));
    }
}
