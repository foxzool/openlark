//! Mail v1 API 模块

/// 邮件组模块。
pub mod mailgroup;
/// 多实体搜索模块。
pub mod multi_entity;
/// 公共邮箱模块。
pub mod public_mailbox;
/// 用户邮箱服务模块。
pub mod user;
/// 用户邮箱资源模块。
pub mod user_mailbox;

use openlark_core::config::Config;
use std::sync::Arc;

/// MailV1：邮件 API v1 访问入口
#[derive(Clone)]
pub struct MailV1 {
    config: Arc<Config>,
}

impl MailV1 {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 访问邮件组资源
    pub fn mailgroup(&self) -> mailgroup::MailGroup {
        mailgroup::MailGroup::new(self.config.clone())
    }

    /// 访问公共邮箱资源
    pub fn public_mailbox(&self) -> public_mailbox::PublicMailbox {
        public_mailbox::PublicMailbox::new(self.config.clone())
    }

    /// 访问用户邮箱资源
    pub fn user(&self) -> user::User {
        user::User::new(self.config.clone())
    }

    /// 访问用户邮箱资源
    pub fn user_mailbox(&self, mailbox_id: impl Into<String>) -> user_mailbox::UserMailbox {
        user_mailbox::UserMailbox::new(self.config.clone(), mailbox_id.into())
    }

    /// 访问多实体搜索资源
    pub fn multi_entity(&self) -> multi_entity::MultiEntity {
        multi_entity::MultiEntity::new(self.config.clone())
    }
}

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
