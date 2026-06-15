//! 多实体搜索模块

pub mod search;

use openlark_core::config::Config;
use std::sync::Arc;

/// MultiEntity：多实体搜索资源入口
#[derive(Clone)]
pub struct MultiEntity {
    config: Arc<Config>,
}

impl MultiEntity {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 创建多实体搜索请求。
    pub fn search(&self) -> search::MultiEntitySearchRequest {
        search::MultiEntitySearchRequest::new(self.config.clone())
    }
}
