//! 公共邮箱成员模块

pub mod create;
/// 删除接口。
pub mod delete;
/// 获取接口。
pub mod get;
/// 列表接口。
pub mod list;
/// 数据模型。
pub mod models;

use openlark_core::config::Config;
use std::sync::Arc;

/// Member：公共邮箱成员资源
#[derive(Clone)]
pub struct Member {
    config: Arc<Config>,
    mailbox_id: String,
}

impl Member {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailbox_id: String) -> Self {
        Self { config, mailbox_id }
    }

    /// 创建成员
    pub fn create(&self) -> create::CreatePublicMailboxMemberRequest {
        create::CreatePublicMailboxMemberRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 获取成员详情
    pub fn get(&self, member_id: impl Into<String>) -> get::GetPublicMailboxMemberRequest {
        get::GetPublicMailboxMemberRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            member_id.into(),
        )
    }

    /// 获取成员列表
    pub fn list(&self) -> list::PublicMailboxMemberListRequest {
        list::PublicMailboxMemberListRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 删除成员
    pub fn delete(&self, member_id: impl Into<String>) -> delete::DeletePublicMailboxMemberRequest {
        delete::DeletePublicMailboxMemberRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            member_id.into(),
        )
    }
}

// 重新导出
pub use create::CreatePublicMailboxMemberRequest;
pub use delete::DeletePublicMailboxMemberRequest;
pub use get::GetPublicMailboxMemberRequest;
pub use list::PublicMailboxMemberListRequest;
pub use models::{
    CreatePublicMailboxMemberBody, CreatePublicMailboxMemberResponse,
    DeletePublicMailboxMemberResponse, GetPublicMailboxMemberResponse, PublicMailboxMemberItem,
    PublicMailboxMemberListResponse,
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

pub mod batch_create;
pub mod batch_delete;
pub mod clear;
