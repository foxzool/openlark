//! 目录服务 V1 API
//!
//! 提供目录服务 V1 版本的 API 访问

use crate::PlatformConfig;
use std::sync::Arc;

/// 可搜可见规则接口。
pub mod collaboration_rule;
/// 关联组织共享实体查询接口。
pub mod collaboration_share_entity;
/// 关联组织列表接口。
pub mod collaboration_tenant;
/// 部门写操作与查询接口。
pub mod department;
/// 部门兼容 facade。
pub mod departments;
/// 员工写操作与查询接口。
pub mod employee;
/// 同步相关接口。
pub mod sync;
/// 用户兼容 facade。
pub mod users;

/// 目录服务 V1 API
#[derive(Debug, Clone)]
pub struct DirectoryV1 {
    config: Arc<PlatformConfig>,
}

impl DirectoryV1 {
    /// 创建新的目录服务 V1 实例
    pub fn new(config: Arc<PlatformConfig>) -> Self {
        Self { config }
    }

    /// department 资源
    pub fn department(&self) -> department::DepartmentService {
        department::DepartmentService::new(self.config.as_ref().clone())
    }

    /// employee 资源
    pub fn employee(&self) -> employee::EmployeeService {
        employee::EmployeeService::new(self.config.as_ref().clone())
    }

    /// collaboration_rule 资源
    pub fn collaboration_rule(&self) -> collaboration_rule::CollaborationRuleService {
        collaboration_rule::CollaborationRuleService::new(self.config.as_ref().clone())
    }

    /// collaboration_share_entity 资源
    pub fn collaboration_share_entity(
        &self,
    ) -> collaboration_share_entity::CollaborationShareEntityService {
        collaboration_share_entity::CollaborationShareEntityService::new(
            self.config.as_ref().clone(),
        )
    }

    /// collaboration_tenant 资源
    pub fn collaboration_tenant(&self) -> collaboration_tenant::CollaborationTenantService {
        collaboration_tenant::CollaborationTenantService::new(self.config.as_ref().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::DirectoryV1;
    use crate::PlatformConfig;

    #[test]
    fn test_directory_v1_creation() {
        let config = PlatformConfig::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let api = DirectoryV1::new(std::sync::Arc::new(config));
        assert_eq!(api.config.app_id(), "test_app_id");
    }

    #[test]
    fn test_directory_v1_chain_access() {
        let config = PlatformConfig::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let api = DirectoryV1::new(std::sync::Arc::new(config));
        // department（create 带 name，patch/delete 带 department_id，search 带 keyword）
        let _ = api.department().create("test_dept".to_string());
        let _ = api.department().filter();
        let _ = api.department().mget();
        let _ = api.department().patch("dept_id".to_string());
        let _ = api.department().delete("dept_id".to_string());
        let _ = api.department().search("kw".to_string());
        // employee（create 带 name+mobile，其余带 employee_id 或 keyword）
        let _ = api.employee().create("n".to_string(), "m".to_string());
        let _ = api.employee().filter();
        let _ = api.employee().mget();
        let _ = api.employee().patch("eid".to_string());
        let _ = api.employee().delete("eid".to_string());
        let _ = api.employee().regular("eid".to_string());
        let _ = api.employee().resurrect("eid".to_string());
        let _ = api.employee().search("kw".to_string());
        let _ = api.employee().to_be_resigned("eid".to_string());
        // collaboration_rule（create 带 name，update/delete 带 rule_id）
        let _ = api.collaboration_rule().create("rule".to_string());
        let _ = api.collaboration_rule().list();
        let _ = api.collaboration_rule().update("rid".to_string());
        let _ = api.collaboration_rule().delete("rid".to_string());
        // 单 list 模块
        let _ = api.collaboration_share_entity().list();
        let _ = api.collaboration_tenant().list();
    }
}
