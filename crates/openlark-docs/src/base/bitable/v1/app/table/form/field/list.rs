//! Bitable 列出表单问题
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-form-field/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

/// 表单问题项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FormFieldQuestion {
    pub field_id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub required: bool,
    pub visible: bool,
}

/// 列出表单问题请求
#[derive(Debug, Clone)]
pub struct ListFormFieldQuestionRequest {
    config: Config,
    app_token: String,
    table_id: String,
    form_id: String,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListFormFieldQuestionRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            form_id: String::new(),
            page_size: None,
            page_token: None,
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

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    pub async fn execute(self) -> SDKResult<ListFormFieldQuestionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListFormFieldQuestionResponse> {
        // === 必填字段验证 ===
        validate_required!(self.app_token.trim(), "应用令牌不能为空");
        validate_required!(self.table_id.trim(), "数据表ID不能为空");
        validate_required!(self.form_id.trim(), "表单ID不能为空");

        // === 边界值验证 ===
        if let Some(page_size) = self.page_size {
            if page_size < 1 || page_size > 100 {
                return Err(openlark_core::error::validation_error(
                    "page_size",
                    "page_size 必须在 1~100 之间",
                ));
            }
        }

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::FormFieldList(
            self.app_token.clone(),
            self.table_id.clone(),
            self.form_id.clone(),
        );

        let mut api_request: ApiRequest<ListFormFieldQuestionResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        if let Some(page_size) = self.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }
        if let Some(page_token) = &self.page_token {
            api_request = api_request.query("page_token", page_token);
        }

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "列出表单问题")
    }
}

/// 列出表单问题响应（data）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListFormFieldQuestionResponse {
    pub items: Vec<FormFieldQuestion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    pub has_more: bool,
    pub total: i32,
}

impl ApiResponseTrait for ListFormFieldQuestionResponse {
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
        let request = ListFormFieldQuestionRequest::new(config)
            .app_token("".to_string())
            .table_id("table_id".to_string())
            .form_id("form_id".to_string());
        let rt = test_runtime();
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }

    #[test]
    fn test_page_size_out_of_range() {
        let config = Config::default();
        let request = ListFormFieldQuestionRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .form_id("form_id".to_string())
            .page_size(101);
        let rt = test_runtime();
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("page_size"));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            ListFormFieldQuestionResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
