//! 更新回复的内容
//!
//! 更新指定评论的某条回复内容。
//!
//! ## 功能说明
//! - 更新评论回复的内容
//! - 支持富文本格式的回复内容
//!
//! ## 字段说明
//! - `file_token`: 文件 token，标识文档
//! - `comment_id`: 评论 ID，标识评论
//! - `reply_id`: 回复 ID，标识要更新的回复
//! - `file_type`: 文件类型，如 docx、sheet、bitable 等
//! - `user_id_type`: 用户 ID 类型，默认为 open_id
//! - `content`: 新的回复内容
//!
//! ## 使用示例
//! ```ignore
//! let content = ReplyContent { /* ... */ };
//! let request = UpdateReplyRequest::new("file_token", "comment_123", "reply_456", "docx", content);
//! update_reply(request, &config, None).await?;
//! ```
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/CommentAPI/update>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

use super::models::ReplyContent;

/// 更新回复请求

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct UpdateReplyRequest {
    /// 文件 token
    pub file_token: String,

    /// 评论 ID
    pub comment_id: String,

    /// 回复 ID
    pub reply_id: String,

    /// 云文档类型（必填）
    pub file_type: String,

    /// 用户 ID 类型（默认 open_id）
    pub user_id_type: Option<String>,

    /// 回复内容
    pub content: ReplyContent,
}

impl UpdateReplyRequest {
    pub fn new(
        file_token: impl Into<String>,

        comment_id: impl Into<String>,

        reply_id: impl Into<String>,

        file_type: impl Into<String>,

        content: ReplyContent,
    ) -> Self {
        Self {
            file_token: file_token.into(),

            comment_id: comment_id.into(),

            reply_id: reply_id.into(),

            file_type: file_type.into(),

            user_id_type: None,

            content,
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());

        self
    }
}

#[derive(Debug, Serialize)]

struct UpdateReplyRequestBody {
    content: ReplyContent,
}

/// 更新回复响应（data 为 {}）

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct UpdateReplyResponse {}

impl ApiResponseTrait for UpdateReplyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新回复的内容
pub async fn update_reply(
    request: UpdateReplyRequest,

    config: &Config,

    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<UpdateReplyResponse> {
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

    let api_endpoint = DriveApi::UpdateCommentReply(
        request.file_token.clone(),
        request.comment_id.clone(),
        request.reply_id.clone(),
    );

    let mut api_request: ApiRequest<UpdateReplyResponse> = ApiRequest::put(&api_endpoint.to_url())
        .body(serialize_params(
            &UpdateReplyRequestBody {
                content: request.content,
            },
            "更新回复的内容",
        )?);

    api_request = api_request.query("file_type", &request.file_type);

    if let Some(user_id_type) = &request.user_id_type {
        api_request = api_request.query("user_id_type", user_id_type);
    }

    // ========== 发送请求并返回响应 ==========
    let response = Transport::request(api_request, config, option).await?;

    extract_response_data(response, "更新回复的内容")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_reply_request_builder() {
        let content = ReplyContent { elements: vec![] };

        let request =
            UpdateReplyRequest::new("file_token", "comment_123", "reply_456", "docx", content);

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.comment_id, "comment_123");
        assert_eq!(request.reply_id, "reply_456");
        assert_eq!(request.file_type, "docx");
    }

    #[test]
    fn test_update_reply_request_with_user_id_type() {
        let content = ReplyContent { elements: vec![] };

        let request =
            UpdateReplyRequest::new("file_token", "comment_123", "reply_456", "docx", content)
                .user_id_type("union_id");

        assert_eq!(request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_update_reply_request_empty_fields() {
        let content = ReplyContent { elements: vec![] };

        let request = UpdateReplyRequest::new("", "comment_123", "reply_456", "docx", content);
        assert!(request.file_token.is_empty());

        let content2 = ReplyContent { elements: vec![] };
        let request2 = UpdateReplyRequest::new("token", "", "reply_456", "docx", content2);
        assert!(request2.comment_id.is_empty());

        let content3 = ReplyContent { elements: vec![] };
        let request3 = UpdateReplyRequest::new("token", "comment_123", "", "docx", content3);
        assert!(request3.reply_id.is_empty());

        let content4 = ReplyContent { elements: vec![] };
        let request4 = UpdateReplyRequest::new("token", "comment_123", "reply_456", "", content4);
        assert!(request4.file_type.is_empty());
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(UpdateReplyResponse::data_format(), ResponseFormat::Data);
    }
}
