/// 别名模块。
pub mod alias;
/// 创建接口。
pub mod create;
/// 删除接口。
pub mod delete;
/// 获取接口。
pub mod get;
/// 列表接口。
pub mod list;
/// 管理员模块。
pub mod manager;
/// 成员模块。
pub mod member;
/// 数据模型。
pub mod models;
/// 更新接口。
pub mod patch;
/// 权限成员模块。
pub mod permission_member;
/// 更新接口。
pub mod update;

use openlark_core::config::Config;
use std::sync::Arc;

/// MailGroup：邮件组资源（v1）
#[derive(Clone)]
pub struct MailGroup {
    config: Arc<Config>,
}

impl MailGroup {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 创建新建请求。
    pub fn create(&self) -> create::CreateMailGroupRequest {
        create::CreateMailGroupRequest::new(self.config.clone())
    }

    /// 创建获取详情请求。
    pub fn get(&self, mail_group_id: impl Into<String>) -> get::GetMailGroupRequest {
        get::GetMailGroupRequest::new(self.config.clone(), mail_group_id.into())
    }

    /// 创建更新请求。
    pub fn update(&self, mail_group_id: impl Into<String>) -> update::UpdateMailGroupRequest {
        update::UpdateMailGroupRequest::new(self.config.clone(), mail_group_id.into())
    }

    /// 创建删除请求。
    pub fn delete(&self, mail_group_id: impl Into<String>) -> delete::DeleteMailGroupRequest {
        delete::DeleteMailGroupRequest::new(self.config.clone(), mail_group_id.into())
    }

    /// 创建列表请求。
    pub fn list(&self) -> list::MailGroupListRequest {
        list::MailGroupListRequest::new(self.config.clone())
    }
}

// 重新导出请求类型
pub use create::CreateMailGroupRequest;
pub use delete::DeleteMailGroupRequest;
pub use get::GetMailGroupRequest;
pub use list::MailGroupListRequest;
pub use update::UpdateMailGroupRequest;

// 重新导出响应类型
pub use models::{
    CreateMailGroupBody, CreateMailGroupResponse, DeleteMailGroupResponse, GetMailGroupResponse,
    MailGroupItem, MailGroupListResponse, UpdateMailGroupBody, UpdateMailGroupResponse,
};
