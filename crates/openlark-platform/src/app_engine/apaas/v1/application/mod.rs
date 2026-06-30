//! 应用引擎模块

pub mod audit_log;
pub mod environment_variable;
pub mod flow;
pub mod function;
pub mod object;
pub mod record_permission;
pub mod role;

use crate::PlatformConfig;
use std::sync::Arc;

/// application 资源服务（中间级，绑定 namespace）
#[derive(Debug, Clone)]
pub struct ApplicationService {
    config: Arc<PlatformConfig>,
    namespace: String,
}

impl ApplicationService {
    /// 创建新的 application 服务
    pub fn new(config: Arc<PlatformConfig>, namespace: impl Into<String>) -> Self {
        Self {
            config,
            namespace: namespace.into(),
        }
    }

    /// application.object 子资源
    pub fn object(&self, object_api_name: impl Into<String>) -> object::ObjectService {
        object::ObjectService::new(self.config.clone(), self.namespace.clone(), object_api_name)
    }

    /// application.role 子资源
    pub fn role(&self, role_api_name: impl Into<String>) -> role::RoleService {
        role::RoleService::new(self.config.clone(), self.namespace.clone(), role_api_name)
    }

    /// application.record_permission 子资源
    pub fn record_permission(
        &self,
        record_permission_api_name: impl Into<String>,
    ) -> record_permission::RecordPermissionService {
        record_permission::RecordPermissionService::new(
            self.config.clone(),
            self.namespace.clone(),
            record_permission_api_name,
        )
    }

    /// application.environment_variable 子资源（叶子级）
    pub fn environment_variable(&self) -> environment_variable::EnvironmentVariableService {
        environment_variable::EnvironmentVariableService::new(
            self.config.as_ref().clone(),
            self.namespace.clone(),
        )
    }

    /// application.function 子资源
    pub fn function(&self, function_api_name: impl Into<String>) -> function::FunctionService {
        function::FunctionService::new(
            self.config.as_ref().clone(),
            self.namespace.clone(),
            function_api_name,
        )
    }

    /// application.flow 子资源
    pub fn flow(&self, flow_id: impl Into<String>) -> flow::FlowService {
        flow::FlowService::new(
            self.config.as_ref().clone(),
            self.namespace.clone(),
            flow_id,
        )
    }

    /// application.audit_log 子资源
    pub fn audit_log(&self) -> audit_log::AuditLogService {
        audit_log::AuditLogService::new(self.config.as_ref().clone(), self.namespace.clone())
    }
}
