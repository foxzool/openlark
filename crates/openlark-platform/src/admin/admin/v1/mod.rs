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
    // reserved：待装访问器/execute（见 #274，不完整脚手架）
    _config: Arc<PlatformConfig>,
}

impl AdminV1 {
    /// 创建新的管理后台 V1 实例。
    pub fn new(config: Arc<PlatformConfig>) -> Self {
        Self { _config: config }
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
        assert_eq!(api._config.app_id(), "test_app_id");
    }
}
