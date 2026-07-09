//! 创建角色
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/functional_role/create>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    contact::contact::v3::functional_role::models::CreateFunctionalRoleResponse,
    endpoints::CONTACT_V3_FUNCTIONAL_ROLES,
};

/// 创建角色请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFunctionalRoleBody {
    /// 角色名称。
    pub role_name: String,
}

/// 创建角色请求
pub struct CreateFunctionalRoleRequest {
    /// 配置信息。
    config: Config,
}

impl CreateFunctionalRoleRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/functional_role/create>
    pub async fn execute(
        self,
        body: CreateFunctionalRoleBody,
    ) -> SDKResult<CreateFunctionalRoleResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: CreateFunctionalRoleBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateFunctionalRoleResponse> {
        validate_required!(body.role_name, "role_name 不能为空");

        // url: POST:/open-apis/contact/v3/functional_roles
        let req: ApiRequest<CreateFunctionalRoleResponse> =
            ApiRequest::post(CONTACT_V3_FUNCTIONAL_ROLES)
                .body(serialize_params(&body, "创建角色")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "创建角色")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/contact/v3/functional_roles
    #[tokio::test]
    async fn test_create_functional_role_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/contact/v3/functional_roles"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "role_id": "test001" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let body: CreateFunctionalRoleBody =
            serde_json::from_value(json!({ "role_name": "test001" })).expect("body 构造");
        CreateFunctionalRoleRequest::new(config)
            .execute(body)
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
