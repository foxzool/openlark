//! 附件 API v2 的数据模型

use serde::Deserialize;

/// 上传附件响应
#[derive(Debug, Clone, Deserialize)]
pub struct UploadAttachmentResponse {
    /// 附件 GUID
    pub attachment_guid: String,
    /// 任务 GUID
    pub task_guid: String,
    /// 文件名
    pub file_name: String,
    /// 文件大小（字节）
    pub file_size: i64,
    /// 文件类型
    pub file_type: String,
    /// 上传时间
    pub created_at: String,
}

/// 删除附件响应
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAttachmentResponse {
    /// 是否删除成功
    pub success: bool,
    /// 附件 GUID
    pub attachment_guid: String,
}
