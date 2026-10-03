//! 通用工具模块（私有，#330：HTTP 管道 helper 已下沉 core::api）。
//!
//! 叶子直接经 `crate::common::api_utils::<fn>` 访问（api_utils re-export core canonical）。

pub mod api_utils;
