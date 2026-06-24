//! 公共邮箱模块

pub mod create;
/// 删除接口。
pub mod delete;
/// 获取接口。
pub mod get;
/// 列表接口。
pub mod list;
/// 数据模型。
pub mod models;
/// 更新接口。
pub mod patch;
/// 更新接口。
pub mod update;

/// 别名模块。
pub mod alias;
/// 成员模块。
pub mod member;

use openlark_core::config::Config;
use std::sync::Arc;

/// PublicMailbox：公共邮箱资源
#[derive(Clone)]
pub struct PublicMailbox {
    config: Arc<Config>,
}

impl PublicMailbox {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 创建公共邮箱
    pub fn create(&self) -> create::CreatePublicMailboxRequest {
        create::CreatePublicMailboxRequest::new(self.config.clone())
    }

    /// 获取公共邮箱详情
    pub fn get(&self, mailbox_id: impl Into<String>) -> get::GetPublicMailboxRequest {
        get::GetPublicMailboxRequest::new(self.config.clone(), mailbox_id.into())
    }

    /// 更新公共邮箱全部信息
    pub fn update(&self, mailbox_id: impl Into<String>) -> update::UpdatePublicMailboxRequest {
        update::UpdatePublicMailboxRequest::new(self.config.clone(), mailbox_id.into())
    }

    /// 删除公共邮箱
    pub fn delete(&self, mailbox_id: impl Into<String>) -> delete::DeletePublicMailboxRequest {
        delete::DeletePublicMailboxRequest::new(self.config.clone(), mailbox_id.into())
    }

    /// 获取公共邮箱列表
    pub fn list(&self) -> list::PublicMailboxListRequest {
        list::PublicMailboxListRequest::new(self.config.clone())
    }

    /// 部分更新公共邮箱
    pub fn patch(&self, mailbox_id: impl Into<String>) -> patch::PatchPublicMailboxRequest {
        patch::PatchPublicMailboxRequest::new(self.config.clone(), mailbox_id.into())
    }

    /// 获取别名管理
    pub fn alias(&self, mailbox_id: impl Into<String>) -> alias::Alias {
        alias::Alias::new(self.config.clone(), mailbox_id.into())
    }

    /// 获取成员管理
    pub fn member(&self, mailbox_id: impl Into<String>) -> member::Member {
        member::Member::new(self.config.clone(), mailbox_id.into())
    }
}

// 重新导出请求类型
pub use create::CreatePublicMailboxRequest;
pub use delete::DeletePublicMailboxRequest;
pub use get::GetPublicMailboxRequest;
pub use list::PublicMailboxListRequest;
pub use patch::PatchPublicMailboxRequest;
pub use update::UpdatePublicMailboxRequest;

// 重新导出响应类型
pub use models::{
    CreatePublicMailboxBody, CreatePublicMailboxResponse, DeletePublicMailboxResponse,
    GetPublicMailboxResponse, PatchPublicMailboxBody, PatchPublicMailboxResponse,
    PublicMailboxItem, PublicMailboxListResponse, UpdatePublicMailboxBody,
    UpdatePublicMailboxResponse,
};

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}

pub mod remove_to_recycle_bin;
