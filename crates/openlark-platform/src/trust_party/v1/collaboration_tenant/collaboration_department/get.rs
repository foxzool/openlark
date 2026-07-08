//! 获取关联组织部门详情
//!
//! 文档: <https://open.feishu.cn/document/trust_party-v1/-collaboraiton-organization/get-2>
//! docPath: <https://open.feishu.cn/document/trust_party-v1/-collaboraiton-organization/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 获取关联组织部门详情 Builder
#[derive(Debug, Clone)]
pub struct CollaborationDepartmentGetRequestBuilder {
    config: Config,
    target_tenant_key: String,
    target_department_id: String,
}

impl CollaborationDepartmentGetRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            target_tenant_key: String::new(),
            target_department_id: String::new(),
        }
    }

    /// 设置目标租户 key
    pub fn target_tenant_key(mut self, key: impl Into<String>) -> Self {
        self.target_tenant_key = key.into();
        self
    }

    /// 设置目标部门 ID
    pub fn target_department_id(mut self, id: impl Into<String>) -> Self {
        self.target_department_id = id.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CollaborationDepartmentGetResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CollaborationDepartmentGetResponse> {
        let url = format!(
            "/open-apis/trust_party/v1/collaboration_tenants/{}/collaboration_departments/{}",
            self.target_tenant_key, self.target_department_id
        );

        let req: ApiRequest<CollaborationDepartmentGetResponse> = ApiRequest::get(&url);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 关联组织部门详情响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollaborationDepartmentGetResponse {
    /// 部门 ID
    #[serde(rename = "department_id")]
    pub department_id: String,
    /// 部门名称
    pub name: String,
    /// 父部门 ID
    #[serde(rename = "parent_department_id")]
    pub parent_department_id: Option<String>,
}

impl ApiResponseTrait for CollaborationDepartmentGetResponse {}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(
    note = "renamed to CollaborationDepartmentGetRequestBuilder, will be removed in v1.0 (#271)"
)]
pub type CollaborationDepartmentGetBuilder = CollaborationDepartmentGetRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../collaboration_tenants/{key}/collaboration_departments/{did} → 强类型 CollaborationDepartmentGetResponse。
    #[tokio::test]
    async fn test_get_collaboration_department_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/trust_party/v1/collaboration_tenants/tk_001/collaboration_departments/d_001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "department_id": "d_001",
                    "name": "工程部",
                    "parent_department_id": "d_000"
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

        let resp = CollaborationDepartmentGetRequestBuilder::new(config)
            .target_tenant_key("tk_001")
            .target_department_id("d_001")
            .execute()
            .await
            .expect("获取关联组织部门详情应成功");
        assert_eq!(resp.department_id, "d_001");
        assert_eq!(resp.name, "工程部");
        assert_eq!(resp.parent_department_id.as_deref(), Some("d_000"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/trust_party/v1/collaboration_tenants/tk_001/collaboration_departments/d_001"
        );
    }
}
