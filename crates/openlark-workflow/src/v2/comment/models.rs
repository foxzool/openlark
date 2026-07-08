//! 评论 API v2 的数据模型

use serde::{Deserialize, Serialize};

/// 创建评论请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateCommentBody {
    /// 评论内容
    pub content: String,
}

/// 更新评论请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateCommentBody {
    /// 评论内容
    pub content: String,
}

/// 创建评论响应
#[derive(Debug, Clone, Deserialize)]
pub struct CreateCommentResponse {
    /// 评论 GUID
    pub comment_guid: String,
    /// 任务 GUID
    pub task_guid: String,
    /// 评论内容
    pub content: String,
    /// 评论创建者
    pub creator: String,
    /// 创建时间
    pub created_at: String,
    /// 更新时间
    pub updated_at: String,
}

/// 获取评论响应
#[derive(Debug, Clone, Deserialize)]
pub struct GetCommentResponse {
    /// 评论 GUID
    pub comment_guid: String,
    /// 任务 GUID
    pub task_guid: String,
    /// 评论内容
    pub content: String,
    /// 评论创建者
    pub creator: String,
    /// 创建时间
    pub created_at: String,
    /// 更新时间
    pub updated_at: String,
}

/// 更新评论响应
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateCommentResponse {
    /// 评论 GUID
    pub comment_guid: String,
    /// 任务 GUID
    pub task_guid: String,
    /// 评论内容
    pub content: String,
    /// 更新时间
    pub updated_at: String,
}

/// 删除评论响应
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteCommentResponse {
    /// 是否删除成功
    pub success: bool,
    /// 评论 GUID
    pub comment_guid: String,
}

/// 评论列表项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommentItem {
    /// 评论 GUID
    pub comment_guid: String,
    /// 任务 GUID
    pub task_guid: String,
    /// 评论内容
    pub content: String,
    /// 评论创建者
    pub creator: String,
    /// 创建时间
    pub created_at: String,
    /// 更新时间
    pub updated_at: String,
}

/// 获取评论列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListCommentsResponse {
    /// 是否还有更多项
    #[serde(default)]
    pub has_more: bool,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 总数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 列表项
    #[serde(default)]
    pub items: Vec<CommentItem>,
}
