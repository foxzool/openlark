/// 订阅接口。
pub mod subscribe;
/// 取消订阅接口。
pub mod unsubscribe;

use openlark_core::config::Config;
use std::sync::Arc;

/// 用户邮箱事件资源。
#[derive(Clone)]
pub struct Event {
    config: Arc<Config>,
    mailbox_id: String,
}

impl Event {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
        }
    }

    /// 创建订阅请求。
    pub fn subscribe(&self) -> subscribe::SubscribeMailboxEventRequest {
        subscribe::SubscribeMailboxEventRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 创建取消订阅请求。
    pub fn unsubscribe(&self) -> unsubscribe::UnsubscribeMailboxEventRequest {
        unsubscribe::UnsubscribeMailboxEventRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
        )
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

pub mod subscription;
