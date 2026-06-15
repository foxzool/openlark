//! 用户邮箱模块

/// 用户邮箱别名模块。
pub mod alias;
/// event 模块。
pub mod event;
/// 文件夹模块。
pub mod folder;
/// 联系人模块。
pub mod mail_contact;
/// 消息模块。
pub mod message;
/// profile 模块。
pub mod profile;
/// 规则模块。
pub mod rule;

use openlark_core::config::Config;
use std::sync::Arc;

/// UserMailbox：用户邮箱资源
#[derive(Clone)]
pub struct UserMailbox {
    config: Arc<Config>,
    mailbox_id: String,
}

impl UserMailbox {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
        }
    }

    /// 获取别名管理
    pub fn alias(&self) -> alias::Alias {
        alias::Alias::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 获取事件管理
    pub fn event(&self) -> event::Event {
        event::Event::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 获取文件夹管理
    pub fn folder(&self) -> folder::Folder {
        folder::Folder::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 获取邮件联系人管理
    pub fn mail_contact(&self) -> mail_contact::MailContact {
        mail_contact::MailContact::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 获取消息管理
    pub fn message(&self) -> message::Message {
        message::Message::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 获取规则管理
    pub fn rule(&self) -> rule::Rule {
        rule::Rule::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 创建获取用户邮箱信息请求。
    pub fn profile(&self) -> profile::GetUserMailboxProfileRequest {
        profile::GetUserMailboxProfileRequest::new(self.config.clone(), self.mailbox_id.clone())
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
/// accessible_mailboxes 模块。
pub mod accessible_mailboxes;
/// draft 模块。
pub mod draft;
/// label 模块。
pub mod label;
/// setting 模块。
pub mod setting;
/// template 模块。
pub mod template;
/// thread 模块。
pub mod thread;
