//! 获取评论列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/comment/list>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::comment::models::ListCommentsResponse;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 获取评论列表请求
#[derive(Debug, Clone)]
pub struct ListCommentsRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务 GUID
    task_guid: String,
    /// 分页大小
    page_size: Option<i32>,
    /// 分页标记
    page_token: Option<String>,
}

impl ListCommentsRequest {
    pub fn new(config: Arc<Config>, task_guid: String) -> Self {
        Self {
            config,
            task_guid,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListCommentsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListCommentsResponse> {
        // 验证必填字段
        validate_required!(self.task_guid.trim(), "任务GUID不能为空");

        let api_endpoint = TaskApiV2::CommentList(self.task_guid.clone());
        let mut request = ApiRequest::<ListCommentsResponse>::get(api_endpoint.to_url());

        // 构建查询参数
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = &self.page_token {
            request = request.query("page_token", page_token);
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取评论列表")
    }
}

impl ApiResponseTrait for ListCommentsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn test_list_comments_request() {
        let config = openlark_core::config::Config::builder()
            .app_id("test")
            .app_secret("test")
            .build();

        let request =
            ListCommentsRequest::new(Arc::new(config), "task_123".to_string()).page_size(20);

        assert_eq!(request.task_guid, "task_123");
        assert_eq!(request.page_size, Some(20));
    }
}
