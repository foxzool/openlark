//! 获取任务评论详情（v1）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v1/taskcomment/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use serde::Deserialize;
use std::sync::Arc;

/// 任务评论详情（v1）
#[derive(Debug, Clone, Deserialize)]
pub struct GetTaskCommentResponseV1 {
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

/// 获取任务评论详情请求（v1）
#[derive(Debug, Clone)]
pub struct GetTaskCommentRequestV1 {
    config: Arc<Config>,
    task_id: String,
    comment_id: String,
}

impl GetTaskCommentRequestV1 {
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
    pub async fn execute(self) -> SDKResult<GetTaskCommentResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetTaskCommentResponseV1> {
        let api_endpoint = crate::common::api_endpoints::TaskApiV1::TaskCommentGet(
            self.task_id.clone(),
            self.comment_id.clone(),
        );
        let request = ApiRequest::<GetTaskCommentResponseV1>::get(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl ApiResponseTrait for GetTaskCommentResponseV1 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_get_task_comment_v1_url() {
        let endpoint = crate::common::api_endpoints::TaskApiV1::TaskCommentGet(
            "task_123".to_string(),
            "comment_456".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v1/tasks/task_123/comments/comment_456"
        );
    }
}
