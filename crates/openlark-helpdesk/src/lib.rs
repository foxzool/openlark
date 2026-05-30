//! # OpenLark 帮助台模块
//!
//! OpenLark SDK 的帮助台模块，提供飞书帮助台 API 的完整访问。

mod service;

/// 帮助台通用工具与端点定义。
pub mod common;

#[cfg(feature = "v1")]
/// helpdesk 模块。
pub mod helpdesk;

/// 帮助台模块常用预导出。
pub mod prelude;

pub use service::HelpdeskService;

/// 帮助台服务客户端类型别名（统一命名为 `XxxClient`）。
pub type HelpdeskClient = HelpdeskService;

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
