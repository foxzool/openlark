//! # OpenLark 平台服务模块
//!
//! OpenLark SDK 的平台服务模块，提供飞书平台管理相关 API 的完整访问。
//!
//! ## 功能特性
//!
//! - **应用引擎**: 应用管理、多租户、应用市场集成
//! - **目录服务**: 用户搜索、组织目录、人员查找
//! - **系统管理**: 系统配置、后台管理、平台工具
//!
//! ## 模块组织
//!
//! 本模块按业务域（bizTag）组织，分两类入口（ADR 0001）：
//! - **Service accessor 入口**（含路径参数绑定层，经 `PlatformService::xxx()`）：
//!   `app_engine`（37 APIs）、`directory`（21 APIs）、`admin`（14 APIs）、`spark`（1 API）
//! - **flat-by-design 直路径**（叶子 `new(Config)` 无路径参数，无 Service 壳，同 analytics 裁决）：
//!   `mdm`、`tenant`、`trust_party`
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use openlark_platform::PlatformService;
//! use openlark_core::prelude::Config;
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // 使用 builder 模式创建配置
//! let config = Config::builder()
//!     .app_id("app_id")
//!     .app_secret("app_secret")
//!     .build();
//!
//! let platform_service = PlatformService::new(config)?;
//!
//! // 具体功能请参考各个子模块的文档
//! # Ok(())
//! # }
//! ```

mod service;

// 通用模块
pub mod common;

// 业务域模块
#[cfg(feature = "app-engine")]
pub mod app_engine;

#[cfg(feature = "directory")]
pub mod directory;

#[cfg(feature = "admin")]
pub mod admin;

// flat-by-design 域（无 Service 壳，直路径访问，ADR 0001）
#[cfg(feature = "mdm")]
pub mod mdm;

#[cfg(feature = "tenant")]
pub mod tenant;

#[cfg(feature = "trust_party")]
pub mod trust_party;

#[cfg(feature = "spark")]
pub mod spark;

// Prelude 模块
pub mod prelude;

// 重新导出核心服务
pub use service::PlatformService;

/// 平台服务客户端类型别名（统一命名为 `XxxClient`）。
pub type PlatformClient = PlatformService;
pub use openlark_core::config::Config;

/// 平台服务模块版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 平台服务配置别名
pub type PlatformConfig = Config;

#[cfg(test)]
mod tests {
    use crate::VERSION;

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

#[cfg(test)]
mod service_tests {
    use super::*;
    use openlark_core::constants::AppType;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .app_type(AppType::SelfBuild)
            .build()
    }

    #[test]
    fn test_platform_service_creation() {
        let config = create_test_config();
        let service = PlatformService::new(config).unwrap();
        assert!(service.config().app_id() == "test_app");
    }

    #[test]
    fn test_platform_service_clone() {
        let config = create_test_config();
        let service = PlatformService::new(config).unwrap();
        let cloned = service.clone();
        assert!(cloned.config().app_id() == "test_app");
    }

    #[test]
    fn test_platform_config_alias() {
        let config: PlatformConfig = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        assert!(config.app_id() == "test_app");
    }

    #[cfg(feature = "app-engine")]
    #[test]
    fn test_platform_service_app_engine() {
        let config = create_test_config();
        let service = PlatformService::new(config).unwrap();
        let _app_engine = service.app_engine();
    }

    #[cfg(feature = "directory")]
    #[test]
    fn test_platform_service_directory() {
        let config = create_test_config();
        let service = PlatformService::new(config).unwrap();
        let _directory = service.directory();
    }

    #[cfg(feature = "admin")]
    #[test]
    fn test_platform_service_admin() {
        let config = create_test_config();
        let service = PlatformService::new(config).unwrap();
        let _admin = service.admin();
    }

    #[cfg(feature = "spark")]
    #[test]
    fn test_platform_service_spark() {
        let config = create_test_config();
        let service = PlatformService::new(config).unwrap();
        let _request = service.spark().v1().directory().user().id_convert();
    }
}
