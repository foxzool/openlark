/// 创建接口。
pub mod create;
/// 删除接口。
pub mod delete;
/// 列表接口。
pub mod list;

use openlark_core::config::Config;
use std::sync::Arc;

/// 邮件组别名资源
#[derive(Clone)]
pub struct Alias {
    config: Arc<Config>,
    mailgroup_id: String,
}

impl Alias {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
        }
    }

    /// 创建列表请求。
    pub fn list(&self) -> list::ListMailGroupAliasRequest {
        list::ListMailGroupAliasRequest::new(self.config.clone(), self.mailgroup_id.clone())
    }

    /// 创建新建请求。
    pub fn create(&self) -> create::CreateMailGroupAliasRequest {
        create::CreateMailGroupAliasRequest::new(self.config.clone(), self.mailgroup_id.clone())
    }

    /// 创建删除请求。
    pub fn delete(&self, alias_id: impl Into<String>) -> delete::DeleteMailGroupAliasRequest {
        delete::DeleteMailGroupAliasRequest::new(
            self.config.clone(),
            self.mailgroup_id.clone(),
            alias_id,
        )
    }
}
