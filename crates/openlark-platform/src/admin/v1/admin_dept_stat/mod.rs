//! Admin department stat module

use openlark_core::config::Config;

pub mod list;

/// admin_dept_stat 资源服务
#[derive(Debug, Clone)]
pub struct AdminDeptStatService {
    config: Config,
}

impl AdminDeptStatService {
    /// 创建新的 admin_dept_stat 服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 部门维度统计列表
    pub fn list(&self) -> list::ListAdminDeptStatRequestBuilder {
        list::ListAdminDeptStatRequestBuilder::new(self.config.clone())
    }
}
