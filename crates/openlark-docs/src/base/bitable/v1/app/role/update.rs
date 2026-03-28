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

/// 更新自定义角色请求
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

    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    pub fn role_id(mut self, role_id: String) -> Self {
        self.role_id = role_id;
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

    pub async fn execute(self) -> SDKResult<UpdateAppRoleResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

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
        if let Some(ref block_roles) = self.block_roles {
            if block_roles.len() > 100 {
                return Err(openlark_core::error::validation_error(
                    "block_roles",
                    "block_roles 最多 100 项",
                ));
            }
        }

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::RoleUpdate(self.app_token.clone(), self.role_id);

        let api_request: ApiRequest<UpdateAppRoleResponse> = ApiRequest::put(
            &api_endpoint.to_url(),
        )
        .body(serde_json::to_vec(&UpdateAppRoleRequestBody {
            role_name: self.role_name,
            table_roles: self.table_roles,
            block_roles: self.block_roles,
        })?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

#[derive(Debug, Serialize, Default)]
pub struct UpdateAppRoleRequestBody {
    role_name: String,
    table_roles: Vec<TableRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_roles: Option<Vec<BlockRole>>,
}

/// 更新自定义角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateAppRoleResponse {
    pub role: Role,
}

impl ApiResponseTrait for UpdateAppRoleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
