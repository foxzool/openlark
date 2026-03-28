//! 删除任务评论（v1）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v1/taskcomment/delete>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use serde::Deserialize;
use std::sync::Arc;

/// 删除任务评论响应（v1）
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteTaskCommentResponseV1 {
    /// 是否成功删除
    pub success: bool,
}

/// 删除任务评论请求（v1）
#[derive(Debug, Clone)]
pub struct DeleteTaskCommentRequestV1 {
    config: Arc<Config>,
    task_id: String,
    comment_id: String,
}

impl DeleteTaskCommentRequestV1 {
    pub fn new(
        config: Arc<Config>,
        task_id: impl Into<String>,
        comment_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            task_id: task_id.into(),
            comment_id: comment_id.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteTaskCommentResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteTaskCommentResponseV1> {
        let api_endpoint = crate::common::api_endpoints::TaskApiV1::TaskCommentDelete(
            self.task_id.clone(),
            self.comment_id.clone(),
        );
        let request = ApiRequest::<DeleteTaskCommentResponseV1>::delete(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl ApiResponseTrait for DeleteTaskCommentResponseV1 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_delete_task_comment_v1_url() {
        let endpoint = crate::common::api_endpoints::TaskApiV1::TaskCommentDelete(
            "task_123".to_string(),
            "comment_456".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v1/tasks/task_123/comments/comment_456"
        );
    }
}
