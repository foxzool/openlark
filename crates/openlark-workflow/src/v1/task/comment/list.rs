//! 获取任务评论列表（v1）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v1/taskcomment/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use serde::Deserialize;
use std::sync::Arc;

/// 任务评论列表项（v1）
#[derive(Debug, Clone, Deserialize)]
pub struct TaskCommentItemV1 {
    /// 评论 ID
    pub comment_id: String,
    /// 评论内容
    pub content: String,
    /// 创建者用户 ID
    pub creator_id: String,
    /// 创建时间
    pub created_at: Option<String>,
    /// 父评论 ID
    pub parent_id: Option<String>,
}

/// 获取任务评论列表响应（v1）
#[derive(Debug, Clone, Deserialize)]
pub struct ListTaskCommentResponseV1 {
    /// 评论列表
    pub items: Vec<TaskCommentItemV1>,
    /// 是否有更多数据
    pub has_more: Option<bool>,
    /// 翻页标记
    pub page_token: Option<String>,
}

/// 获取任务评论列表请求（v1）
#[derive(Debug, Clone)]
pub struct ListTaskCommentRequestV1 {
    config: Arc<Config>,
    task_id: String,
    /// 每页数量
    page_size: Option<i32>,
    /// 翻页标记
    page_token: Option<String>,
}

impl ListTaskCommentRequestV1 {
    pub fn new(config: Arc<Config>, task_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
            page_size: None,
            page_token: None,
        }
    }

    /// 设置每页数量
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置翻页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListTaskCommentResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListTaskCommentResponseV1> {
        let api_endpoint =
            crate::common::api_endpoints::TaskApiV1::TaskCommentList(self.task_id.clone());
        let mut request = ApiRequest::<ListTaskCommentResponseV1>::get(api_endpoint.to_url());

        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl ApiResponseTrait for ListTaskCommentResponseV1 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_list_task_comment_v1_url() {
        let endpoint =
            crate::common::api_endpoints::TaskApiV1::TaskCommentList("task_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v1/tasks/task_123/comments"
        );
    }
}
