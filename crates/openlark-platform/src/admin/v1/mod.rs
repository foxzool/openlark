//! 管理后台 V1 API
//!
//! 提供管理后台 V1 版本的 API 访问。

use crate::PlatformConfig;
use std::sync::Arc;

/// 部门维度统计接口。
pub mod admin_dept_stat;
/// 用户维度统计接口。
pub mod admin_user_stat;
/// 审计兼容 facade。
pub mod audit;
/// 审计信息查询接口。
pub mod audit_info;
/// 勋章管理接口。
pub mod badge;
/// 勋章图片上传接口。
pub mod badge_image;
/// 密码重置接口。
pub mod password;
/// 用户管理兼容 facade。
pub mod users;

/// 管理后台 V1 API
#[derive(Debug, Clone)]
pub struct AdminV1 {
    config: Arc<PlatformConfig>,
}

impl AdminV1 {
    /// 创建新的管理后台 V1 实例。
    pub fn new(config: Arc<PlatformConfig>) -> Self {
        Self { config }
    }

    /// badge 资源
    pub fn badge(&self) -> badge::BadgeService {
        badge::BadgeService::new(self.config.as_ref().clone())
    }

    /// badge_image 资源
    pub fn badge_image(&self) -> badge_image::BadgeImageService {
        badge_image::BadgeImageService::new(self.config.as_ref().clone())
    }

    /// password 资源
    pub fn password(&self) -> password::PasswordService {
        password::PasswordService::new(self.config.as_ref().clone())
    }

    /// admin_dept_stat 资源
    pub fn admin_dept_stat(&self) -> admin_dept_stat::AdminDeptStatService {
        admin_dept_stat::AdminDeptStatService::new(self.config.as_ref().clone())
    }

    /// admin_user_stat 资源
    pub fn admin_user_stat(&self) -> admin_user_stat::AdminUserStatService {
        admin_user_stat::AdminUserStatService::new(self.config.as_ref().clone())
    }

    /// audit_info 资源
    pub fn audit_info(&self) -> audit_info::AuditInfoService {
        audit_info::AuditInfoService::new(self.config.as_ref().clone())
    }

    /// audit facade（D5：复用已有 AuditApi stub 类型）
    pub fn audit(&self) -> audit::AuditApi {
        audit::AuditApi::new(self.config.clone())
    }

    /// users facade（D5：复用已有 UsersApi stub 类型）
    pub fn users(&self) -> users::UsersApi {
        users::UsersApi::new(self.config.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::AdminV1;
    use crate::PlatformConfig;

    #[test]
    fn test_admin_v1_creation() {
        let config = PlatformConfig::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let api = AdminV1::new(std::sync::Arc::new(config));
        assert_eq!(api.config.app_id(), "test_app_id");
    }

    #[test]
    fn test_admin_v1_chain_access() {
        let config = PlatformConfig::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let api = AdminV1::new(std::sync::Arc::new(config));
        // 6 操作集合叶子 builder 可达
        let _ = api.badge().create();
        let _ = api.badge().list();
        let _ = api.badge_image().create();
        let _ = api.password().reset();
        let _ = api.admin_dept_stat().list();
        let _ = api.admin_user_stat().list();
        let _ = api.audit_info().list();
        // badge.grant 深一级
        let _ = api.badge().grant().list();
        // 2 facade（D5：返回已有 AuditApi/UsersApi stub 类型）
        let _ = api.audit();
        let _ = api.users();
    }
}
