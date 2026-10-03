//! 用户设置服务
//!
//! 提供用户设置相关的服务入口

use crate::UserConfig;
use std::sync::Arc;

/// 用户设置服务
///
/// 提供个人设置（system_status）功能的统一入口（ADR 0001：砍 `PersonalSettingsResource`
/// 单转发中间层，`system_status()` 直达 6 个真实构建器）。
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
    pub fn new(config: UserConfig) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    /// 获取客户端配置。
    pub fn config(&self) -> Arc<UserConfig> {
        self.config.clone()
    }

    /// system_status 资源入口（6 个真实请求构建器）。
    ///
    /// 直达 `SystemStatusResource`（list / create / patch / delete / batch_open /
    /// batch_close），消除原 `service.personal_settings().system_status()` 中
    /// `PersonalSettingsResource` 单转发中间层。
    pub fn system_status(&self) -> crate::personal_settings::SystemStatusResource {
        crate::personal_settings::SystemStatusResource::new(self.config.clone())
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

        let _service = UserService::new(config);
    }
}
