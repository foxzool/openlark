#![allow(dead_code)]
#![allow(clippy::module_inception)]
//! # OpenLark 机器人模块
//!
//! OpenLark SDK 的机器人模块，提供飞书机器人搜索 API。
//!
//! ## 功能特性
//!
//! - **机器人搜索**: 按关键词搜索当前用户可见的机器人

mod service;

// bot 模块
#[cfg(feature = "v4")]
/// 机器人 API 模块。
pub mod bot;

// 重新导出核心服务
/// 机器人服务统一入口。
pub use service::BotService;

/// 机器人服务客户端类型别名（统一命名为 `XxxClient`）。
pub type BotClient = BotService;

/// 机器人模块版本信息
/// 当前 crate 版本号。
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_ne!(VERSION, "");
    }
}
