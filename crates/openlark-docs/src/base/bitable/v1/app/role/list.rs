//! Bitable 列出自定义角色
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-role/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use super::models::Role;

/// 列出自定义角色请求
#[derive(Debug, Clone)]
pub struct ListAppRoleRequest {
    config: Config,
    app_token: String,
    page_token: Option<String>,
    page_size: Option<i32>,
}

impl ListAppRoleRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            page_token: None,
            page_size: None,
        }
    }

    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 分页大小，最大值 30
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size.min(30));
        self
    }

    pub async fn execute(self) -> SDKResult<ListAppRoleResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListAppRoleResponse> {
        // === 必填字段验证 ===
        validate_required!(self.app_token.trim(), "app_token");

        // === 边界值验证 ===
        if let Some(page_size) = self.page_size {
            if page_size <= 0 {
                return Err(openlark_core::error::validation_error(
                    "page_size",
                    "page_size 必须大于 0",
                ));
            }
        }

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::RoleList(self.app_token.clone());

        let mut api_request: ApiRequest<ListAppRoleResponse> =
            ApiRequest::get(&api_endpoint.to_url());
        api_request = api_request.query_opt("page_token", self.page_token);
        api_request = api_request.query_opt("page_size", self.page_size.map(|v| v.to_string()));

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

/// 列出自定义角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListAppRoleResponse {
    /// 自定义角色列表
    pub items: Vec<Role>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 总数
    pub total: i32,
}

impl ApiResponseTrait for ListAppRoleResponse {
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
        let request = ListAppRoleRequest::new(config).app_token("".to_string());
        let rt = test_runtime();
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("app_token"));
    }

    #[test]
    fn test_invalid_page_size() {
        let config = Config::default();
        let request = ListAppRoleRequest::new(config)
            .app_token("app_token".to_string())
            .page_size(0);
        let rt = test_runtime();
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("page_size"));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(ListAppRoleResponse::data_format(), ResponseFormat::Data);
    }
}
