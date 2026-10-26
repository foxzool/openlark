//! 关联组织共享实体查询 API

use openlark_core::config::Config;

pub mod list;

/// collaboration_share_entity 资源服务
#[derive(Debug, Clone)]
pub struct CollaborationShareEntityService {
    config: Config,
}

impl CollaborationShareEntityService {
    /// 创建新的 collaboration_share_entity 服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 共享实体列表
    ///
    /// `target_tenant_key` 为对方组织的 tenant key（必填查询参数）。
    pub fn list(
        &self,
        target_tenant_key: impl Into<String>,
    ) -> list::CollaborationShareEntityListRequestBuilder {
        list::CollaborationShareEntityListRequestBuilder::new(
            self.config.clone(),
            target_tenant_key,
        )
    }
}
