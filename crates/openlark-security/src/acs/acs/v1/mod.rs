//! ACS v1 API 版本实现
//!
//! 模块组织：
//! - **复数 `*_s/` 目录**（`users`、`devices`、…）= 门面 Service，返回端点 Request 构建器。
//! - **单数 `*/` 目录**（`user`、`device`、…）= 真实端点实现（Transport + validate_required!）。

// 端点实现（单数目录，真实 Transport 实现）
pub mod access_record;
pub mod client_device;
pub mod device;
pub mod face;
pub mod openapi_audit;
pub mod rule;
pub mod rule_external;
pub mod user;
pub mod visitor;

// 门面 Service（复数目录，返回上面的 Request 构建器）
pub mod access_records;
pub mod devices;
pub mod user_faces;
pub mod users;
pub mod visitors;
