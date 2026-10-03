//! record_permission module

/// 记录权限成员管理。
pub mod member;

use crate::PlatformConfig;
use std::sync::Arc;

/// application.record_permission 资源服务（中间级，绑 namespace + record_permission_api_name）
#[derive(Debug, Clone)]
pub struct RecordPermissionService {
    config: Arc<PlatformConfig>,
    namespace: String,
    record_permission_api_name: String,
}

impl RecordPermissionService {
    /// 创建新的 record_permission 服务
    pub fn new(
        config: Arc<PlatformConfig>,
        namespace: impl Into<String>,
        record_permission_api_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            record_permission_api_name: record_permission_api_name.into(),
        }
    }
    /// record_permission.member 子资源（叶子级）
    pub fn member(&self) -> member::RecordPermissionMemberService {
        member::RecordPermissionMemberService::new(
            self.config.as_ref().clone(),
            self.namespace.clone(),
            self.record_permission_api_name.clone(),
        )
    }
}
