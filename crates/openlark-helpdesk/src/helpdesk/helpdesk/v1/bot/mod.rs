//! 机器人模块
//!
//! 提供服务台机器人相关功能。

pub mod message;

use openlark_core::config::Config;
use std::sync::Arc;

/// 机器人服务
///
/// 提供服务台机器人相关功能。
#[derive(Clone)]
pub struct Bot {
    config: Arc<Config>,
}

impl Bot {
    /// 创建新的机器人服务实例
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 获取消息服务
    pub fn message(&self) -> message::Message {
        message::Message::new(self.config.clone())
    }
}
