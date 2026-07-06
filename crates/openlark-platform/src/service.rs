//! 平台服务
//!
//! 提供平台管理相关的服务入口

use crate::PlatformConfig;
use std::sync::Arc;

/// 平台服务
///
/// 提供应用引擎、目录服务、系统管理等功能的统一入口。
#[derive(Debug, Clone)]
pub struct PlatformService {
    /// 客户端配置
    config: Arc<PlatformConfig>,
}

impl PlatformService {
    /// 创建新的平台服务实例
    ///
    /// # 参数
    ///
    /// * `config` - 平台服务配置
    pub fn new(config: PlatformConfig) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    /// 获取客户端配置
    pub fn config(&self) -> Arc<PlatformConfig> {
        self.config.clone()
    }

    /// 应用引擎服务
    ///
    /// 提供应用管理、多租户、应用市场等功能。
    #[cfg(feature = "app-engine")]
    pub fn app_engine(&self) -> crate::app_engine::AppEngineService {
        crate::app_engine::AppEngineService::new(self.config.clone())
    }

    /// 目录服务
    ///
    /// 提供用户搜索、组织目录、人员查找等功能。
    #[cfg(feature = "directory")]
    pub fn directory(&self) -> crate::directory::DirectoryService {
        crate::directory::DirectoryService::new(self.config.clone())
    }

    /// 系统管理服务
    ///
    /// 提供系统配置、后台管理、平台工具等功能。
    #[cfg(feature = "admin")]
    pub fn admin(&self) -> crate::admin::AdminService {
        crate::admin::AdminService::new(self.config.clone())
    }

    /// 妙搭平台服务
    ///
    /// 提供妙搭和开放平台用户 ID 转换等功能。
    #[cfg(feature = "spark")]
    pub fn spark(&self) -> crate::spark::SparkService {
        crate::spark::SparkService::new(self.config.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::{PlatformConfig, PlatformService};

    #[test]
    fn test_service_creation() {
        let config = PlatformConfig::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let _service = PlatformService::new(config);
    }
}
