//! Bitable 更新自定义角色
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-role/update>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use super::models::{BlockRole, Role, TableRole};

/// 更新自定义角色请求。
#[derive(Debug, Clone)]
pub struct UpdateAppRoleRequest {
    config: Config,
    app_token: String,
    role_id: String,
    role_name: String,
    table_roles: Vec<TableRole>,
    block_roles: Option<Vec<BlockRole>>,
}

impl UpdateAppRoleRequest {
    /// 创建新的自定义角色更新请求。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            role_id: String::new(),
            role_name: String::new(),
            table_roles: Vec::new(),
            block_roles: None,
        }
    }

    /// 设置多维表格 token。
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置角色 ID。
    pub fn role_id(mut self, role_id: String) -> Self {
        self.role_id = role_id;
        self
    }

    /// 设置角色名称。
    pub fn role_name(mut self, role_name: String) -> Self {
        self.role_name = role_name;
        self
    }

    /// 设置表级权限列表。
    pub fn table_roles(mut self, table_roles: Vec<TableRole>) -> Self {
        self.table_roles = table_roles;
        self
    }

    /// 设置仪表盘权限列表。
    pub fn block_roles(mut self, block_roles: Vec<BlockRole>) -> Self {
        self.block_roles = Some(block_roles);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<UpdateAppRoleResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UpdateAppRoleResponse> {
        validate_required!(self.app_token.trim(), "app_token");
        validate_required!(self.role_id.trim(), "role_id");
        validate_required!(self.role_name.trim(), "role_name");
        validate_required!(self.table_roles, "table_roles");
        if self.table_roles.len() > 100 {
            return Err(openlark_core::error::validation_error(
                "table_roles",
                "table_roles 最多 100 项",
            ));
        }
        if let Some(ref block_roles) = self.block_roles
            && block_roles.len() > 100
        {
            return Err(openlark_core::error::validation_error(
                "block_roles",
                "block_roles 最多 100 项",
            ));
        }

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::RoleUpdate(self.app_token.clone(), self.role_id);

        // #439: method 来自 catalog
        let api_request: ApiRequest<UpdateAppRoleResponse> = api_endpoint
            .to_request::<UpdateAppRoleResponse>()
            .body(serde_json::to_vec(&UpdateAppRoleRequestBody {
                role_name: self.role_name,
                table_roles: self.table_roles,
                block_roles: self.block_roles,
            })?);

        Transport::request_typed(
            api_request,
            &self.config,
            Some(option),
            "Bitable 更新自定义角色",
        )
        .await
    }
}

/// 更新自定义角色请求体（内部使用）。
#[derive(Debug, Serialize, Default)]
pub struct UpdateAppRoleRequestBody {
    role_name: String,
    table_roles: Vec<TableRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_roles: Option<Vec<BlockRole>>,
}

/// 更新自定义角色响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateAppRoleResponse {
    /// 更新后的角色信息。
    pub role: Role,
}

impl ApiResponseTrait for UpdateAppRoleResponse {
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

    /// 端到端：PUT .../roles/{role_id} → UpdateAppRoleResponse。
    #[tokio::test]
    async fn test_update_app_role_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/open-apis/bitable/v1/apps/app001/roles/role001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "role": { "role_name": "角色名", "table_roles": [] } }
            })))
            .mount(&server).await;
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();
        UpdateAppRoleRequest::new(config)
            .app_token("app001".into())
            .role_id("role001".into())
            .role_name("角色名".into())
            .table_roles(vec![TableRole {
                table_perm: 0,
                table_name: None,
                table_id: None,
                rec_rule: None,
                field_perm: None,
                allow_add_record: None,
                allow_delete_record: None,
            }])
            .execute()
            .await
            .expect("更新角色应成功");
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/bitable/v1/apps/app001/roles/role001"
        );
    }
}
