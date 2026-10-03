//! 更新自定义角色
//!
//! docPath: <https://open.feishu.cn/document/docs/bitable-v1/advanced-permission/app-role/update-2>

use crate::base::base::v2::models::AppRole;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::{BaseApiV2, CatalogEndpoint};
use crate::common::api_utils::*;

/// 更新自定义角色
#[derive(Debug)]
pub struct Update {
    config: Config,
    app_token: String,
    role_id: String,
    req: UpdateReq,
}

#[derive(Debug, Serialize, Deserialize)]
/// 更新自定义角色请求体。
pub struct UpdateReq {
    /// 自定义角色的名字
    pub role_name: String,
    /// 数据表权限配置列表（结构按 JSON 透传）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_roles: Option<Vec<serde_json::Value>>,
    /// Block 权限配置列表（结构按 JSON 透传）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_roles: Option<Vec<serde_json::Value>>,
    /// Base 规则（结构按 JSON 透传）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_rule: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
/// 更新自定义角色响应。
pub struct UpdateResp {
    /// 自定义角色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<AppRole>,
}

impl Update {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            role_id: String::new(),
            req: UpdateReq {
                role_name: String::new(),
                table_roles: None,
                block_roles: None,
                base_rule: None,
            },
        }
    }

    /// 应用 token
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 角色ID
    pub fn role_id(mut self, role_id: impl Into<String>) -> Self {
        self.role_id = role_id.into();
        self
    }

    /// 自定义角色的名字
    pub fn role_name(mut self, role_name: impl Into<String>) -> Self {
        self.req.role_name = role_name.into();
        self
    }

    /// 数据表权限配置列表（table_roles）
    pub fn table_roles(mut self, table_roles: Vec<serde_json::Value>) -> Self {
        self.req.table_roles = Some(table_roles);
        self
    }

    /// Block 权限配置列表（block_roles）
    pub fn block_roles(mut self, block_roles: Vec<serde_json::Value>) -> Self {
        self.req.block_roles = Some(block_roles);
        self
    }

    /// Base 规则（base_rule）
    pub fn base_rule(mut self, base_rule: serde_json::Value) -> Self {
        self.req.base_rule = Some(base_rule);
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<UpdateResp> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateResp> {
        validate_required!(self.app_token, "app_token 不能为空");
        validate_required!(self.role_id, "role_id 不能为空");
        validate_required!(self.req.role_name, "role_name 不能为空");
        if self.req.role_name.chars().count() > 100 {
            return Err(openlark_core::error::validation_error(
                "role_name",
                "role_name 长度不能超过 100 字符",
            ));
        }
        if let Some(table_roles) = &self.req.table_roles
            && table_roles.len() > 100
        {
            return Err(openlark_core::error::validation_error(
                "table_roles",
                "table_roles 长度不能超过 100",
            ));
        }

        let api_endpoint = BaseApiV2::RoleUpdate(self.app_token, self.role_id);

        // #438: method 来自 catalog
        let api_request: ApiRequest<UpdateResp> = api_endpoint
            .to_request()
            .body(serialize_params(&self.req, "更新自定义角色")?);

        Transport::request_typed(api_request, &self.config, Some(option), "更新自定义角色").await
    }
}

impl ApiResponseTrait for UpdateResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PUT .../roles/{role_id} → UpdateResp。
    /// 同时断言 method、path、auth（catalog 提供）和响应（#438）。
    #[tokio::test]
    async fn test_update_role_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/open-apis/base/v2/apps/app001/roles/role001"))
            .and(header("Authorization", "Bearer test-tenant-token"))
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
        let option = openlark_core::req_option::RequestOption::builder()
            .tenant_access_token("test-tenant-token")
            .build();
        let resp = Update::new(config)
            .app_token("app001")
            .role_id("role001")
            .role_name("新角色名")
            .execute_with_options(option)
            .await
            .expect("更新角色应成功");
        assert!(resp.role.is_none());
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].method, "PUT");
        assert_eq!(
            received[0].url.path(),
            "/open-apis/base/v2/apps/app001/roles/role001"
        );
        assert_eq!(
            received[0]
                .headers
                .get("authorization")
                .and_then(|h| h.to_str().ok()),
            Some("Bearer test-tenant-token")
        );
    }

    #[test]
    fn test_update_role_uses_put_from_catalog_438() {
        let ep = BaseApiV2::RoleUpdate("app".into(), "role001".into());
        let req: openlark_core::api::ApiRequest<UpdateResp> = ep.to_request();
        assert_eq!(req.method(), &openlark_core::api::HttpMethod::Put);
        assert_eq!(
            req.supported_access_token_types(),
            vec![
                openlark_core::constants::AccessTokenType::User,
                openlark_core::constants::AccessTokenType::Tenant
            ]
        );
    }
}
