//! 创建任务评论（v1）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v1/taskcomment/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 创建任务评论请求体（v1）
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateTaskCommentBodyV1 {
    /// 评论内容
    pub content: String,
    /// 父评论 ID（回复评论时使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

/// 创建任务评论响应（v1）
#[derive(Debug, Clone, Deserialize)]
pub struct CreateTaskCommentResponseV1 {
    /// 评论 ID
    pub comment_id: String,
}

/// 创建任务评论请求（v1）
#[derive(Debug, Clone)]
pub struct CreateTaskCommentRequestV1 {
    config: Arc<Config>,
    task_id: String,
    body: CreateTaskCommentBodyV1,
}

impl CreateTaskCommentRequestV1 {
    pub fn new(config: Arc<Config>, task_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
            body: CreateTaskCommentBodyV1::default(),
        }
    }

    /// 设置评论内容
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.body.content = content.into();
        self
    }

    /// 设置父评论 ID（回复评论时使用）
    pub fn parent_id(mut self, parent_id: impl Into<String>) -> Self {
        self.body.parent_id = Some(parent_id.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateTaskCommentResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateTaskCommentResponseV1> {
        validate_required!(self.body.content.trim(), "评论内容不能为空");

        let api_endpoint =
            crate::common::api_endpoints::TaskApiV1::TaskCommentCreate(self.task_id.clone());
        let mut request = ApiRequest::<CreateTaskCommentResponseV1>::post(api_endpoint.to_url());

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

impl ApiResponseTrait for CreateTaskCommentResponseV1 {
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
    fn test_create_task_comment_v1_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = CreateTaskCommentRequestV1::new(config.clone(), "task_123")
            .content("这是一条评论")
            .parent_id("comment_parent");

        assert_eq!(request.body.content, "这是一条评论");
        assert_eq!(request.body.parent_id, Some("comment_parent".to_string()));
    }

    #[test]
    fn test_task_comment_create_v1_url() {
        let endpoint =
            crate::common::api_endpoints::TaskApiV1::TaskCommentCreate("task_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v1/tasks/task_123/comments"
        );
    }
}
