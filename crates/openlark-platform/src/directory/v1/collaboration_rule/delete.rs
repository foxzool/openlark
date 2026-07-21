//! 删除可搜可见规则
//!
//! 文档: <https://open.feishu.cn/document/trust_party-v1/searchable-and-visible-rules/delete>
//! docPath: <https://open.feishu.cn/document/trust_party-v1/searchable-and-visible-rules/delete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 删除可搜可见规则 Builder
#[derive(Debug, Clone)]
pub struct CollaborationRuleDeleteRequestBuilder {
    config: Config,
    /// 规则 ID
    collaboration_rule_id: String,
}

impl CollaborationRuleDeleteRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, collaboration_rule_id: impl Into<String>) -> Self {
        Self {
            config,
            collaboration_rule_id: collaboration_rule_id.into(),
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
        let url = format!(
            "/open-apis/directory/v1/collaboration_rules/{}",
            self.collaboration_rule_id
        );

        let req: ApiRequest<CollaborationRuleDeleteResponse> = ApiRequest::delete(&url);
        Transport::request_typed(req, &self.config, Some(option), "删除可搜可见规则").await
    }
}

/// 删除可搜可见规则响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollaborationRuleDeleteResponse {
    /// 规则 ID
    #[serde(rename = "collaboration_rule_id")]
    pub collaboration_rule_id: String,
    /// 结果消息
    #[serde(rename = "message")]
    pub message: String,
}

impl ApiResponseTrait for CollaborationRuleDeleteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(
    note = "renamed to CollaborationRuleDeleteRequestBuilder, will be removed in v1.0 (#271)"
)]
pub type CollaborationRuleDeleteBuilder = CollaborationRuleDeleteRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：DELETE .../directory/v1/collaboration_rules/{id} → 强类型 CollaborationRuleDeleteResponse。
    #[tokio::test]
    async fn test_delete_collaboration_rule_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/directory/v1/collaboration_rules/rule_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "collaboration_rule_id": "rule_001",
                    "message": "deleted"
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

        let resp = CollaborationRuleDeleteRequestBuilder::new(config, "rule_001")
            .execute()
            .await
            .expect("删除可搜可见规则应成功");
        assert_eq!(resp.collaboration_rule_id, "rule_001");
        assert_eq!(resp.message, "deleted");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/directory/v1/collaboration_rules/rule_001"
        );
        assert_eq!(received[0].method, "DELETE");
    }
}
