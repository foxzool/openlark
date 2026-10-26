//! 更新可搜可见规则
//!
//! 文档: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/collaboration_rule/update>
//! docPath: <https://open.feishu.cn/document/trust_party-v1/searchable-and-visible-rules/update>
//!
//! 官方请求体：subjects / objects 均为必填（collaboration_rule_entities）；
//! 必填查询参数 target_tenant_key；成功响应 data 为空对象。

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    error::CoreError,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use super::CollaborationRuleEntities;

/// 更新可搜可见规则 Builder
#[derive(Debug, Clone)]
pub struct CollaborationRuleUpdateRequestBuilder {
    config: Config,
    /// 规则 ID
    collaboration_rule_id: String,
    /// 对方组织的 tenant key（必填查询参数）
    target_tenant_key: String,
    /// 请求体
    body: CollaborationRuleUpdateBody,
}

impl CollaborationRuleUpdateRequestBuilder {
    /// 创建新的 Builder
    ///
    /// `target_tenant_key` 为对方组织的 tenant key，可通过
    /// 「管理员获取所有关联组织列表」接口获取。
    pub fn new(
        config: Config,
        collaboration_rule_id: impl Into<String>,
        target_tenant_key: impl Into<String>,
    ) -> Self {
        Self {
            config,
            collaboration_rule_id: collaboration_rule_id.into(),
            target_tenant_key: target_tenant_key.into(),
            body: CollaborationRuleUpdateBody {
                subjects: CollaborationRuleEntities::new(),
                objects: CollaborationRuleEntities::new(),
            },
        }
    }

    /// 设置规则主体（必填，需至少包含一个实体）
    pub fn subjects(mut self, subjects: CollaborationRuleEntities) -> Self {
        self.body.subjects = subjects;
        self
    }

    /// 设置规则客体（必填，需至少包含一个实体）
    pub fn objects(mut self, objects: CollaborationRuleEntities) -> Self {
        self.body.objects = objects;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CollaborationRuleUpdateResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CollaborationRuleUpdateResponse> {
        validate_required!(self.collaboration_rule_id, "collaboration_rule_id 不能为空");
        validate_required!(self.target_tenant_key, "target_tenant_key 不能为空");
        validate_entities("subjects", &self.body.subjects)?;
        validate_entities("objects", &self.body.objects)?;

        let url = format!(
            "/open-apis/directory/v1/collaboration_rules/{}",
            self.collaboration_rule_id
        );

        let req: ApiRequest<CollaborationRuleUpdateResponse> = ApiRequest::put(&url)
            .body(serde_json::to_value(&self.body)?)
            .query("target_tenant_key", self.target_tenant_key);
        Transport::request_typed(req, &self.config, Some(option), "更新可搜可见规则").await
    }
}

/// 校验主体/客体至少设置一个实体（官方错误码 2223106 拒绝空实体）。
fn validate_entities(label: &str, entities: &CollaborationRuleEntities) -> SDKResult<()> {
    if entities.is_empty() {
        return Err(CoreError::validation_msg(format!(
            "{label} 需至少设置一个实体（open_user_ids/open_department_ids/open_group_ids）"
        )));
    }
    Ok(())
}

/// 更新可搜可见规则请求体（subjects / objects 均必填）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationRuleUpdateBody {
    /// 规则主体
    pub subjects: CollaborationRuleEntities,
    /// 规则客体
    pub objects: CollaborationRuleEntities,
}

/// 更新可搜可见规则响应（官方 data 为空对象）。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollaborationRuleUpdateResponse {}

impl ApiResponseTrait for CollaborationRuleUpdateResponse {}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(
    note = "renamed to CollaborationRuleUpdateRequestBuilder, will be removed in v1.0 (#271)"
)]
pub type CollaborationRuleUpdateBuilder = CollaborationRuleUpdateRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PUT 官方 subjects/objects 请求体 + 必填查询参数 → 成功响应 data 为空对象。
    #[tokio::test]
    async fn test_update_collaboration_rule_sends_official_body() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/open-apis/directory/v1/collaboration_rules/rule_001"))
            .and(query_param("target_tenant_key", "test_key"))
            .and(body_json(json!({
                "subjects": {
                    "open_user_ids": ["od-112121"]
                },
                "objects": {
                    "open_group_ids": ["od-12121"]
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {}
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let _resp = CollaborationRuleUpdateRequestBuilder::new(config, "rule_001", "test_key")
            .subjects(CollaborationRuleEntities::new().with_open_user_ids(vec!["od-112121"]))
            .objects(CollaborationRuleEntities::new().with_open_group_ids(vec!["od-12121"]))
            .execute()
            .await
            .expect("更新可搜可见规则应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/directory/v1/collaboration_rules/rule_001"
        );
        assert_eq!(received[0].method, "PUT");
    }

    /// 必填校验：target_tenant_key 为空时不发请求。
    #[tokio::test]
    async fn test_update_collaboration_rule_validates_required_query() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let result = CollaborationRuleUpdateRequestBuilder::new(config, "rule_001", " ")
            .subjects(CollaborationRuleEntities::new().with_open_user_ids(vec!["od-1"]))
            .objects(CollaborationRuleEntities::new().with_open_user_ids(vec!["od-2"]))
            .execute()
            .await;
        assert!(result.is_err());
    }
}
