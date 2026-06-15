//! 搜索机器人
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/bot-v4/bot/search

use openlark_core::config::Config;
use std::sync::Arc;

/// 搜索机器人请求。
#[derive(Debug, Clone)]
pub struct SearchBotRequest {
    #[allow(dead_code)]
    config: Arc<Config>,
}

impl SearchBotRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}
