//! 千人千面菜单。
//!
//! 三个接口共用 `POST|GET|DELETE /open-apis/bot/v3/bot_menu`，只接受 tenant access token。

pub mod delete_bot_menu;
pub mod get_bot_menu;
pub mod models;
pub mod set_bot_menu;

/// 千人千面菜单端点。创建、获取、删除是同一路径上的不同方法。
pub const BOT_V3_BOT_MENU: &str = "/open-apis/bot/v3/bot_menu";

pub use models::{
    BotMenu, BotMenuBehavior, BotMenuIcon, BotMenuItem, BotMenuRedirectLink, BotMenuUdIcon,
};
