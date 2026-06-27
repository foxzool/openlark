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
    // reserved：待装访问器/execute（见 #274，不完整脚手架）
    _config: Arc<PlatformConfig>,
}

impl DirectoryV1 {
    /// 创建新的目录服务 V1 实例
    pub fn new(config: Arc<PlatformConfig>) -> Self {
        Self { _config: config }
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
        assert_eq!(api._config.app_id(), "test_app_id");
    }
}
