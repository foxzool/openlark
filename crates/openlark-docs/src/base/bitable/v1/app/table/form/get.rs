//! Bitable 获取表单API
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-form/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use super::models::Form;

/// 获取表单请求
#[derive(Debug, Clone)]
pub struct GetFormRequest {
    config: Config,
    app_token: String,
    table_id: String,
    form_id: String,
}

impl GetFormRequest {
    /// 创建获取表单请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            form_id: String::new(),
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置数据表ID
    pub fn table_id(mut self, table_id: String) -> Self {
        self.table_id = table_id;
        self
    }

    /// 设置表单ID
    pub fn form_id(mut self, form_id: String) -> Self {
        self.form_id = form_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetFormResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<GetFormResponse> {
        // 参数验证
        validate_required!(self.app_token.trim(), "app_token");
        validate_required!(self.table_id.trim(), "table_id");
        validate_required!(self.form_id.trim(), "form_id");

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::FormGet(self.app_token, self.table_id, self.form_id);

        let api_request: ApiRequest<GetFormResponse> = ApiRequest::get(&api_endpoint.to_url());
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

/// 获取表单响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetFormResponse {
    /// 表单元数据
    pub form: Form,
}

impl ApiResponseTrait for GetFormResponse {
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
        let request = GetFormRequest::new(config)
            .app_token("".to_string())
            .table_id("table_id".to_string())
            .form_id("form_id".to_string());
        let rt = test_runtime();
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("app_token"));
    }

    #[test]
    fn test_empty_table_id() {
        let config = Config::default();
        let request = GetFormRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("".to_string())
            .form_id("form_id".to_string());
        let rt = test_runtime();
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("table_id"));
    }

    #[test]
    fn test_empty_form_id() {
        let config = Config::default();
        let request = GetFormRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .form_id("".to_string());
        let rt = test_runtime();
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("form_id"));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(GetFormResponse::data_format(), ResponseFormat::Data);
    }
}
