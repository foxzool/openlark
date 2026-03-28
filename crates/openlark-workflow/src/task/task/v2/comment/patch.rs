//! 更新评论（PATCH）
//!
//! docPath: <https://open.feishu.cn/document/task-v2/comment/patch>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::comment::models::{UpdateCommentBody, UpdateCommentResponse};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 更新评论请求
#[derive(Debug, Clone)]
pub struct PatchCommentRequest {
    config: Arc<Config>,
    comment_id: String,
    body: UpdateCommentBody,
}

impl PatchCommentRequest {
    pub fn new(config: Arc<Config>, comment_id: impl Into<String>) -> Self {
        Self {
            config,
            comment_id: comment_id.into(),
            body: UpdateCommentBody::default(),
        }
    }

    /// 设置评论内容
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.body.content = Some(content.into());
        self
    }

    /// 设置要更新的字段
    pub fn update_fields(mut self, update_fields: Vec<String>) -> Self {
        self.body.update_fields = Some(update_fields);
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
        validate_required!(self.comment_id.trim(), "评论 ID 不能为空");

        let api_endpoint = TaskApiV2::CommentUpdate(self.comment_id.clone(), self.comment_id);
        let mut request = ApiRequest::<UpdateCommentResponse>::patch(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "更新评论")?);

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
    

    #[test]
    fn test_patch_comment_url() {
        let endpoint = TaskApiV2::CommentUpdate("comment_123".to_string(), "comment_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v2/comments/comment_123"
        );
    }
}
