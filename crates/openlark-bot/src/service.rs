use openlark_core::config::Config;
use std::sync::Arc;

/// BotService：机器人服务的统一入口
///
/// 提供对机器人搜索 API（v4）的访问能力。
#[derive(Clone)]
pub struct BotService {
    // config 仅在 v4 feature 开启时被 search_bot() accessor 读取；feature 关闭时受控标注为预期死代码。
    #[cfg_attr(not(feature = "v4"), expect(dead_code))]
    config: Arc<Config>,
}

impl BotService {
    /// 创建新的机器人服务实例。
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    /// 搜索机器人请求构建器（直达 leaf）。
    ///
    /// ADR 0001：消除 `Bot` / `V4` / `BotResource` 3 层纯转发壳
    /// （原 `service.bot().v4().bot().search()` 4 跳 → `service.search_bot()` 1 跳）。
    /// leaf `SearchBotRequest` API 不变。
    #[cfg(feature = "v4")]
    pub fn search_bot(&self) -> crate::bot::bot::v4::bot::search::SearchBotRequest {
        crate::bot::bot::v4::bot::search::SearchBotRequest::new(self.config.clone())
    }
}
