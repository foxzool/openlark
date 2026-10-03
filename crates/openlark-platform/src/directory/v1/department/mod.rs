//! 部门相关 API

use openlark_core::config::Config;

pub mod create;
pub mod delete;
pub mod filter;
pub mod mget;
pub mod patch;
pub mod search;

/// department 资源服务
#[derive(Debug, Clone)]
pub struct DepartmentService {
    config: Config,
}

impl DepartmentService {
    /// 创建新的 department 服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建部门
    pub fn create(&self, name: impl Into<String>) -> create::DepartmentCreateRequestBuilder {
        create::DepartmentCreateRequestBuilder::new(self.config.clone(), name)
    }

    /// 删除部门
    pub fn delete(
        &self,
        department_id: impl Into<String>,
    ) -> delete::DepartmentDeleteRequestBuilder {
        delete::DepartmentDeleteRequestBuilder::new(self.config.clone(), department_id)
    }

    /// 部门筛选
    pub fn filter(&self) -> filter::DepartmentFilterRequestBuilder {
        filter::DepartmentFilterRequestBuilder::new(self.config.clone())
    }

    /// 批量获取部门
    pub fn mget(&self) -> mget::DepartmentMgetRequestBuilder {
        mget::DepartmentMgetRequestBuilder::new(self.config.clone())
    }

    /// 更新部门
    pub fn patch(&self, department_id: impl Into<String>) -> patch::DepartmentPatchRequestBuilder {
        patch::DepartmentPatchRequestBuilder::new(self.config.clone(), department_id)
    }

    /// 搜索部门
    pub fn search(&self, keyword: impl Into<String>) -> search::DepartmentSearchRequestBuilder {
        search::DepartmentSearchRequestBuilder::new(self.config.clone(), keyword)
    }
}
