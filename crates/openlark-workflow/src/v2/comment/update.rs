//! 更新评论
//!
//! docPath: <https://open.feishu.cn/document/task-v2/comment/patch>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::comment::models::{UpdateCommentBody, UpdateCommentResponse};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use std::sync::Arc;

/// 更新评论请求
#[derive(Debug, Clone)]
pub struct UpdateCommentRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 评论 GUID
    comment_guid: String,
    /// 请求体
    body: UpdateCommentBody,
    /// 用户 ID 类型
    user_id_type: Option<String>,
}

impl UpdateCommentRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, comment_guid: String) -> Self {
        Self {
            config,
            comment_guid,
            body: UpdateCommentBody {
                comment: Default::default(),
                update_fields: vec!["content".to_string()],
            },
            user_id_type: None,
        }
    }

    /// 设置评论内容
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.body.comment.content = Some(content.into());
        self
    }

    /// 设置要更新的字段名。
    pub fn update_fields(mut self, update_fields: Vec<String>) -> Self {
        self.body.update_fields = update_fields;
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateCommentResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateCommentResponse> {
        // 验证必填字段
        validate_required!(self.comment_guid.trim(), "评论GUID不能为空");
        validate_required!(
            self.body
                .comment
                .content
                .as_deref()
                .unwrap_or_default()
                .trim(),
            "评论内容不能为空"
        );

        let api_endpoint = TaskApiV2::CommentUpdate(self.comment_guid.clone());
        let mut request = ApiRequest::<UpdateCommentResponse>::patch(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "更新评论")?);
        if let Some(user_id_type) = &self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "更新评论")
    }
}

impl ApiResponseTrait for UpdateCommentResponse {
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
    fn test_update_comment_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request =
            UpdateCommentRequest::new(config, "comment_456".to_string()).content("更新的评论内容");

        assert_eq!(request.comment_guid, "comment_456");
        assert_eq!(
            request.body.comment.content.as_deref(),
            Some("更新的评论内容")
        );
    }
}
