#![allow(dead_code)]
#![allow(clippy::module_inception)]
//! # OpenLark 邮件模块
//!
//! OpenLark SDK 的邮件模块，提供飞书邮件组 API 的完整访问。
//!
//! ## 功能特性
//!
//! - **邮件组管理**: 创建、更新、删除、查询邮件组
//! - **成员管理**: 添加、删除邮件组成员
//! - **别名管理**: 邮件别名配置
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use openlark_mail::MailService;
//! use openlark_core::config::Config;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = Config::builder()
//!     .app_id("your_app_id")
//!     .app_secret("your_app_secret")
//!     .build();
//!
//! let mail_service = MailService::new(config);
//!
//! // 创建邮件组
//! let result = mail_service
//!     .mailgroup()
//!     .create()
//!     .mail_group_id("team@example.com")
//!     .description("项目团队邮件组")
//!     .execute()
//!     .await?;
//! # Ok(())
//! # }
//! ```

mod service;

// 通用模块
/// 邮件模块通用工具与端点定义。
pub mod common;

// mail 模块
#[cfg(feature = "v1")]
/// 邮件 API 模块。
pub mod mail;

// Prelude 模块
/// 邮件模块常用预导出。
pub mod prelude;

// 重新导出核心服务
/// 邮件服务统一入口。
pub use service::MailService;

/// 邮件服务客户端类型别名（统一命名为 `XxxClient`）。
pub type MailClient = MailService;

/// 邮件模块版本信息
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
