//! 更新任务评论（v1）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v1/taskcomment/update>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 更新任务评论请求体（v1）
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateTaskCommentBodyV1 {
    /// 评论内容
    pub content: String,
}

/// 更新任务评论响应（v1）
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTaskCommentResponseV1 {
    /// 评论 ID
    pub comment_id: String,
    /// 评论内容
    pub content: String,
}

/// 更新任务评论请求（v1）
#[derive(Debug, Clone)]
pub struct UpdateTaskCommentRequestV1 {
    config: Arc<Config>,
    task_id: String,
    comment_id: String,
    body: UpdateTaskCommentBodyV1,
}

impl UpdateTaskCommentRequestV1 {
    pub fn new(
        config: Arc<Config>,
        task_id: impl Into<String>,
        comment_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            task_id: task_id.into(),
            comment_id: comment_id.into(),
            body: UpdateTaskCommentBodyV1::default(),
        }
    }

    /// 设置评论内容
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.body.content = content.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateTaskCommentResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateTaskCommentResponseV1> {
        validate_required!(self.body.content.trim(), "评论内容不能为空");

        let api_endpoint = crate::common::api_endpoints::TaskApiV1::TaskCommentUpdate(
            self.task_id.clone(),
            self.comment_id.clone(),
        );
        let mut request = ApiRequest::<UpdateTaskCommentResponseV1>::put(api_endpoint.to_url());

        let body_json = serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::validation_error("序列化请求体失败", e.to_string().as_str())
        })?;

        request = request.body(body_json);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl ApiResponseTrait for UpdateTaskCommentResponseV1 {
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
    fn test_update_task_comment_v1_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = UpdateTaskCommentRequestV1::new(config.clone(), "task_123", "comment_456")
            .content("更新后的评论内容");

        assert_eq!(request.body.content, "更新后的评论内容");
    }

    #[test]
    fn test_task_comment_update_v1_url() {
        let endpoint = crate::common::api_endpoints::TaskApiV1::TaskCommentUpdate(
            "task_123".to_string(),
            "comment_456".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v1/tasks/task_123/comments/comment_456"
        );
    }
}
