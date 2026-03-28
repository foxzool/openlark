//! Bitable 新增自定义角色
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-role/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use super::models::{BlockRole, Role, TableRole};
use crate::common::api_utils::*;

/// 新增自定义角色请求
#[derive(Debug, Clone)]
pub struct CreateAppRoleRequest {
    config: Config,
    app_token: String,
    role_name: String,
    table_roles: Vec<TableRole>,
    block_roles: Option<Vec<BlockRole>>,
}

impl CreateAppRoleRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            role_name: String::new(),
            table_roles: Vec::new(),
            block_roles: None,
        }
    }

    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    pub fn role_name(mut self, role_name: String) -> Self {
        self.role_name = role_name;
        self
    }

    pub fn table_roles(mut self, table_roles: Vec<TableRole>) -> Self {
        self.table_roles = table_roles;
        self
    }

    pub fn block_roles(mut self, block_roles: Vec<BlockRole>) -> Self {
        self.block_roles = Some(block_roles);
        self
    }

    pub async fn execute(self) -> SDKResult<CreateAppRoleResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateAppRoleResponse> {
        // === 必填字段验证 ===
        validate_required!(self.app_token.trim(), "应用令牌不能为空");
        validate_required!(self.role_name.trim(), "角色名称不能为空");
        validate_required!(self.table_roles, "表格角色列表不能为空");

        // === 边界值验证 ===
        if self.table_roles.len() > 100 {
            return Err(openlark_core::error::validation_error(
                "table_roles",
                "table_roles 最多 100 项",
            ));
        }
        if let Some(ref block_roles) = self.block_roles {
            if block_roles.len() > 100 {
                return Err(openlark_core::error::validation_error(
                    "block_roles",
                    "block_roles 最多 100 项",
                ));
            }
        }

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::RoleCreate(self.app_token.clone());

        let api_request: ApiRequest<CreateAppRoleResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(
                &CreateAppRoleRequestBody {
                    role_name: self.role_name,
                    table_roles: self.table_roles,
                    block_roles: self.block_roles,
                },
                "新增角色",
            )?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "新增角色")
    }
}

/// 新增自定义角色请求体（内部使用）
#[derive(Serialize)]
pub struct CreateAppRoleRequestBody {
    role_name: String,
    table_roles: Vec<TableRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_roles: Option<Vec<BlockRole>>,
}

/// 新增自定义角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateAppRoleResponse {
    pub role: Role,
}

impl ApiResponseTrait for CreateAppRoleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::test_runtime;

    #[test]
    fn test_empty_app_token() {
        let config = Config::default();
        let request = CreateAppRoleRequest::new(config)
            .app_token("".to_string())
            .role_name("角色名".to_string())
            .table_roles(vec![]);
        let rt = test_runtime();
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_role_name() {
        let config = Config::default();
        let request = CreateAppRoleRequest::new(config)
            .app_token("app_token".to_string())
            .role_name("".to_string())
            .table_roles(vec![]);
        let rt = test_runtime();
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_table_roles() {
        let config = Config::default();
        let request = CreateAppRoleRequest::new(config)
            .app_token("app_token".to_string())
            .role_name("角色名".to_string())
            .table_roles(vec![]);
        let rt = test_runtime();
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(CreateAppRoleResponse::data_format(), ResponseFormat::Data);
    }
}
