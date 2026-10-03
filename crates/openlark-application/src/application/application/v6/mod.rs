//! 应用 API v6 访问入口。
//!
//! 残破 stub（双 `application` path / catalog 无端点 / 错误 method 壳）已在 #382 移除。
//! 正确入口在 `application` 资源树、`app_badge`、`app_recommend_rule`、`scope`。

use openlark_core::config::Config;
use std::sync::Arc;

/// ApplicationV6：应用 API v6 访问入口
#[derive(Clone)]
pub struct ApplicationV6 {
    // 保留 config 供后续子资源 accessor 使用；当前保留模块经路径类型直接构造
    _config: Arc<Config>,
}

impl ApplicationV6 {
    /// 创建新的 ApplicationV6 实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { _config: config }
    }
}

pub mod app_badge;
pub mod app_recommend_rule;
pub mod application;
pub mod scope;
