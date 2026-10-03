//! Admin user stat module

use openlark_core::config::Config;

pub mod list;

/// admin_user_stat 资源服务
#[derive(Debug, Clone)]
pub struct AdminUserStatService {
    config: Config,
}

impl AdminUserStatService {
    /// 创建新的 admin_user_stat 服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 用户维度统计列表
    pub fn list(&self) -> list::ListAdminUserStatRequestBuilder {
        list::ListAdminUserStatRequestBuilder::new(self.config.clone())
    }
}
