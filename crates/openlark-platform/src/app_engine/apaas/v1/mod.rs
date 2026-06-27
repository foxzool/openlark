//! aPaaS V1 API
//!
//! 提供 aPaaS V1 版本的 API 访问

use crate::PlatformConfig;
use std::sync::Arc;

/// 应用列表与基础信息接口。
pub mod app;
/// 应用级对象、函数与环境变量接口。
pub mod application;
/// 审批实例查询与取消接口。
pub mod approval_instance;
/// 审批任务处理接口。
pub mod approval_task;
/// 席位活跃度查询接口。
pub mod seat_activity;
/// 席位分配查询接口。
pub mod seat_assignment;
/// 用户任务处理接口。
pub mod user_task;
/// 工作空间数据与元数据接口。
pub mod workspace;

/// aPaaS V1 API
#[derive(Debug, Clone)]
pub struct ApaasV1 {
    // reserved：待装访问器/execute（见 #274，不完整脚手架）
    _config: Arc<PlatformConfig>,
}

impl ApaasV1 {
    /// 创建新的 aPaaS V1 实例。
    pub fn new(config: Arc<PlatformConfig>) -> Self {
        Self { _config: config }
    }
}

#[cfg(test)]
mod tests {
    use super::ApaasV1;
    use crate::PlatformConfig;

    #[test]
    fn test_apaas_v1_creation() {
        let config = PlatformConfig::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let api = ApaasV1::new(std::sync::Arc::new(config));
        assert_eq!(api._config.app_id(), "test_app_id");
    }
}
