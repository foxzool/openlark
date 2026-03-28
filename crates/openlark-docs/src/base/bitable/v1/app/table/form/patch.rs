//! Bitable 更新表单元数据
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-form/patch>

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

/// 更新表单元数据请求
#[derive(Debug, Clone)]
pub struct PatchFormRequest {
    config: Config,
    app_token: String,
    table_id: String,
    form_id: String,

    name: Option<String>,
    description: Option<String>,
    shared: Option<bool>,
    shared_limit: Option<String>,
    submit_limit_once: Option<bool>,
}

impl PatchFormRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            form_id: String::new(),
            name: None,
            description: None,
            shared: None,
            shared_limit: None,
            submit_limit_once: None,
        }
    }

    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    pub fn table_id(mut self, table_id: String) -> Self {
        self.table_id = table_id;
        self
    }

    pub fn form_id(mut self, form_id: String) -> Self {
        self.form_id = form_id;
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn shared(mut self, shared: bool) -> Self {
        self.shared = Some(shared);
        self
    }

    pub fn shared_limit(mut self, shared_limit: String) -> Self {
        self.shared_limit = Some(shared_limit);
        self
    }

    pub fn submit_limit_once(mut self, submit_limit_once: bool) -> Self {
        self.submit_limit_once = Some(submit_limit_once);
        self
    }

    pub async fn execute(self) -> SDKResult<PatchFormResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<PatchFormResponse> {
        // === 必填字段验证 ===
        validate_required!(self.app_token.trim(), "app_token");
        validate_required!(self.table_id.trim(), "table_id");
        validate_required!(self.form_id.trim(), "form_id");

        if self.name.is_none()
            && self.description.is_none()
            && self.shared.is_none()
            && self.shared_limit.is_none()
            && self.submit_limit_once.is_none()
        {
            return Err(openlark_core::error::validation_error(
                "body",
                "至少需要提供一个要更新的字段",
            ));
        }

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::FormPatch(self.app_token, self.table_id, self.form_id);

        let api_request: ApiRequest<PatchFormResponse> = ApiRequest::patch(&api_endpoint.to_url())
            .body(serde_json::to_vec(&PatchFormRequestBody {
                name: self.name,
                description: self.description,
                shared: self.shared,
                shared_limit: self.shared_limit,
                submit_limit_once: self.submit_limit_once,
            })?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

#[derive(Serialize)]
struct PatchFormRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shared_limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    submit_limit_once: Option<bool>,
}

/// 更新表单元数据响应（data）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchFormResponse {
    pub form: Form,
}

impl ApiResponseTrait for PatchFormResponse {
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
        let request = PatchFormRequest::new(config)
            .app_token("".to_string())
            .table_id("table_id".to_string())
            .form_id("form_id".to_string())
            .name("表单名".to_string());
        let rt = test_runtime();
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("app_token"));
    }

    #[test]
    fn test_no_update_fields() {
        let config = Config::default();
        let request = PatchFormRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .form_id("form_id".to_string());
        let rt = test_runtime();
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("至少需要提供一个要更新的字段"));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(PatchFormResponse::data_format(), ResponseFormat::Data);
    }
}
