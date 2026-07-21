//! 创建评论
//!
//! docPath: <https://open.feishu.cn/document/task-v2/comment/create>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::comment::models::{CreateCommentBody, CreateCommentResponse};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use std::sync::Arc;

/// 创建评论请求
#[derive(Debug, Clone)]
pub struct CreateCommentRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 请求体
    body: CreateCommentBody,
    /// 用户 ID 类型
    user_id_type: Option<String>,
}

impl CreateCommentRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, task_guid: String) -> Self {
        Self {
            config,
            body: CreateCommentBody {
                resource_type: Some("task".to_string()),
                resource_id: Some(task_guid),
                ..CreateCommentBody::default()
            },
            user_id_type: None,
        }
    }

    /// 设置评论内容
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.body.content = Some(content.into());
        self
    }

    /// 设置被回复评论的 ID。
    pub fn reply_to_comment_id(mut self, comment_id: impl Into<String>) -> Self {
        self.body.reply_to_comment_id = Some(comment_id.into());
        self
    }

    /// 设置资源类型。
    pub fn resource_type(mut self, resource_type: impl Into<String>) -> Self {
        self.body.resource_type = Some(resource_type.into());
        self
    }

    /// 设置资源 ID。
    pub fn resource_id(mut self, resource_id: impl Into<String>) -> Self {
        self.body.resource_id = Some(resource_id.into());
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateCommentResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateCommentResponse> {
        // 验证必填字段
        validate_required!(
            self.body.content.as_deref().unwrap_or_default().trim(),
            "评论内容不能为空"
        );

        let api_endpoint = TaskApiV2::CommentCreate;
        let mut request = ApiRequest::<CreateCommentResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "创建评论")?);
        if let Some(user_id_type) = &self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "创建评论",
        )
        .await
    }
}

impl ApiResponseTrait for CreateCommentResponse {
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
    fn test_create_comment_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request =
            CreateCommentRequest::new(config, "task_123".to_string()).content("这是一条评论");

        assert_eq!(request.body.resource_id.as_deref(), Some("task_123"));
        assert_eq!(request.body.content.as_deref(), Some("这是一条评论"));
    }
}
