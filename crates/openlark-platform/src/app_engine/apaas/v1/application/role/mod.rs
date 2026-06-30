//! role module

/// 角色成员管理。
pub mod member;

use crate::PlatformConfig;
use std::sync::Arc;

/// application.role 资源服务（中间级，绑 namespace + role_api_name）
#[derive(Debug, Clone)]
pub struct RoleService {
    config: Arc<PlatformConfig>,
    namespace: String,
    role_api_name: String,
}

impl RoleService {
    /// 创建新的 role 服务
    pub fn new(
        config: Arc<PlatformConfig>,
        namespace: impl Into<String>,
        role_api_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            role_api_name: role_api_name.into(),
        }
    }
    /// role.member 子资源（叶子级）
    pub fn member(&self) -> member::RoleMemberService {
        member::RoleMemberService::new(
            self.config.as_ref().clone(),
            self.namespace.clone(),
            self.role_api_name.clone(),
        )
    }
}
