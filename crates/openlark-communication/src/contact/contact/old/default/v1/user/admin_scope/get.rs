//! 获取应用管理员管理范围
//!
//! docPath: <https://open.feishu.cn/document/server-docs/application-v6/admin/obtain-an-app-admin%E2%80%99s-management-permissions>

use openlark_core::{SDKResult, api::ApiRequest, config::Config, http::Transport};

use crate::endpoints::CONTACT_V1_USER_ADMIN_SCOPE_GET;

/// 获取应用管理员管理范围
pub struct GetAdminScopeRequest {
    /// 配置信息。
    config: Config,
}

impl GetAdminScopeRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/application-v6/admin/obtain-an-app-admin%E2%80%99s-management-permissions>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // url: GET:/open-apis/contact/v1/user/admin_scope/get
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(CONTACT_V1_USER_ADMIN_SCOPE_GET);
        Transport::request_typed(req, &self.config, Some(option), "获取应用管理员管理范围").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/contact/v1/user/admin_scope/get
    #[tokio::test]
    async fn test_get_admin_scope_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/contact/v1/user/admin_scope/get"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {}
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        GetAdminScopeRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
