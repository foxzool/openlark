//! 添加全文评论
//!
//! 向云文档添加全文评论。全文评论是针对整个文档的评论，不是针对文档中特定位置的评论。
//!
//! ## 功能说明
//! - 支持为云文档添加全文评论
//! - 评论必须包含至少一条回复
//! - 支持富文本格式的评论内容
//!
//! ## 字段说明
//! - `file_token`: 文件 token，标识要评论的文档
//! - `file_type`: 文件类型，如 docx、sheet、bitable、wiki 等
//! - `reply_list`: 评论里的回复列表，必须至少包含一条回复
//! - `user_id_type`: 用户 ID 类型，默认为 open_id
//!
//! ## 使用示例
//! ```ignore
//! let reply_list = CreateCommentReplyList {
//!     replies: vec![CreateCommentReply {
//!         content: CommentContent { elements },
//!     }],
//! };
//! let request = CreateCommentRequest::new("file_token", "docx", reply_list);
//! let comment = create_comment(request, &config, None).await?;
//! ```
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/CommentAPI/create>

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

use super::models::{Comment, CreateCommentReplyList};

/// 添加评论请求

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct CreateCommentRequest {
    /// 文件 token
    pub file_token: String,

    /// 文件类型（必填）
    pub file_type: String,

    /// 用户 ID 类型（默认 open_id）

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,

    /// 评论里的回复列表（必填，至少 1 条）
    pub reply_list: CreateCommentReplyList,
}

impl CreateCommentRequest {
    pub fn new(
        file_token: impl Into<String>,

        file_type: impl Into<String>,

        reply_list: CreateCommentReplyList,
    ) -> Self {
        Self {
            file_token: file_token.into(),

            file_type: file_type.into(),

            user_id_type: None,

            reply_list,
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());

        self
    }
}

#[derive(Debug, Serialize)]

struct CreateCommentRequestBody {
    reply_list: CreateCommentReplyList,
}

/// 添加全文评论
pub async fn create_comment(
    request: CreateCommentRequest,

    config: &Config,

    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Comment> {
    // ========== 参数校验 ==========

    if request.file_token.trim().is_empty() {
        return Err(openlark_core::error::validation_error(
            "file_token",
            "file_token 不能为空",
        ));
    }

    if request.file_type.trim().is_empty() {
        return Err(openlark_core::error::validation_error(
            "file_type",
            "file_type 不能为空",
        ));
    }

    super::validate_comment_file_type_for_create(&request.file_type)?;

    if request.reply_list.replies.is_empty() {
        return Err(openlark_core::error::validation_error(
            "reply_list",
            "reply_list.replies 不能为空",
        ));
    }

    // ========== 构建 API 请求 ==========

    let api_endpoint = DriveApi::CreateComment(request.file_token.clone());

    let mut api_request: ApiRequest<Comment> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(
            &CreateCommentRequestBody {
                reply_list: request.reply_list,
            },
            "添加全文评论",
        )?);

    api_request = api_request.query("file_type", &request.file_type);

    if let Some(user_id_type) = &request.user_id_type {
        api_request = api_request.query("user_id_type", user_id_type);
    }

    // ========== 发送请求并返回响应 ==========
    let response = Transport::request(api_request, config, option).await?;

    extract_response_data(response, "添加全文评论")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_comment_request_builder() {
        let reply_list = CreateCommentReplyList {
            replies: vec![super::super::CreateCommentReply {
                content: super::super::models::CommentContent { elements: vec![] },
            }],
        };

        let request = CreateCommentRequest::new("file_token", "docx", reply_list);

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.file_type, "docx");
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_create_comment_request_with_user_id_type() {
        let reply_list = CreateCommentReplyList {
            replies: vec![super::super::CreateCommentReply {
                content: super::super::models::CommentContent { elements: vec![] },
            }],
        };

        let request =
            CreateCommentRequest::new("file_token", "docx", reply_list).user_id_type("union_id");

        assert_eq!(request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_create_comment_request_empty_fields() {
        let reply_list = CreateCommentReplyList {
            replies: vec![super::super::CreateCommentReply {
                content: super::super::models::CommentContent { elements: vec![] },
            }],
        };

        // 测试空字符串字段
        let request = CreateCommentRequest::new("", "docx", reply_list.clone());
        assert!(request.file_token.is_empty());

        let request2 = CreateCommentRequest::new("token", "", reply_list);
        assert!(request2.file_type.is_empty());
    }
}
