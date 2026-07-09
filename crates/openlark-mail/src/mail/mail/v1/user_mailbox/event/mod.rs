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
pub mod subscription;
