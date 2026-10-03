//! 门禁设备管理端点（Transport 实现）。
//!
//! 真实请求逻辑在各子模块；[`crate::acs::acs::v1::devices::DevicesService`] 是返回这些构建器的门面。

pub mod approve;
pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod query;
pub mod update;
