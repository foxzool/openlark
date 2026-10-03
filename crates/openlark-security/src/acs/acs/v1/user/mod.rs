//! 门禁用户管理端点（Transport 实现）。
//!
//! 真实请求逻辑在各子模块；[`crate::acs::acs::v1::users::UsersService`] 是返回这些构建器的门面。

pub mod create;
pub mod delete;
pub mod face;
pub mod get;
pub mod list;
pub mod patch;
