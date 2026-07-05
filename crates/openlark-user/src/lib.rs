//! # OpenLark 用户设置模块
//!
//! OpenLark SDK 的用户设置模块，提供飞书个人设置（system_status）相关 API 的访问。
//!
//! ## 功能特性
//!
//! - **个人设置**: system_status 资源（list / get / create / patch / delete / batch_open / batch_close）
//!
//! ## 模块组织
//!
//! 本模块按功能域组织：
//! - `personal_settings` - 个人设置 system_status API（门面 `UserService::personal_settings()` 收敛）
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use openlark_user::UserService;
//! use openlark_core::prelude::Config;
//!
//! let config = Config::builder()
//!     .app_id("app_id")
//!     .app_secret("app_secret")
//!     .build();
//!
//! let user_service = UserService::new(config).unwrap();
//!
//! // 经门面 accessor 获取个人设置服务（system_status 7 个真实构建器）
//! let system_status = user_service.personal_settings().system_status();
//! let _list_req = system_status.list();
//! ```

mod service;

// 通用模块
/// 用户设置共享模型。
pub mod common;
/// 个人设置模块（system_status）。
pub mod personal_settings;

// Prelude 模块
/// 常用类型预导出模块。
pub mod prelude;

// 重新导出核心服务
/// 用户服务统一入口。
pub use service::UserService;

/// 用户服务客户端类型别名（统一命名为 `XxxClient`）。
pub type UserClient = UserService;
/// `openlark-core` 配置类型的便捷导出。
pub use openlark_core::config::Config;

/// 用户设置模块版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 用户服务配置别名
pub type UserConfig = Config;

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_ne!(VERSION, "");
    }

    #[test]
    fn test_module_composition() {
        // 验证模块组织正确
        assert_eq!(VERSION, env!("CARGO_PKG_VERSION"));
    }
}
