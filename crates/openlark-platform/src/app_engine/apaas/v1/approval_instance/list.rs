//! 获取人工任务列表 API

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 获取审批实例列表的请求构建器。
pub struct ListInstanceRequestBuilder {
    page_size: Option<u32>,
    page_token: Option<String>,
    user_id: Option<String>,
    config: Config,
}

impl ListInstanceRequestBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            page_size: None,
            page_token: None,
            user_id: None,
            config,
        }
    }

    /// 设置分页大小。
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页游标。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置要过滤的用户 ID。
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<ListInstanceResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListInstanceResponse> {
        let mut url = String::from("/open-apis/apaas/v1/approval_instances");
        let mut params = Vec::new();

        if let Some(size) = self.page_size {
            params.push(format!("page_size={size}"));
        }
        if let Some(token) = self.page_token {
            params.push(format!("page_token={token}"));
        }
        if let Some(uid) = self.user_id {
            params.push(format!("user_id={uid}"));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let api_request: ApiRequest<ListInstanceResponse> = ApiRequest::get(url);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取人工任务列表", "响应数据为空")
        })
    }
}

/// 审批实例列表响应。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListInstanceResponse {
    /// 审批实例列表。
    pub items: Vec<InstanceItem>,
    /// 下一页分页游标。
    pub page_token: Option<String>,
    /// 是否还有更多数据。
    pub has_more: bool,
}

/// 单个审批实例信息。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceItem {
    /// 审批实例 ID。
    pub instance_id: String,
    /// 审批实例状态。
    pub status: String,
    /// 发起人 ID。
    pub initiator_id: String,
    /// 创建时间。
    pub create_time: String,
}

impl ApiResponseTrait for ListInstanceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to ListInstanceRequestBuilder, will be removed in v1.0 (#271)")]
pub type ListInstanceBuilder = ListInstanceRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = ListInstanceRequestBuilder::new(config.clone())
            .page_size(1)
            .page_token("test".to_string());
        let _ = request;
    }
}
