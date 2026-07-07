//! 机器人消息模块
//!
//! 通过服务台机器人向工单绑定的群内发送消息。

pub mod create;

use openlark_core::config::Config;
use std::sync::Arc;

/// 机器人消息服务
///
/// 提供通过服务台机器人发送消息的功能。
#[derive(Clone)]
pub struct Message {
    config: Arc<Config>,
}

impl Message {
    /// 创建新的机器人消息服务实例
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 通过服务台机器人发送消息
    pub fn create(&self) -> create::CreateBotMessageRequestBuilder {
        create::CreateBotMessageRequestBuilder::new(self.config.clone())
    }
}
