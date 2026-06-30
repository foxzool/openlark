//! 关联组织列表 API

use openlark_core::config::Config;

pub mod list;

/// collaboration_tenant 资源服务
#[derive(Debug, Clone)]
pub struct CollaborationTenantService {
    config: Config,
}

impl CollaborationTenantService {
    /// 创建新的 collaboration_tenant 服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 关联组织列表
    pub fn list(&self) -> list::CollaborationTenantListRequestBuilder {
        list::CollaborationTenantListRequestBuilder::new(self.config.clone())
    }
}
