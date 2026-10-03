//! 应用引擎模块
//!
//! 提供应用管理、多租户、应用市场等功能 (37 APIs)

use crate::PlatformConfig;
use std::sync::Arc;

/// 应用引擎服务
///
/// 提供应用引擎相关 API 的访问入口。
#[derive(Debug, Clone)]
pub struct AppEngineService {
    /// 客户端配置
    config: Arc<PlatformConfig>,
}

impl AppEngineService {
    /// 创建新的应用引擎服务实例
    pub fn new(config: Arc<PlatformConfig>) -> Self {
        Self { config }
    }

    /// 获取客户端配置
    pub fn config(&self) -> Arc<PlatformConfig> {
        self.config.clone()
    }

    /// V1 版本 API
    pub fn v1(&self) -> crate::app_engine::apaas::v1::ApaasV1 {
        crate::app_engine::apaas::v1::ApaasV1::new(self.config.clone())
    }
}

pub mod apaas;

#[cfg(test)]
mod tests {
    use crate::{PlatformConfig, app_engine::AppEngineService};

    #[test]
    fn test_service_creation() {
        let config = PlatformConfig::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let service = AppEngineService::new(std::sync::Arc::new(config));
        // PlatformConfig 实现了 Deref，可以直接访问 app_id
        assert_eq!(service.config().app_id(), "test_app_id");
    }
}
