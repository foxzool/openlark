//! 系统管理模块
//!
//! 提供系统配置、后台管理、平台工具等功能 (14 APIs)

use crate::PlatformConfig;
use std::sync::Arc;

/// 系统管理服务
///
/// 提供系统管理相关 API 的访问入口。
#[derive(Debug, Clone)]
pub struct AdminService {
    /// 客户端配置
    config: Arc<PlatformConfig>,
}

impl AdminService {
    /// 创建新的系统管理服务实例
    pub fn new(config: Arc<PlatformConfig>) -> Self {
        Self { config }
    }

    /// 获取客户端配置
    pub fn config(&self) -> Arc<PlatformConfig> {
        self.config.clone()
    }

    /// V1 版本 API
    pub fn v1(&self) -> crate::admin::admin::v1::AdminV1 {
        crate::admin::admin::v1::AdminV1::new(self.config.clone())
    }
}

pub mod admin;

#[cfg(test)]
mod tests {
    use crate::{PlatformConfig, admin::AdminService};

    #[test]
    fn test_service_creation() {
        let config = PlatformConfig::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let service = AdminService::new(std::sync::Arc::new(config));
        // PlatformConfig 实现了 Deref，可以直接访问 app_id
        assert_eq!(service.config().app_id(), "test_app_id");
    }
}
