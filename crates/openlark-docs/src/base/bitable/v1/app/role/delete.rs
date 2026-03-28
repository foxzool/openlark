//! Bitable 删除自定义角色
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-role/delete>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 删除自定义角色请求
#[derive(Debug, Clone)]
pub struct DeleteAppRoleRequest {
    config: Config,
    app_token: String,
    role_id: String,
}

impl DeleteAppRoleRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            role_id: String::new(),
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

    pub async fn execute(self) -> SDKResult<DeleteAppRoleResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteAppRoleResponse> {
        // === 必填字段验证 ===
        validate_required!(self.app_token.trim(), "app_token");
        validate_required!(self.role_id.trim(), "role_id");

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::RoleDelete(self.app_token, self.role_id);

        let api_request: ApiRequest<DeleteAppRoleResponse> =
            ApiRequest::delete(&api_endpoint.to_url());

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

/// 删除自定义角色响应（data 为空对象）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeleteAppRoleResponse {}

impl ApiResponseTrait for DeleteAppRoleResponse {
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
        let request = DeleteAppRoleRequest::new(config)
            .app_token("".to_string())
            .role_id("role_id".to_string());
        let rt = test_runtime();
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("app_token"));
    }

    #[test]
    fn test_empty_role_id() {
        let config = Config::default();
        let request = DeleteAppRoleRequest::new(config)
            .app_token("app_token".to_string())
            .role_id("".to_string());
        let rt = test_runtime();
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("role_id"));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(DeleteAppRoleResponse::data_format(), ResponseFormat::Data);
    }
}
