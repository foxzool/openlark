//! 即时通讯模块
//!
//! 提供 IM 业务入口与 v1/v2 版本 API 路由。

// 先按 meta.Project 建目录，后续逐个补齐
/// Card 模块。
pub mod card;
#[path = "im/mod.rs"]
mod project;
/// IM v1/v2 版本入口。
pub use project::{v1, v2};
/// Im Ephemeral 模块。
pub mod im_ephemeral;
/// IM Message 模块。
pub mod im_message;
