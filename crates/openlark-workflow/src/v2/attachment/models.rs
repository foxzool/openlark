//! 附件 API v2 的数据模型

use serde::Deserialize;

/// 附件归属资源。
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct AttachmentResource {
    /// 资源类型。
    #[serde(default)]
    pub r#type: Option<String>,
    /// 资源 ID。
    #[serde(default)]
    pub id: Option<String>,
}

/// 附件上传者。
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct AttachmentUploader {
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

/// 附件详情。
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct AttachmentInfo {
    /// 附件 GUID。
    #[serde(default)]
    pub guid: Option<String>,
    /// 附件在云文档系统中的 token。
    #[serde(default)]
    pub file_token: Option<String>,
    /// 附件名。
    #[serde(default)]
    pub name: Option<String>,
    /// 文件大小（字节）。
    #[serde(default)]
    pub size: Option<i64>,
    /// 附件归属资源。
    #[serde(default)]
    pub resource: Option<AttachmentResource>,
    /// 附件上传者。
    #[serde(default)]
    pub uploader: Option<AttachmentUploader>,
    /// 是否为封面图。
    #[serde(default)]
    pub is_cover: Option<bool>,
    /// 上传时间戳（毫秒）。
    #[serde(default)]
    pub uploaded_at: Option<String>,
    /// 临时下载 URL。
    #[serde(default)]
    pub url: Option<String>,
}

/// 上传附件响应。
#[derive(Debug, Clone, Deserialize)]
pub struct UploadAttachmentResponse {
    /// 上传的附件列表。
    #[serde(default)]
    pub items: Vec<AttachmentInfo>,
}

/// 删除附件响应。
#[derive(Debug, Clone, Deserialize, Default)]
pub struct DeleteAttachmentResponse {}
