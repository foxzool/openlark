//! 数据分析服务
//!
//! 提供数据分析相关的服务入口

use crate::AnalyticsConfig;
use std::sync::Arc;

/// 数据分析服务
///
/// 数据分析服务的统一入口（search API 经 `crate::search::search::v2::*` 直路径访问，ADR 0001 扁平收口）。
#[derive(Debug, Clone)]
pub struct AnalyticsService {
    /// 客户端配置
    config: Arc<AnalyticsConfig>,
}

impl AnalyticsService {
    /// 创建新的数据分析服务实例。
    ///
    /// 构造不会失败，故返回 `Self`（非 `SDKResult`）——#350 P9 接口形状撒谎修正，
    /// 与 `PlatformService::new` / `UserService::new` 一致。
    pub fn new(config: AnalyticsConfig) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    /// 获取客户端配置
    pub fn config(&self) -> Arc<AnalyticsConfig> {
        self.config.clone()
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        let config = AnalyticsConfig::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let service = AnalyticsService::new(config);
        assert_eq!(service.config().app_id(), "test_app_id");
    }
}
