//! 用户设置服务
//!
//! 提供用户设置相关的服务入口

use crate::UserConfig;
use openlark_core::SDKResult;
use std::sync::Arc;

/// 用户设置服务
///
/// 提供个人设置（system_status 等）功能的统一入口。
#[derive(Debug, Clone)]
pub struct UserService {
    /// 客户端配置
    config: Arc<UserConfig>,
}

impl UserService {
    /// 创建新的用户设置服务实例
    ///
    /// # 参数
    ///
    /// * `config` - 用户设置服务配置
    ///
    /// # 返回
    ///
    /// 返回用户设置服务实例或错误
    pub fn new(config: UserConfig) -> SDKResult<Self> {
        Ok(Self {
            config: Arc::new(config),
        })
    }

    /// 获取客户端配置。
    pub fn config(&self) -> Arc<UserConfig> {
        self.config.clone()
    }

    /// 个人设置（system_status 等真实 API 入口）。
    ///
    /// 收敛 system_status 资源的真实请求构建器，避免三重嵌套模块全路径
    /// （`personal_settings::personal_settings::v1::system_status::*`）。
    pub fn personal_settings(&self) -> crate::personal_settings::PersonalSettingsService {
        crate::personal_settings::PersonalSettingsService::new(self.config.clone())
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        let config = UserConfig::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let service = UserService::new(config);
        assert!(service.is_ok());
    }
}
