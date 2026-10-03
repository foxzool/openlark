//! 开通范围。

/// 查询用户是否在应用开通范围。
pub mod check_user;

pub use check_user::{CheckUserPaidScopeRequest, CheckUserPaidScopeResponse, PaidScopeStatus};
