#![allow(clippy::module_inception)]
//! # OpenLark 商店付费模块
//!
//! 提供飞书应用商店付费相关 API：查询订单详情、租户订单列表、用户开通范围。

mod service;

/// 共享模块。
pub mod common;

#[cfg(feature = "v1")]
/// 商店付费 API 模块。
pub mod pay;

/// 商店付费服务统一入口。
pub use service::PayService;

/// 商店付费客户端类型别名（统一命名为 `XxxClient`）。
pub type PayClient = PayService;

/// 当前 crate 版本号。
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_ne!(VERSION, "");
    }
}
