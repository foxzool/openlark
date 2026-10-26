//! 查询可搜可见规则
//!
//! 文档: <https://open.feishu.cn/document/trust_party-v1/searchable-and-visible-rules/list>
//! docPath: <https://open.feishu.cn/document/trust_party-v1/searchable-and-visible-rules/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 查询可搜可见规则 Builder
#[derive(Debug, Clone)]
pub struct CollaborationRuleListRequestBuilder {
    config: Config,
    /// 对方组织的 tenant key
    target_tenant_key: String,
    /// 分页大小（0~100，默认 100）
    page_size: Option<u32>,
    /// 分页标记（第一次请求不填）
    page_token: Option<String>,
}

impl CollaborationRuleListRequestBuilder {
    /// 创建新的 Builder
    ///
    /// `target_tenant_key` 为对方组织的 tenant key，可通过
    /// 「管理员获取所有关联组织列表」接口获取。
    pub fn new(config: Config, target_tenant_key: impl Into<String>) -> Self {
        Self {
            config,
            target_tenant_key: target_tenant_key.into(),
            page_size: None,
            page_token: None,
        }
    }

    /// 设置分页大小（0~100，默认 100）
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（第一次请求不填，后续传上一次返回的 page_token）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CollaborationRuleListResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CollaborationRuleListResponse> {
        validate_required!(self.target_tenant_key, "target_tenant_key 不能为空");

        let url = "/open-apis/directory/v1/collaboration_rules";

        let req: ApiRequest<CollaborationRuleListResponse> = ApiRequest::get(url)
            .query("target_tenant_key", self.target_tenant_key)
            .query_opt("page_size", self.page_size.map(|v| v.to_string()))
            .query_opt("page_token", self.page_token);
        Transport::request_typed(req, &self.config, Some(option), "查询可搜可见规则").await
    }
}

/// 可搜可见规则信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollaborationRuleInfo {
    /// 规则 ID
    #[serde(rename = "collaboration_rule_id")]
    collaboration_rule_id: String,
    /// 规则名称
    #[serde(rename = "name")]
    name: String,
    /// 搜索可见范围类型
    #[serde(rename = "search_visible_scope_type")]
    search_visible_scope_type: String,
    /// 搜索可见范围用户列表
    #[serde(rename = "search_visible_scope_user_ids")]
    search_visible_scope_user_ids: Vec<String>,
    /// 搜索可见范围部门列表
    #[serde(rename = "search_visible_scope_department_ids")]
    search_visible_scope_department_ids: Vec<String>,
    /// 创建时间
    #[serde(rename = "created_at")]
    created_at: i64,
    /// 更新时间
    #[serde(rename = "updated_at")]
    updated_at: i64,
}

/// 查询可搜可见规则响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollaborationRuleListResponse {
    /// 可搜可见规则列表
    #[serde(rename = "items")]
    pub items: Vec<CollaborationRuleInfo>,
}

impl ApiResponseTrait for CollaborationRuleListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(
    note = "renamed to CollaborationRuleListRequestBuilder, will be removed in v1.0 (#271)"
)]
pub type CollaborationRuleListBuilder = CollaborationRuleListRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET 携带必填 target_tenant_key 查询参数 → 强类型 CollaborationRuleListResponse。
    #[tokio::test]
    async fn test_list_collaboration_rules_sends_required_query() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/directory/v1/collaboration_rules"))
            .and(query_param("target_tenant_key", "test_key"))
            .and(query_param("page_size", "10"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [],
                    "has_more": false
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = CollaborationRuleListRequestBuilder::new(config, "test_key")
            .page_size(10)
            .execute()
            .await
            .expect("查询可搜可见规则应成功");
        assert!(resp.items.is_empty());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/directory/v1/collaboration_rules"
        );
        assert_eq!(received[0].method, "GET");
    }

    /// 必填校验：target_tenant_key 为空时不发请求。
    #[tokio::test]
    async fn test_list_collaboration_rules_validates_required_query() {
        let server = MockServer::start().await;
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let result = CollaborationRuleListRequestBuilder::new(config, " ")
            .execute()
            .await;
        assert!(result.is_err());
        assert!(
            server
                .received_requests()
                .await
                .unwrap_or_default()
                .is_empty()
        );
    }
}
