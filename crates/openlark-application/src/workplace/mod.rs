//! 工作台模块
//!
//! 提供工作台访问数据统计相关 API。

pub mod workplace;

use openlark_core::config::Config;
use std::sync::Arc;

/// WorkplaceService：工作台服务入口。
#[derive(Clone)]
pub struct WorkplaceService {
    config: Arc<Config>,
}

impl WorkplaceService {
    /// 创建新的 WorkplaceService 实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 访问 v1 版本工作台 API。
    pub fn v1(&self) -> crate::workplace::workplace::v1::WorkplaceV1 {
        crate::workplace::workplace::v1::WorkplaceV1::new(self.config.clone())
    }
}
