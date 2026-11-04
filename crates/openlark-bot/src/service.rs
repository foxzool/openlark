use openlark_core::config::Config;
use std::sync::Arc;

/// BotService：机器人服务的统一入口
///
/// 提供机器人信息（v3）、千人千面菜单（v3）和搜索机器人（v4）。
#[derive(Clone)]
pub struct BotService {
    // config 只在 v3/v4 入口读取；两个 feature 都关闭时这里是预期死代码。
    #[cfg_attr(not(any(feature = "v3", feature = "v4")), expect(dead_code))]
    config: Arc<Config>,
}

impl BotService {
    /// 创建新的机器人服务实例。
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    /// 获取当前应用机器人信息。
    ///
    /// 端点、租户 token 和响应模型都在这个请求上。
    #[cfg(feature = "v3")]
    pub fn bot_info(&self) -> crate::bot::bot::v3::info::GetBotInfoRequest {
        crate::bot::bot::v3::info::GetBotInfoRequest::new(Arc::clone(&self.config))
    }

    /// 获取指定用户可见的千人千面菜单。
    #[cfg(feature = "v3")]
    pub fn get_bot_menu(&self) -> crate::bot::bot::v3::bot_menu::get_bot_menu::GetBotMenuRequest {
        crate::bot::bot::v3::bot_menu::get_bot_menu::GetBotMenuRequest::new(
            self.config.as_ref().clone(),
        )
    }

    /// 为指定用户创建仅其可见的千人千面菜单。
    #[cfg(feature = "v3")]
    pub fn set_bot_menu(&self) -> crate::bot::bot::v3::bot_menu::set_bot_menu::SetBotMenuRequest {
        crate::bot::bot::v3::bot_menu::set_bot_menu::SetBotMenuRequest::new(
            self.config.as_ref().clone(),
        )
    }

    /// 删除指定用户可见的千人千面菜单。
    #[cfg(feature = "v3")]
    pub fn delete_bot_menu(
        &self,
    ) -> crate::bot::bot::v3::bot_menu::delete_bot_menu::DeleteBotMenuRequest {
        crate::bot::bot::v3::bot_menu::delete_bot_menu::DeleteBotMenuRequest::new(
            self.config.as_ref().clone(),
        )
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
