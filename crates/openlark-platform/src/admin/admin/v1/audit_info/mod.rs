//! 审计信息查询模块。

use openlark_core::config::Config;

/// 查询审计信息列表。
pub mod list;

/// audit_info 资源服务
#[derive(Debug, Clone)]
pub struct AuditInfoService {
    config: Config,
}

impl AuditInfoService {
    /// 创建新的 audit_info 服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询审计信息列表
    pub fn list(&self) -> list::ListAuditInfoRequestBuilder {
        list::ListAuditInfoRequestBuilder::new(self.config.clone())
    }
}
