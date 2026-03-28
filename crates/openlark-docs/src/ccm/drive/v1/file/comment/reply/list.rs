//! 获取回复信息
//!
//! 获取指定评论的所有回复列表。
//!
//! ## 功能说明
//! - 获取评论下的所有回复
//! - 支持分页查询
//!
//! ## 字段说明
//! - `file_token`: 文件 token，标识文档
//! - `comment_id`: 评论 ID，标识评论
//! - `file_type`: 文件类型，如 docx、sheet、bitable 等
//! - `page_size`: 分页大小，1~100
//! - `page_token`: 分页标记
//! - `user_id_type`: 用户 ID 类型，默认为 open_id
//!
//! ## 使用示例
//! ```ignore
//! let request = ListCommentReplyRequest::new("file_token", "comment_123", "docx")
//!     .page_size(20)
//!     .page_token("token");
//! let response = list_comment_reply(request, &config, None).await?;
//! ```
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/CommentAPI/list-2>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

use super::models::ReplyInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct ListCommentReplyRequest {
    pub file_token: String,

    pub comment_id: String,

    /// 云文档类型（必填）
    pub file_type: String,

    pub page_token: Option<String>,

    pub page_size: Option<i32>,

    pub user_id_type: Option<String>,
}

impl ListCommentReplyRequest {
    pub fn new(
        file_token: impl Into<String>,

        comment_id: impl Into<String>,

        file_type: impl Into<String>,
    ) -> Self {
        Self {
            file_token: file_token.into(),

            comment_id: comment_id.into(),

            file_type: file_type.into(),

            page_token: None,

            page_size: None,

            user_id_type: None,
        }
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());

        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);

        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());

        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct ListCommentReplyResponse {
    #[serde(default)]
    pub items: Vec<ReplyInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,

    pub has_more: bool,
}

impl ApiResponseTrait for ListCommentReplyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

pub async fn list_comment_reply(
    request: ListCommentReplyRequest,

    config: &Config,

    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<ListCommentReplyResponse> {
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

    if request.file_type.trim().is_empty() {
        return Err(openlark_core::error::validation_error(
            "file_type",
            "file_type 不能为空",
        ));
    }

    super::super::validate_comment_file_type_for_list_like(&request.file_type)?;

    if let Some(page_size) = request.page_size {
        if !(1..=100).contains(&page_size) {
            return Err(openlark_core::error::validation_error(
                "page_size",
                "page_size 必须在 1~100 之间",
            ));
        }
    }

    // ========== 构建 API 请求 ==========

    let api_endpoint =
        DriveApi::ListCommentReplies(request.file_token.clone(), request.comment_id.clone());

    let mut api_request: ApiRequest<ListCommentReplyResponse> =
        ApiRequest::get(&api_endpoint.to_url());

    api_request = api_request.query("file_type", &request.file_type);

    if let Some(token) = &request.page_token {
        api_request = api_request.query("page_token", token);
    }

    if let Some(size) = request.page_size {
        api_request = api_request.query("page_size", &size.to_string());
    }

    if let Some(user_id_type) = &request.user_id_type {
        api_request = api_request.query("user_id_type", user_id_type);
    }

    // ========== 发送请求并返回响应 ==========
    let response = Transport::request(api_request, config, option).await?;

    extract_response_data(response, "获取回复信息")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_comment_reply_request_builder() {
        let request = ListCommentReplyRequest::new("file_token", "comment_123", "docx");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.comment_id, "comment_123");
        assert_eq!(request.file_type, "docx");
        assert!(request.page_size.is_none());
        assert!(request.page_token.is_none());
    }

    #[test]
    fn test_list_comment_reply_request_with_params() {
        let request = ListCommentReplyRequest::new("file_token", "comment_123", "docx")
            .page_size(20)
            .page_token("next_page_token")
            .user_id_type("union_id");

        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token, Some("next_page_token".to_string()));
        assert_eq!(request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_list_comment_reply_request_empty_fields() {
        let request = ListCommentReplyRequest::new("", "comment_123", "docx");
        assert!(request.file_token.is_empty());

        let request2 = ListCommentReplyRequest::new("token", "", "docx");
        assert!(request2.comment_id.is_empty());

        let request3 = ListCommentReplyRequest::new("token", "comment_123", "");
        assert!(request3.file_type.is_empty());
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            ListCommentReplyResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
