//! 通讯模块通用工具
//!
//! 提供 API 实现中常用的工具函数与宏，减少重复代码。

pub mod api_utils;
pub mod chain;
#[cfg(feature = "im")]
pub(crate) mod message_card;
pub mod models;
