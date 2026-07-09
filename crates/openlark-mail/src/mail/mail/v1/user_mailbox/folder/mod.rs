/// 创建接口。
pub mod create;
/// 删除接口。
pub mod delete;
/// 列表接口。
pub mod list;
/// 更新接口。
pub mod patch;

use openlark_core::config::Config;
use std::sync::Arc;

/// 用户邮箱文件夹资源。
#[derive(Clone)]
pub struct Folder {
    config: Arc<Config>,
    mailbox_id: String,
}

impl Folder {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
        }
    }

    /// 创建列表请求。
    pub fn list(&self) -> list::ListMailboxFolderRequest {
        list::ListMailboxFolderRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 创建新建请求。
    pub fn create(&self) -> create::CreateMailboxFolderRequest {
        create::CreateMailboxFolderRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 创建补丁请求。
    pub fn patch(&self, folder_id: impl Into<String>) -> patch::PatchMailboxFolderRequest {
        patch::PatchMailboxFolderRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            folder_id,
        )
    }

    /// 创建删除请求。
    pub fn delete(&self, folder_id: impl Into<String>) -> delete::DeleteMailboxFolderRequest {
        delete::DeleteMailboxFolderRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            folder_id,
        )
    }
}
/// get 模块。
pub mod get;
