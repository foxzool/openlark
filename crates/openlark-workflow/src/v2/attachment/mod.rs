/// 删除接口。
pub mod delete;
/// 获取接口。
pub mod get;
/// 列表接口。
pub mod list;
/// 数据模型。
pub mod models;
/// 上传接口。
pub mod upload;

use openlark_core::config::Config;
use std::sync::Arc;

/// Attachment：附件资源
#[derive(Clone)]
pub struct Attachment {
    config: Arc<Config>,
    task_guid: String,
}

impl Attachment {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            task_guid: String::new(),
        }
    }

    /// 绑定任务上下文。
    pub fn with_task(mut self, task_guid: impl Into<String>) -> Self {
        self.task_guid = task_guid.into();
        self
    }

    /// upload。
    pub fn upload(&self, file_path: String) -> upload::UploadAttachmentRequest {
        upload::UploadAttachmentRequest::new(self.config.clone(), self.task_guid.clone(), file_path)
    }

    /// 创建删除请求。
    pub fn delete(&self, attachment_guid: impl Into<String>) -> delete::DeleteAttachmentRequest {
        delete::DeleteAttachmentRequest::new(self.config.clone(), attachment_guid.into())
    }

    /// 获取附件（不需要 task_guid）
    pub fn get(&self, attachment_guid: impl Into<String>) -> get::GetAttachmentRequest {
        get::GetAttachmentRequest::new(self.config.clone(), attachment_guid.into())
    }

    /// 列取附件（不需要 task_guid）
    pub fn list(&self) -> list::ListAttachmentsRequest {
        list::ListAttachmentsRequest::new(self.config.clone()).task_guid(self.task_guid.clone())
    }
}

// 重新导出请求类型
pub use delete::DeleteAttachmentRequest;
pub use get::GetAttachmentRequest;
pub use list::ListAttachmentsRequest;
pub use upload::UploadAttachmentRequest;

// 重新导出响应类型
pub use get::GetAttachmentResponse;
pub use list::{AttachmentListItem, ListAttachmentsResponse};
pub use models::{
    AttachmentInfo, AttachmentResource, AttachmentUploader, DeleteAttachmentResponse,
    UploadAttachmentResponse,
};

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn create_test_config() -> Arc<Config> {
        Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        )
    }

    #[test]
    fn test_attachment_new() {
        let config = create_test_config();
        let attachment = Attachment::new(config);
        assert!(attachment.task_guid.is_empty());
    }

    #[test]
    fn test_attachment_with_task() {
        let config = create_test_config();
        let attachment = Attachment::new(config).with_task("task_123");
        assert_eq!(attachment.task_guid, "task_123");
    }

    #[test]
    fn test_attachment_upload() {
        let config = create_test_config();
        let attachment = Attachment::new(config).with_task("task_123");
        let _request = attachment.upload("/path/to/file.txt".to_string());
    }

    #[test]
    fn test_attachment_delete() {
        let config = create_test_config();
        let attachment = Attachment::new(config).with_task("task_123");
        let _request = attachment.delete("attachment_456");
    }

    #[test]
    fn test_attachment_get() {
        let config = create_test_config();
        let attachment = Attachment::new(config);
        let _request = attachment.get("attachment_456");
    }

    #[test]
    fn test_attachment_list() {
        let config = create_test_config();
        let attachment = Attachment::new(config);
        let _request = attachment.list();
    }
}
