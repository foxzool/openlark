//! bot 资源 v4

pub mod search;

use openlark_core::config::Config;
use std::sync::Arc;

/// BotResource：机器人资源
#[derive(Clone)]
pub struct BotResource {
    config: Arc<Config>,
}

impl BotResource {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 创建搜索机器人请求。
    pub fn search(&self) -> search::SearchBotRequest {
        search::SearchBotRequest::new(self.config.clone())
    }
}
