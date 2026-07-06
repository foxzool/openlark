//! Spark directory 相关 API

use crate::PlatformConfig;
use std::sync::Arc;

pub mod user;

/// Spark directory 服务
#[derive(Debug, Clone)]
pub struct SparkDirectoryService {
    config: Arc<PlatformConfig>,
}

impl SparkDirectoryService {
    /// 创建新的 Spark directory 服务实例
    pub fn new(config: Arc<PlatformConfig>) -> Self {
        Self { config }
    }

    /// user 资源
    pub fn user(&self) -> user::DirectoryUserService {
        user::DirectoryUserService::new(self.config.as_ref().clone())
    }
}
