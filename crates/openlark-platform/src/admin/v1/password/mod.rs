//! Password module

use openlark_core::config::Config;

pub mod reset;

/// password 资源服务
#[derive(Debug, Clone)]
pub struct PasswordService {
    config: Config,
}

impl PasswordService {
    /// 创建新的 password 服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 重置密码
    pub fn reset(&self) -> reset::ResetPasswordRequestBuilder {
        reset::ResetPasswordRequestBuilder::new(self.config.clone())
    }
}
