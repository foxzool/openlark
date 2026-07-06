//! 租户应用运营数据模块

pub mod query;

use openlark_core::config::Config;

/// tenant_app_metrics 资源服务
#[derive(Debug, Clone)]
pub struct TenantAppMetricsService {
    config: Config,
}

impl TenantAppMetricsService {
    /// 创建新的 tenant_app_metrics 服务。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询应用运营数据。
    pub fn query(&self) -> query::TenantAppMetricsQueryRequest {
        query::TenantAppMetricsQueryRequest::new(self.config.clone())
    }
}
