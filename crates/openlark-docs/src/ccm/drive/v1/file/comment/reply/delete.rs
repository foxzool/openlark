//! 删除回复
//!
//! 删除指定评论的某条回复。
//!
//! ## 功能说明
//! - 删除评论的单条回复
//! - 删除后无法恢复
//!
//! ## 字段说明
//! - `file_token`: 文件 token，标识文档
//! - `comment_id`: 评论 ID，标识评论
//! - `reply_id`: 回复 ID，标识要删除的回复
//! - `file_type`: 文件类型，如 docx、sheet、bitable 等
//!
//! ## 使用示例
//! ```ignore
//! let request = DeleteCommentReplyRequest::new("file_token", "comment_123", "reply_456", "docx");
//! delete_comment_reply(request, &config, None).await?;
//! ```
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/CommentAPI/delete>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct DeleteCommentReplyRequest {
    pub file_token: String,

    pub comment_id: String,

    pub reply_id: String,

    /// 云文档类型（必填）
    pub file_type: String,
}

impl DeleteCommentReplyRequest {
    pub fn new(
        file_token: impl Into<String>,

        comment_id: impl Into<String>,

        reply_id: impl Into<String>,

        file_type: impl Into<String>,
    ) -> Self {
        Self {
            file_token: file_token.into(),

            comment_id: comment_id.into(),

            reply_id: reply_id.into(),

            file_type: file_type.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct DeleteCommentReplyResponse {}

impl ApiResponseTrait for DeleteCommentReplyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

pub async fn delete_comment_reply(
    request: DeleteCommentReplyRequest,

    config: &Config,

    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<DeleteCommentReplyResponse> {
    // ========== 参数校验 ==========

    if request.file_token.trim().is_empty() {
        return Err(openlark_core::error::validation_error(
            "file_token",
            "file_token 不能为空",
        ));
    }

    if request.comment_id.trim().is_empty() {
        return Err(openlark_core::error::validation_error(
            "comment_id",
            "comment_id 不能为空",
        ));
    }

    if request.reply_id.trim().is_empty() {
        return Err(openlark_core::error::validation_error(
            "reply_id",
            "reply_id 不能为空",
        ));
    }

    if request.file_type.trim().is_empty() {
        return Err(openlark_core::error::validation_error(
            "file_type",
            "file_type 不能为空",
        ));
    }

    super::super::validate_comment_file_type_for_list_like(&request.file_type)?;

    // ========== 构建 API 请求 ==========

    let api_endpoint = DriveApi::DeleteCommentReply(
        request.file_token.clone(),
        request.comment_id.clone(),
        request.reply_id.clone(),
    );

    let mut api_request: ApiRequest<DeleteCommentReplyResponse> =
        ApiRequest::delete(&api_endpoint.to_url());

    api_request = api_request.query("file_type", &request.file_type);

    // ========== 发送请求并返回响应 ==========
    let response = Transport::request(api_request, config, option).await?;

    extract_response_data(response, "删除回复")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_comment_reply_request_builder() {
        let request =
            DeleteCommentReplyRequest::new("file_token", "comment_123", "reply_456", "docx");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.comment_id, "comment_123");
        assert_eq!(request.reply_id, "reply_456");
        assert_eq!(request.file_type, "docx");
    }

    #[test]
    fn test_delete_comment_reply_request_empty_fields() {
        let request = DeleteCommentReplyRequest::new("", "comment_123", "reply_456", "docx");
        assert!(request.file_token.is_empty());

        let request2 = DeleteCommentReplyRequest::new("token", "", "reply_456", "docx");
        assert!(request2.comment_id.is_empty());

        let request3 = DeleteCommentReplyRequest::new("token", "comment_123", "", "docx");
        assert!(request3.reply_id.is_empty());

        let request4 = DeleteCommentReplyRequest::new("token", "comment_123", "reply_456", "");
        assert!(request4.file_type.is_empty());
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            DeleteCommentReplyResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
