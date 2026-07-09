//! 应用 API v1 访问入口。
//!
//! 历史上 v1 下有大量双 `application` path 残破 stub（#382 已移除）。
//! 现仅保留仍可调用的端点（路径与 catalog 对齐的实现）。

use openlark_core::config::Config;
use std::sync::Arc;

/// ApplicationV1：应用 API v1 访问入口
#[derive(Clone)]
pub struct ApplicationV1 {
    // 保留 config 供后续子资源 accessor 使用；当前保留模块经路径类型直接构造
    _config: Arc<Config>,
}

impl ApplicationV1 {
    /// 创建新的 ApplicationV1 实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { _config: config }
    }
}

pub mod app_badge;
pub mod application;
