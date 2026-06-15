/// 获取接口。
pub mod get;
/// get_by_card 模块。
pub mod get_by_card;
/// 列表接口。
pub mod list;
/// recall 模块（邮件撤回）。
pub mod recall;
/// send 模块。
pub mod send;
/// send_status 模块（查询发送状态）。
pub mod send_status;

use openlark_core::config::Config;
use std::sync::Arc;

/// 用户邮箱消息资源。
#[derive(Clone)]
pub struct Message {
    config: Arc<Config>,
    mailbox_id: String,
}

impl Message {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
        }
    }

    /// 创建列表请求。
    pub fn list(&self) -> list::ListMailboxMessageRequest {
        list::ListMailboxMessageRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 创建获取详情请求。
    pub fn get(&self, message_id: impl Into<String>) -> get::GetMailboxMessageRequest {
        get::GetMailboxMessageRequest::new(self.config.clone(), self.mailbox_id.clone(), message_id)
    }

    /// get_by_card。
    pub fn get_by_card(&self) -> get_by_card::GetMailboxMessageByCardRequest {
        get_by_card::GetMailboxMessageByCardRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
        )
    }

    /// send。
    pub fn send(&self) -> send::SendMailboxMessageRequest {
        send::SendMailboxMessageRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 创建查询发送状态请求。
    pub fn send_status(
        &self,
        message_id: impl Into<String>,
    ) -> send_status::GetMailSendStatusRequest {
        send_status::GetMailSendStatusRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            message_id,
        )
    }

    /// 访问邮件撤回资源。
    pub fn recall(&self, message_id: impl Into<String>) -> recall::Recall {
        recall::Recall::new(self.config.clone(), self.mailbox_id.clone(), message_id)
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
/// batch_get 模块。
pub mod batch_get;
/// batch_modify 模块。
pub mod batch_modify;
/// batch_trash 模块。
pub mod batch_trash;
/// list_thread_message 模块。
pub mod list_thread_message;
/// modify 模块。
pub mod modify;
/// trash 模块。
pub mod trash;
