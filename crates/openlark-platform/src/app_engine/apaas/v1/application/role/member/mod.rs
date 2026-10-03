//! 角色成员相关 API

pub mod batch_create_authorization;
pub mod batch_remove_authorization;
pub mod get;

use openlark_core::config::Config;

/// role.member 资源服务（叶子级，持 owned Config + namespace + role_api_name）
#[derive(Debug, Clone)]
pub struct RoleMemberService {
    config: Config,
    namespace: String,
    role_api_name: String,
}

impl RoleMemberService {
    /// 创建新的 role.member 服务
    pub fn new(
        config: Config,
        namespace: impl Into<String>,
        role_api_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            role_api_name: role_api_name.into(),
        }
    }
    /// 查询角色成员
    pub fn get(&self) -> get::RoleMemberGetRequestBuilder {
        get::RoleMemberGetRequestBuilder::new(
            self.config.clone(),
            self.namespace.clone(),
            self.role_api_name.clone(),
        )
    }
    /// 批量新增角色成员授权
    pub fn batch_create_authorization(
        &self,
    ) -> batch_create_authorization::RoleMemberBatchCreateAuthRequestBuilder {
        batch_create_authorization::RoleMemberBatchCreateAuthRequestBuilder::new(
            self.config.clone(),
            self.namespace.clone(),
            self.role_api_name.clone(),
        )
    }
    /// 批量移除角色成员授权
    pub fn batch_remove_authorization(
        &self,
    ) -> batch_remove_authorization::RoleMemberBatchRemoveAuthRequestBuilder {
        batch_remove_authorization::RoleMemberBatchRemoveAuthRequestBuilder::new(
            self.config.clone(),
            self.namespace.clone(),
            self.role_api_name.clone(),
        )
    }
}
