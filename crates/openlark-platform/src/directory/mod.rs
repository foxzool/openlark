//! 目录服务模块
//!
//! 提供用户搜索、组织目录、人员查找等功能 (21 APIs)

use crate::PlatformConfig;
use std::sync::Arc;

/// 目录服务
///
/// 提供目录服务相关 API 的访问入口。
#[derive(Debug, Clone)]
pub struct DirectoryService {
    /// 客户端配置
    config: Arc<PlatformConfig>,
}

impl DirectoryService {
    /// 创建新的目录服务实例
    pub fn new(config: Arc<PlatformConfig>) -> Self {
        Self { config }
    }

    /// 获取客户端配置
    pub fn config(&self) -> Arc<PlatformConfig> {
        self.config.clone()
    }

    /// V1 版本 API
    pub fn v1(&self) -> crate::directory::directory::v1::DirectoryV1 {
        crate::directory::directory::v1::DirectoryV1::new(self.config.clone())
    }
}

pub mod directory;

#[cfg(test)]
mod tests {
    use crate::{PlatformConfig, directory::DirectoryService};

    #[test]
    fn test_service_creation() {
        let config = PlatformConfig::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let service = DirectoryService::new(std::sync::Arc::new(config));
        // PlatformConfig 实现了 Deref，可以直接访问 app_id
        assert_eq!(service.config().app_id(), "test_app_id");
    }
}
