//! Spark V1 API

use crate::PlatformConfig;
use std::sync::Arc;

/// app 模块。
pub mod app;
/// directory 模块。
pub mod directory;

/// Spark V1 API
#[derive(Debug, Clone)]
pub struct SparkV1 {
    config: Arc<PlatformConfig>,
}

impl SparkV1 {
    /// 创建新的 Spark V1 实例
    pub fn new(config: Arc<PlatformConfig>) -> Self {
        Self { config }
    }

    /// directory 资源
    pub fn directory(&self) -> directory::SparkDirectoryService {
        directory::SparkDirectoryService::new(self.config.clone())
    }

    /// app 资源
    pub fn app(&self) -> app::SparkAppService {
        app::SparkAppService::new(self.config.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::SparkV1;

    #[test]
    fn test_spark_v1_directory_access() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let api = SparkV1::new(std::sync::Arc::new(config));
        let _ = api.directory();
    }

    #[test]
    fn spark_app_issue_194_app_access() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let api = SparkV1::new(std::sync::Arc::new(config));
        let _ = api.app();
    }
}
