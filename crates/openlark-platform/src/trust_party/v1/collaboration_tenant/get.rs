//! 获取关联组织详情
//!
//! 文档: <https://open.feishu.cn/document/trust_party-v1/-collaboraiton-organization/get>
//! docPath: <https://open.feishu.cn/document/trust_party-v1/-collaboraiton-organization/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 获取关联组织详情 Builder
#[derive(Debug, Clone)]
pub struct CollaborationTenantGetRequestBuilder {
    config: Config,
    target_tenant_key: String,
}

impl CollaborationTenantGetRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            target_tenant_key: String::new(),
        }
    }

    /// 设置目标租户 key
    pub fn target_tenant_key(mut self, key: impl Into<String>) -> Self {
        self.target_tenant_key = key.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CollaborationTenantGetResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CollaborationTenantGetResponse> {
        let url = format!(
            "/open-apis/trust_party/v1/collaboration_tenants/{}",
            self.target_tenant_key
        );

        let req: ApiRequest<CollaborationTenantGetResponse> = ApiRequest::get(&url);
        Transport::request_typed(req, &self.config, Some(option), "Operation").await
    }
}

/// 关联组织详情响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollaborationTenantGetResponse {
    /// 租户 key
    #[serde(rename = "tenant_key")]
    pub tenant_key: String,
    /// 租户名称
    #[serde(rename = "tenant_name")]
    pub tenant_name: String,
    /// 关联状态
    pub status: String,
}

impl ApiResponseTrait for CollaborationTenantGetResponse {}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(
    note = "renamed to CollaborationTenantGetRequestBuilder, will be removed in v1.0 (#271)"
)]
pub type CollaborationTenantGetBuilder = CollaborationTenantGetRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../trust_party/v1/collaboration_tenants/{key} → 强类型 CollaborationTenantGetResponse。
    #[tokio::test]
    async fn test_get_collaboration_tenant_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/trust_party/v1/collaboration_tenants/tk_001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "tenant_key": "tk_001",
                    "tenant_name": "acme",
                    "status": "ACTIVE"
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

        let resp = CollaborationTenantGetRequestBuilder::new(config)
            .target_tenant_key("tk_001")
            .execute()
            .await
            .expect("获取关联组织详情应成功");
        assert_eq!(resp.tenant_key, "tk_001");
        assert_eq!(resp.tenant_name, "acme");
        assert_eq!(resp.status, "ACTIVE");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/trust_party/v1/collaboration_tenants/tk_001"
        );
    }
}
