//! 人脸资源端点（Transport 实现）。
//!
//! 注：这是独立的人脸资源（`/acs/v1/faces/{face_id}`），与用户人脸
//! （`user/face/*`，即 `/acs/v1/users/{user_id}/face`）是不同端点。

pub mod create;
pub mod delete;
pub mod get;
