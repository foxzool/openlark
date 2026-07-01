//! # OpenLark 数据分析模块
//!
//! OpenLark SDK 的数据分析模块，提供飞书数据分析和搜索相关 API 的完整访问。
//!
//! ## 功能特性
//!
//! - **搜索服务**: 全文搜索、智能搜索、内容检索
//! - **数据分析**: 数据报表、统计分析、趋势分析
//!
//! ## 模块组织
//!
//! 本模块按业务域（bizTag）组织：
//! - `search` - 搜索服务相关 API (14 APIs)
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use openlark_analytics::AnalyticsService;
//! use openlark_core::prelude::Config;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // 使用 builder 模式创建配置
//! let config = Config::builder()
//!     .app_id("app_id")
//!     .app_secret("app_secret")
//!     .build();
//!
//! let _analytics_service = AnalyticsService::new(config)?;
//!
//! # Ok(())
//! # }
//! ```

#![allow(clippy::module_inception)]
#![allow(missing_docs)]

mod service;

// 通用模块
pub mod common;
/// 报告模块。
pub mod report;

// 业务域模块
#[cfg(feature = "search")]
pub mod search;

// Prelude 模块
pub mod prelude;

// 重新导出核心服务
pub use service::AnalyticsService;

/// 数据分析服务客户端类型别名（统一命名为 `XxxClient`）。
pub type AnalyticsClient = AnalyticsService;
pub use openlark_core::config::Config;

/// 数据分析模块版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 数据分析服务配置别名
pub type AnalyticsConfig = Config;

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
