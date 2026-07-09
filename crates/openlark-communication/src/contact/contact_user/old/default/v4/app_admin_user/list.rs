//! 查询应用管理员列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/application-v6/admin/query-app-administrator-list>

use openlark_core::{SDKResult, api::ApiRequest, config::Config, http::Transport};

use crate::{common::api_utils::extract_response_data, endpoints::USER_V4_APP_ADMIN_USER_LIST};

/// 查询应用管理员列表请求
pub struct ListAppAdminUserRequest {
    config: Config,
}

impl ListAppAdminUserRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/application-v6/admin/query-app-administrator-list>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // url: GET:/open-apis/user/v4/app_admin_user/list
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(USER_V4_APP_ADMIN_USER_LIST);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询应用管理员列表")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/user/v4/app_admin_user/list
    #[tokio::test]
    async fn test_list_app_admin_user_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/user/v4/app_admin_user/list"))
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

        ListAppAdminUserRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
