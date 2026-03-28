//! 删除指定知识库
//!
//! 删除指定的知识库。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/faq-management/faq/delete>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::extract_response_data;

/// 删除知识库响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFaqResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DeleteFaqResult>,
}

impl openlark_core::api::ApiResponseTrait for DeleteFaqResponse {}

/// 删除知识库结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFaqResult {
    /// 是否删除成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

/// 删除知识库请求
#[derive(Debug, Clone)]
pub struct DeleteFaqRequest {
    config: Arc<Config>,
    id: String,
}

impl DeleteFaqRequest {
    /// 创建新的删除知识库请求
    pub fn new(config: Arc<Config>, id: String) -> Self {
        Self { config, id }
    }

    /// 执行删除知识库请求
    pub async fn execute(self) -> SDKResult<DeleteFaqResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行删除知识库请求（支持自定义选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<DeleteFaqResponse> {
        let req: ApiRequest<DeleteFaqResponse> =
            ApiRequest::delete(HelpdeskApiV1::FaqDelete(self.id.clone()).to_url());

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "删除知识库")
    }
}

/// 删除知识库请求构建器
#[derive(Debug, Clone)]
pub struct DeleteFaqRequestBuilder {
    config: Arc<Config>,
    id: String,
}

impl DeleteFaqRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, id: String) -> Self {
        Self { config, id }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<DeleteFaqResponse> {
        let request = DeleteFaqRequest::new(self.config.clone(), self.id.clone());
        request.execute().await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        &self,
        option: RequestOption,
    ) -> SDKResult<DeleteFaqResponse> {
        let request = DeleteFaqRequest::new(self.config.clone(), self.id.clone());
        request.execute_with_options(option).await
    }
}

/// 执行删除知识库
pub async fn delete_faq(config: &Config, id: String) -> SDKResult<DeleteFaqResponse> {
    delete_faq_with_options(config, id, RequestOption::default()).await
}

/// 执行删除知识库（支持自定义选项）
pub async fn delete_faq_with_options(
    config: &Config,
    id: String,
    option: RequestOption,
) -> SDKResult<DeleteFaqResponse> {
    let req: ApiRequest<DeleteFaqResponse> =
        ApiRequest::delete(HelpdeskApiV1::FaqDelete(id).to_url());

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "删除知识库")
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = DeleteFaqRequestBuilder::new(Arc::new(config), "faq_123".to_string());

        assert_eq!(builder.id, "faq_123");
    }
}
