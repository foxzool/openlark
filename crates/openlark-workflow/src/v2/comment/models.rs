//! 评论 API v2 的数据模型

use serde::{Deserialize, Serialize};

/// 可写评论字段。
#[derive(Debug, Clone, Serialize, Default, PartialEq)]
pub struct InputComment {
    /// 评论内容。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 被回复评论的 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_comment_id: Option<String>,
    /// 评论归属资源类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// 评论归属资源 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

/// 创建评论请求体。
pub type CreateCommentBody = InputComment;

/// 更新评论请求体。
#[derive(Debug, Clone, Serialize, Default, PartialEq)]
pub struct UpdateCommentBody {
    /// 要更新的评论字段。
    pub comment: InputComment,
    /// 要更新的字段名。
    pub update_fields: Vec<String>,
}

/// 评论创建者。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommentCreator {
    /// 成员 ID。
    #[serde(default)]
    pub id: Option<String>,
    /// 成员类型。
    #[serde(default)]
    pub r#type: Option<String>,
    /// 成员角色。
    #[serde(default)]
    pub role: Option<String>,
    /// 成员名称。
    #[serde(default)]
    pub name: Option<String>,
}

/// 评论详情。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommentItem {
    /// 评论 ID。
    #[serde(default)]
    pub id: Option<String>,
    /// 评论内容。
    #[serde(default)]
    pub content: Option<String>,
    /// 评论创建者。
    #[serde(default)]
    pub creator: Option<CommentCreator>,
    /// 被回复评论的 ID。
    #[serde(default)]
    pub reply_to_comment_id: Option<String>,
    /// 创建时间戳（毫秒）。
    #[serde(default)]
    pub created_at: Option<String>,
    /// 更新时间戳（毫秒）。
    #[serde(default)]
    pub updated_at: Option<String>,
    /// 资源类型。
    #[serde(default)]
    pub resource_type: Option<String>,
    /// 资源 ID。
    #[serde(default)]
    pub resource_id: Option<String>,
}

/// 创建评论响应。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateCommentResponse {
    /// 创建的评论。
    pub comment: CommentItem,
}

/// 获取评论响应。
#[derive(Debug, Clone, Deserialize)]
pub struct GetCommentResponse {
    /// 评论详情。
    pub comment: CommentItem,
}

/// 更新评论响应。
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateCommentResponse {
    /// 更新后的评论。
    pub comment: CommentItem,
}

/// 删除评论响应。
#[derive(Debug, Clone, Deserialize, Default)]
pub struct DeleteCommentResponse {}

/// 获取评论列表响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListCommentsResponse {
    /// 是否还有更多项。
    #[serde(default)]
    pub has_more: bool,
    /// 分页标记。
    #[serde(default)]
    pub page_token: Option<String>,
    /// 列表项。
    #[serde(default)]
    pub items: Vec<CommentItem>,
}
