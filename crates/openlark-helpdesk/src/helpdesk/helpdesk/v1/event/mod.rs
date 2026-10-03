//! 事件订阅模块
//!
//! 提供服务台事件订阅和取消订阅功能。

pub mod subscribe;
/// unsubscribe 模块。
pub mod unsubscribe;

use openlark_core::config::Config;
use std::sync::Arc;

/// 事件服务
///
/// 提供服务台事件订阅和取消订阅功能。
#[derive(Clone)]
pub struct Event {
    config: Arc<Config>,
}

impl Event {
    /// 创建新的事件服务实例
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 订阅服务台事件
    pub fn subscribe(&self) -> subscribe::EventSubscribeRequestBuilder {
        subscribe::EventSubscribeRequestBuilder::new(self.config.clone())
    }

    /// 取消订阅服务台事件
    pub fn unsubscribe(&self) -> unsubscribe::EventUnsubscribeRequestBuilder {
        unsubscribe::EventUnsubscribeRequestBuilder::new(self.config.clone())
    }
}
