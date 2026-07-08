use serde::{Deserialize, Serialize};

/// 评论回复信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyInfo {
    /// 回复ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_id: Option<String>,
    /// 回复内容
    pub content: ReplyContent,
    /// 回复者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    /// 其他内容（图片 token 等）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<ReplyExtra>,
}

/// 回复内容。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyContent {
    /// 回复文本内容
    pub elements: Vec<ReplyElement>,
}

/// 回复内容元素。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyElement {
    /// 元素类型。
    #[serde(rename = "type")]
    pub r#type: String,
    /// 文本内容。
    pub text_run: Option<TextRun>,
    /// 文档链接。
    pub docs_link: Option<DocsLink>,
    /// @ 人信息。
    pub person: Option<Person>,
}

/// 回复内容中的文本元素。
/// 回复扩展信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyExtra {
    /// 图片 token 列表。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub image_list: Vec<String>,
}

/// 文本片段。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextRun {
    /// 文本内容。
    pub text: String,
}

/// 文档链接信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsLink {
    /// 链接地址。
    pub url: String,
}

/// 人员引用信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    /// 用户 ID。
    pub user_id: String,
}
