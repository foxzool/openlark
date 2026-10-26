//! 删除可搜可见规则
//!
//! 文档: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/directory-v1/collaboration_rule/delete>
//! docPath: <https://open.feishu.cn/document/trust_party-v1/searchable-and-visible-rules/delete>
//!
//! 无请求体；必填查询参数 target_tenant_key；成功响应 data 为空对象。

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 删除可搜可见规则 Builder
#[derive(Debug, Clone)]
pub struct CollaborationRuleDeleteRequestBuilder {
    config: Config,
    /// 规则 ID
    collaboration_rule_id: String,
    /// 对方组织的 tenant key（必填查询参数）
    target_tenant_key: String,
}

impl CollaborationRuleDeleteRequestBuilder {
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
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CollaborationRuleDeleteResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CollaborationRuleDeleteResponse> {
        validate_required!(self.collaboration_rule_id, "collaboration_rule_id 不能为空");
        validate_required!(self.target_tenant_key, "target_tenant_key 不能为空");

        let url = format!(
            "/open-apis/directory/v1/collaboration_rules/{}",
            self.collaboration_rule_id
        );

        let req: ApiRequest<CollaborationRuleDeleteResponse> =
            ApiRequest::delete(&url).query("target_tenant_key", self.target_tenant_key);
        Transport::request_typed(req, &self.config, Some(option), "删除可搜可见规则").await
    }
}

/// 删除可搜可见规则响应（官方 data 为空对象）。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollaborationRuleDeleteResponse {}

impl ApiResponseTrait for CollaborationRuleDeleteResponse {}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(
    note = "renamed to CollaborationRuleDeleteRequestBuilder, will be removed in v1.0 (#271)"
)]
pub type CollaborationRuleDeleteBuilder = CollaborationRuleDeleteRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：DELETE 携带必填 target_tenant_key 查询参数 → 成功响应 data 为空对象。
    #[tokio::test]
    async fn test_delete_collaboration_rule_sends_required_query() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/directory/v1/collaboration_rules/rule_001"))
            .and(query_param("target_tenant_key", "test_key"))
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

        let _resp = CollaborationRuleDeleteRequestBuilder::new(config, "rule_001", "test_key")
            .execute()
            .await
            .expect("删除可搜可见规则应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/directory/v1/collaboration_rules/rule_001"
        );
        assert_eq!(received[0].method, "DELETE");
    }

    /// 必填校验：target_tenant_key 为空时不发请求。
    #[tokio::test]
    async fn test_delete_collaboration_rule_validates_required_query() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let result = CollaborationRuleDeleteRequestBuilder::new(config, "rule_001", " ")
            .execute()
            .await;
        assert!(result.is_err());
    }
}
