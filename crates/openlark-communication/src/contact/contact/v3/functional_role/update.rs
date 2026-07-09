//! 修改角色名称
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/functional_role/update>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::{
        api_utils::{extract_response_data, serialize_params},
        models::EmptyData,
    },
    endpoints::CONTACT_V3_FUNCTIONAL_ROLES,
};

/// 修改角色名称请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFunctionalRoleBody {
    /// 角色名称。
    pub role_name: String,
}

/// 修改角色名称请求
pub struct UpdateFunctionalRoleRequest {
    /// 配置信息。
    config: Config,
    /// 角色 ID。
    role_id: String,
}

impl UpdateFunctionalRoleRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            role_id: String::new(),
        }
    }

    /// 角色 ID（路径参数）
    pub fn role_id(mut self, role_id: impl Into<String>) -> Self {
        self.role_id = role_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/functional_role/update>
    pub async fn execute(self, body: UpdateFunctionalRoleBody) -> SDKResult<EmptyData> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: UpdateFunctionalRoleBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        validate_required!(self.role_id, "role_id 不能为空");
        validate_required!(body.role_name, "role_name 不能为空");

        // url: PUT:/open-apis/contact/v3/functional_roles/:role_id
        let req: ApiRequest<EmptyData> =
            ApiRequest::put(format!("{}/{}", CONTACT_V3_FUNCTIONAL_ROLES, self.role_id))
                .body(serialize_params(&body, "修改角色名称")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "修改角色名称")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PUT /open-apis/contact/v3/functional_roles/test001
    #[tokio::test]
    async fn test_update_functional_role_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/open-apis/contact/v3/functional_roles/test001"))
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

        let body: UpdateFunctionalRoleBody =
            serde_json::from_value(json!({ "role_name": "test001" })).expect("body 构造");
        UpdateFunctionalRoleRequest::new(config)
            .role_id("test001".to_string())
            .execute(body)
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
