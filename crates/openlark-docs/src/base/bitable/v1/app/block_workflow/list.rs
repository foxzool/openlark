//! 列出工作流
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-block_workflow/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::BitableApiV1;

/// 工作流信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlockWorkflow {
    /// 工作流唯一键
    pub workflow_id: String,
    /// 工作流标题
    pub title: String,
    /// 工作流状态
    pub status: String,
}

/// 列出工作流响应（data）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListBlockWorkflowResponse {
    /// 工作流列表
    pub workflows: Vec<BlockWorkflow>,
}

impl ApiResponseTrait for ListBlockWorkflowResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出工作流请求。
#[derive(Debug, Clone)]
pub struct ListBlockWorkflowRequest {
    config: Config,
    app_token: String,
}

impl ListBlockWorkflowRequest {
    /// 创建新的工作流列表请求。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
        }
    }

    /// 多维表格 app_token（路径参数）
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ListBlockWorkflowResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListBlockWorkflowResponse> {
        validate_required!(self.app_token.trim(), "app_token 不能为空");

        let api_endpoint = BitableApiV1::BlockWorkflowList(self.app_token);
        let api_request: ApiRequest<ListBlockWorkflowResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use openlark_core::testing::prelude::test_runtime;

    #[test]
    fn test_empty_app_token() {
        let config = Config::default();
        let request = ListBlockWorkflowRequest::new(config).app_token("");
        let rt = test_runtime();
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            ListBlockWorkflowResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
