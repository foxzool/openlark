//! 记录权限成员相关 API

pub mod batch_create_authorization;
pub mod batch_remove_authorization;

use openlark_core::config::Config;

/// record_permission.member 资源服务（叶子级）
#[derive(Debug, Clone)]
pub struct RecordPermissionMemberService {
    config: Config,
    namespace: String,
    record_permission_api_name: String,
}

impl RecordPermissionMemberService {
    /// 创建新的 record_permission.member 服务
    pub fn new(
        config: Config,
        namespace: impl Into<String>,
        record_permission_api_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            record_permission_api_name: record_permission_api_name.into(),
        }
    }
    /// 批量新增记录权限成员授权
    pub fn batch_create_authorization(
        &self,
    ) -> batch_create_authorization::RecordPermissionBatchCreateAuthRequestBuilder {
        batch_create_authorization::RecordPermissionBatchCreateAuthRequestBuilder::new(
            self.config.clone(),
            self.namespace.clone(),
            self.record_permission_api_name.clone(),
        )
    }
    /// 批量移除记录权限成员授权
    pub fn batch_remove_authorization(
        &self,
    ) -> batch_remove_authorization::RecordPermissionBatchRemoveAuthRequestBuilder {
        batch_remove_authorization::RecordPermissionBatchRemoveAuthRequestBuilder::new(
            self.config.clone(),
            self.namespace.clone(),
            self.record_permission_api_name.clone(),
        )
    }
}
