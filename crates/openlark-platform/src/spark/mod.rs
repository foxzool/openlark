//! 妙搭平台模块
//!
//! 提供妙搭平台相关 API 的访问入口。

use crate::PlatformConfig;
use std::sync::Arc;

/// 妙搭平台服务
#[derive(Debug, Clone)]
pub struct SparkService {
    config: Arc<PlatformConfig>,
}

impl SparkService {
    /// 创建新的妙搭平台服务实例
    pub fn new(config: Arc<PlatformConfig>) -> Self {
        Self { config }
    }

    /// 获取客户端配置
    pub fn config(&self) -> Arc<PlatformConfig> {
        self.config.clone()
    }

    /// V1 版本 API
    pub fn v1(&self) -> crate::spark::spark::v1::SparkV1 {
        crate::spark::spark::v1::SparkV1::new(self.config.clone())
    }
}

pub mod spark;

#[cfg(test)]
mod tests {
    use crate::{PlatformConfig, spark::SparkService};

    #[test]
    fn test_service_creation() {
        let config = PlatformConfig::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let service = SparkService::new(std::sync::Arc::new(config));
        assert_eq!(service.config().app_id(), "test_app_id");
    }
}
