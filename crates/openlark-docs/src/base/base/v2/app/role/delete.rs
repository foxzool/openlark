//! 删除自定义角色
//!
//! docPath: <https://open.feishu.cn/document/docs/bitable-v1/advanced-permission/app-role/delete-2>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::{BaseApiV2, CatalogEndpoint};
use crate::common::api_utils::*;

/// 删除自定义角色请求。
#[derive(Debug, Clone)]
pub struct Delete {
    config: Config,
    app_token: String,
    role_id: String,
}

impl Delete {
    /// 创建新的删除请求。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            role_id: String::new(),
        }
    }

    /// 设置应用 token。
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 设置角色 ID。
    pub fn role_id(mut self, role_id: impl Into<String>) -> Self {
        self.role_id = role_id.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<DeleteResp> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteResp> {
        validate_required!(self.app_token.trim(), "app_token 不能为空");
        validate_required!(self.role_id.trim(), "role_id 不能为空");

        let api_endpoint = BaseApiV2::RoleDelete(self.app_token, self.role_id);

        let api_request: ApiRequest<DeleteResp> = api_endpoint.to_request();

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除自定义角色")
    }
}

/// 删除自定义角色响应（data 为空对象）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct DeleteResp {}

impl ApiResponseTrait for DeleteResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：DELETE .../roles/{role_id} → DeleteResp。
    #[tokio::test]
    async fn test_delete_role_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/base/v2/apps/app001/roles/role001"))
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
        Delete::new(config)
            .app_token("app001")
            .role_id("role001")
            .execute()
            .await
            .expect("删除角色应成功");
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/base/v2/apps/app001/roles/role001"
        );
    }

    #[test]
    fn test_delete_role_uses_delete_from_catalog_438() {
        let ep = BaseApiV2::RoleDelete("app".into(), "role001".into());
        let req: openlark_core::api::ApiRequest<DeleteResp> = ep.to_request();
        assert_eq!(req.method(), &openlark_core::api::HttpMethod::Delete);
    }
}
