/// 文件评论（comment）相关模型
///
/// 注意：该文件仅存放模型结构，不计入 API 文件数量。
use serde::{Deserialize, Serialize};

use openlark_core::api::ApiResponseTrait;

/// 评论内容
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommentContent {
    /// 评论内容元素列表。
    #[serde(default)]
    pub elements: Vec<CommentElement>,
}

/// 评论内容元素
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommentElement {
    /// 元素类型（例如 text_run）
    #[serde(rename = "type")]
    pub element_type: String,
    /// 文本内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_run: Option<TextRun>,
    /// 文档链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs_link: Option<DocsLink>,
    /// @人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<Person>,
}

/// 文本片段。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TextRun {
    /// 文本内容。
    pub text: String,
}

/// 文档链接信息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocsLink {
    /// 链接地址。
    pub url: String,
}

/// 人员引用信息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Person {
    /// 用户 ID。
    pub user_id: String,
}

/// 回复的其他内容（图片 token 等）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReplyExtra {
    /// 图片 token 列表。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub image_list: Vec<String>,
}

/// 评论回复（响应体）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommentReply {
    /// 回复内容。
    pub content: CommentContent,
    /// 回复 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_id: Option<String>,
    /// 回复者用户 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 创建时间。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 更新时间。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    /// 扩展信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<ReplyExtra>,
}

/// 评论回复列表（响应体）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommentReplyList {
    /// 回复列表。
    #[serde(default)]
    pub replies: Vec<CommentReply>,
}

/// 新建评论时的回复项（请求体）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateCommentReply {
    /// 回复内容。
    pub content: CommentContent,
}

/// 新建评论时的回复列表（请求体）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateCommentReplyList {
    /// 回复列表。
    #[serde(default)]
    pub replies: Vec<CreateCommentReply>,
}

/// 评论信息（用于 get/create/list/batch_query 的返回 items）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Comment {
    /// 评论 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<String>,
    /// 评论者用户 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 创建时间。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 更新时间。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    /// 是否已解决。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_solved: Option<bool>,
    /// 解决时间。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solved_time: Option<i64>,
    /// 解决者用户 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solver_user_id: Option<String>,

    /// 回复分页：是否有更多回复
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 回复分页：分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,

    /// 是否是全文评论
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_whole: Option<bool>,
    /// 局部评论的引用字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,

    /// 评论里的回复列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_list: Option<CommentReplyList>,
}

impl ApiResponseTrait for Comment {}
