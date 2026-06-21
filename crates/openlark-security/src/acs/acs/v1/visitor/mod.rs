//! 访客管理端点（Transport 实现）。
//!
//! 真实请求逻辑在各子模块；[`crate::acs::acs::v1::visitors::VisitorsService`] 是返回
//! 这些构建器的门面。

pub mod create;
pub mod delete;
