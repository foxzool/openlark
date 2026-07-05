//! # OpenLark 应用管理模块
//!
//! OpenLark SDK 的应用管理模块，提供飞书应用 API 的完整访问。

#![allow(clippy::module_inception)]

mod service;

/// 应用管理模块的通用工具与端点定义。
pub mod common;

/// 飞书应用 API（v1/v5/v6/v7 各自独立 feature 门控）。
#[cfg(any(feature = "v1", feature = "v5", feature = "v6", feature = "v7"))]
pub mod application;

/// Workplace 场景相关 API。
#[cfg(feature = "workplace")]
pub mod workplace;

/// 应用管理模块常用预导出。
pub mod prelude;

/// 应用管理服务统一入口。
pub use service::ApplicationService;

/// 应用服务客户端类型别名（统一命名为 `XxxClient`）。
pub type ApplicationClient = ApplicationService;

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
