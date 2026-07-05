//! Application API 模块
//!
//! 各版本经独立 feature 门控（v1/v5/v6/v7），不再统一搭 v1 feature 车。

/// 应用管理 v1 版本 API。
#[cfg(feature = "v1")]
pub mod v1;
/// 应用管理 v5 版本 API。
#[cfg(feature = "v5")]
pub mod v5;
/// 应用管理 v6 版本 API。
#[cfg(feature = "v6")]
pub mod v6;
/// 应用管理 v7 版本 API。
#[cfg(feature = "v7")]
pub mod v7;
