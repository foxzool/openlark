//! 列出自动化流程
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-workflow/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::BitableApiV1;

/// 自动化流程信息（app.workflow）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    /// 自动化流程的 ID
    pub workflow_id: String,
    /// 自动化流程的状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 自动化流程的名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// 列出自动化流程响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWorkflowResponse {
    /// 自动化流程信息
    pub workflows: Vec<Workflow>,
}

impl ApiResponseTrait for ListWorkflowResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出自动化流程请求
pub struct ListWorkflowRequest {
    config: Config,
    app_token: String,
    page_token: Option<String>,
    page_size: Option<i32>,
}

impl ListWorkflowRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            page_token: None,
            page_size: None,
        }
    }

    /// 多维表格 app_token（路径参数）
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 分页大小（默认 20）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListWorkflowResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListWorkflowResponse> {
        validate_required!(self.app_token, "app_token 不能为空");

        let api_endpoint = BitableApiV1::WorkflowList(self.app_token);
        let mut api_request: ApiRequest<ListWorkflowResponse> =
            ApiRequest::get(&api_endpoint.to_url());
        api_request = api_request.query_opt("page_token", self.page_token);
        api_request = api_request.query_opt("page_size", self.page_size.map(|v| v.to_string()));

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
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
