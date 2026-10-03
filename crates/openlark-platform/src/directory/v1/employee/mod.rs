//! 员工相关 API

use openlark_core::config::Config;

pub mod create;
pub mod delete;
pub mod filter;
pub mod mget;
pub mod patch;
pub mod regular;
pub mod resurrect;
pub mod search;
pub mod to_be_resigned;

/// employee 资源服务
#[derive(Debug, Clone)]
pub struct EmployeeService {
    config: Config,
}

impl EmployeeService {
    /// 创建新的 employee 服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建员工
    pub fn create(
        &self,
        name: impl Into<String>,
        mobile: impl Into<String>,
    ) -> create::EmployeeCreateRequestBuilder {
        create::EmployeeCreateRequestBuilder::new(self.config.clone(), name, mobile)
    }

    /// 删除员工
    pub fn delete(&self, employee_id: impl Into<String>) -> delete::EmployeeDeleteRequestBuilder {
        delete::EmployeeDeleteRequestBuilder::new(self.config.clone(), employee_id)
    }

    /// 员工筛选
    pub fn filter(&self) -> filter::EmployeeFilterRequestBuilder {
        filter::EmployeeFilterRequestBuilder::new(self.config.clone())
    }

    /// 批量获取员工
    pub fn mget(&self) -> mget::EmployeeMgetRequestBuilder {
        mget::EmployeeMgetRequestBuilder::new(self.config.clone())
    }

    /// 更新员工
    pub fn patch(&self, employee_id: impl Into<String>) -> patch::EmployeePatchRequestBuilder {
        patch::EmployeePatchRequestBuilder::new(self.config.clone(), employee_id)
    }

    /// 员工转正
    pub fn regular(
        &self,
        employee_id: impl Into<String>,
    ) -> regular::EmployeeRegularRequestBuilder {
        regular::EmployeeRegularRequestBuilder::new(self.config.clone(), employee_id)
    }

    /// 员工复活
    pub fn resurrect(
        &self,
        employee_id: impl Into<String>,
    ) -> resurrect::EmployeeResurrectRequestBuilder {
        resurrect::EmployeeResurrectRequestBuilder::new(self.config.clone(), employee_id)
    }

    /// 搜索员工
    pub fn search(&self, keyword: impl Into<String>) -> search::EmployeeSearchRequestBuilder {
        search::EmployeeSearchRequestBuilder::new(self.config.clone(), keyword)
    }

    /// 员工待离职
    pub fn to_be_resigned(
        &self,
        employee_id: impl Into<String>,
    ) -> to_be_resigned::EmployeeToBeResignedRequestBuilder {
        to_be_resigned::EmployeeToBeResignedRequestBuilder::new(self.config.clone(), employee_id)
    }
}
