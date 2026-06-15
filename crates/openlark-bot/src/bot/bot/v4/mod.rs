//! Bot v4 模块

pub mod bot;

use openlark_core::config::Config;
use std::sync::Arc;

/// V4：机器人 v4 版本入口
#[derive(Clone)]
pub struct V4 {
    config: Arc<Config>,
}

impl V4 {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 访问 bot 资源。
    pub fn bot(&self) -> bot::BotResource {
        bot::BotResource::new(self.config.clone())
    }
}
