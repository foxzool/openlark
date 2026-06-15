//! bot 资源模块

pub mod v4;

use openlark_core::config::Config;
use std::sync::Arc;

/// Bot：机器人资源入口
#[derive(Clone)]
pub struct Bot {
    config: Arc<Config>,
}

impl Bot {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 访问 v4 API。
    pub fn v4(&self) -> v4::V4 {
        v4::V4::new(self.config.clone())
    }
}
