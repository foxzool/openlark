/// 创建接口。
pub mod create;
/// 删除接口。
pub mod delete;
/// 列表接口。
pub mod list;

use openlark_core::config::Config;
use std::sync::Arc;

/// 用户邮箱别名资源
#[derive(Clone)]
pub struct Alias {
    config: Arc<Config>,
    mailbox_id: String,
}

impl Alias {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
        }
    }

    /// 创建列表请求。
    pub fn list(&self) -> list::ListMailboxAliasRequest {
        list::ListMailboxAliasRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 创建新建请求。
    pub fn create(&self) -> create::CreateMailboxAliasRequest {
        create::CreateMailboxAliasRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 创建删除请求。
    pub fn delete(&self, alias_id: impl Into<String>) -> delete::DeleteMailboxAliasRequest {
        delete::DeleteMailboxAliasRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            alias_id,
        )
    }
}
