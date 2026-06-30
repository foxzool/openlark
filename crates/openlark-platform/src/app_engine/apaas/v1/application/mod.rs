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
///
/// 深嵌套子级访问器（audit_log/environment_variable/object/role/...）见 Task 4。
#[derive(Debug, Clone)]
pub struct ApplicationService {
    // Task 4 将消费（深嵌套 audit_log/object/role/... 访问器）
    #[allow(dead_code)]
    config: Arc<PlatformConfig>,
    #[allow(dead_code)]
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
}
